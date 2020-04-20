use spacelox_core::chunk::{AlignedByteCode, Chunk, UpvalueIndex};
use spacelox_core::io::{Io, StdIo};
use spacelox_core::managed::{Manage, Managed, Trace};
use spacelox_core::token::{Token, TokenKind};
use spacelox_core::utils::{copy_string, do_if_some};
use spacelox_core::{
  constants::{INIT, SCRIPT, SUPER, THIS},
  value::{ArityKind, Fun, FunKind, Value}, hooks::Hooks,
};
use std::convert::TryInto;
use std::mem;

#[cfg(feature = "debug")]
use crate::debug::disassemble_chunk;

/// The result of a compilation
pub struct CompilerResult {
  /// Was an error encountered while this chunk was compiled
  pub success: bool,

  /// The chunk that was compiled
  pub fun: Managed<Fun>,
}

const UNINITIALIZED: i16 = -1;

#[derive(Debug, Clone)]
pub struct Local {
  /// name of the local
  name: Option<String>,

  /// depth of the local
  depth: i16,

  /// is this local captured
  is_captured: bool,
}

/// The spacelox compiler
pub struct Compiler<'a, I: Io + 'static> {
  /// The current function
  fun: Managed<Fun>,

  /// The type the current function scope
  fun_kind: FunKind,

  /// Analytics for the compiler
  hooks: &'a Hooks<'a>,

  /// The environments standard io access
  io: I,

  /// Number of locals
  local_count: usize,

  /// Current scope depth
  scope_depth: i16,

  /// locals in this function
  locals: Vec<Local>,

  /// upvalues in this function
  upvalues: Vec<UpvalueIndex>,
}

impl<'a, I: Io + Clone> Compiler<'a, I> {
  pub fn new(io: I, hooks: &'a Hooks) -> Self {
    let fun = hooks.manage(
      Fun::new(hooks.manage_str(String::from(SCRIPT)))
    );

    let mut compiler = Self {
      fun,
      fun_kind: FunKind::Script,
      hooks,
      io,
      local_count: 1,
      scope_depth: 0,
      locals: vec![
        Local {
          name: Option::None,
          depth: UNINITIALIZED,
          is_captured: false,
        };
        std::u8::MAX as usize
      ],
      upvalues: vec![UpvalueIndex::Local(0); std::u8::MAX as usize],
    };

    compiler.locals[0] = first_local(FunKind::Script);
    compiler
  }

  pub fn compile(mut self, value: &crate::gene_types::Value) -> CompilerResult {
    CompilerResult {
      success: true,
      fun: self.fun,
    }
  }
}

/// Get the first local for a given function kind
fn first_local(fun_kind: FunKind) -> Local {
  match fun_kind {
    FunKind::Fun => Local {
      name: Option::None,
      depth: 0,
      is_captured: false,
    },
    _ => Local {
      name: Some("this".to_string()),
      depth: 0,
      is_captured: false,
    },
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::gene_parser::Parser;
  use crate::debug::disassemble_chunk;
  use spacelox_core::chunk::decode_u16;
  use spacelox_core::memory::Gc;
  use spacelox_core::{hooks::NoContext, io::{NativeIo, NativeStdIo}};

  enum ByteCodeTest {
    Code(AlignedByteCode),
    Fun((u8, Vec<ByteCodeTest>)),
  }

  fn test_compile<'a>(src: &str, gc: &mut Gc) -> Managed<Fun> {
    let mut parser = Parser::new(src);
    let value = parser.parse().unwrap();

    let mut context = NoContext::new(gc);
    let hooks = &Hooks::new(&mut context);

    let io = NativeIo::new();
    let compiler = Compiler::new(io, &hooks);
    let result = compiler.compile(&value);
    assert_eq!(result.success, true);

    result.fun
  }

  fn decode_byte_code(fun: Managed<Fun>) -> Vec<AlignedByteCode> {
    let bytes = &fun.chunk().instructions;
    let mut decoded = Vec::new();
    let mut offset = 0;

    while offset < bytes.len() {
      let (byte_code, new_offset) = AlignedByteCode::decode(&bytes, offset);

      match byte_code {
        AlignedByteCode::Closure(closure) => {
          decoded.push(byte_code);
          offset = decode_byte_code_closure(fun, &mut decoded, new_offset, closure)
        }
        _ => {
          decoded.push(byte_code);
          offset = new_offset;
        }
      }
    }

    decoded
  }

  fn decode_byte_code_closure(
    fun: Managed<Fun>,
    decoded: &mut Vec<AlignedByteCode>,
    offset: usize,
    slot: u8,
  ) -> usize {
    let inner_fun = fun.chunk().constants[slot as usize].to_fun();
    let mut current_offset = offset;

    let instructions = &fun.chunk().instructions;
    for _ in 0..inner_fun.upvalue_count {
      let scalar = decode_u16(&instructions[offset..offset + 2]);

      let upvalue_index: UpvalueIndex = unsafe { mem::transmute(scalar) };
      decoded.push(AlignedByteCode::UpvalueIndex(upvalue_index));
      current_offset = current_offset + 2;
    }

    current_offset
  }

  fn assert_simple_bytecode(fun: Managed<Fun>, code: &[AlignedByteCode]) {
    disassemble_chunk(&NativeStdIo::new(), &fun.chunk(), "test");
    let decoded_byte_code = decode_byte_code(fun);
    assert_eq!(decoded_byte_code.len(), code.len());

    decoded_byte_code
      .iter()
      .zip(code.iter())
      .for_each(|(actual, expect)| assert_eq!(actual, expect));
  }

  fn assert_fun_bytecode(fun: Managed<Fun>, code: &[ByteCodeTest]) {
    disassemble_chunk(&NativeStdIo::new(), &fun.chunk(), &*fun.name);
    let decoded_byte_code = decode_byte_code(fun);
    assert_eq!(decoded_byte_code.len(), code.len(), "for fun {}", fun.name);

    for i in 0..code.len() {
      match decoded_byte_code[i] {
        AlignedByteCode::Closure(index) => {
          let fun = fun.chunk().constants[index as usize].to_fun();

          match &code[i] {
            ByteCodeTest::Fun((expected, inner)) => {
              assert_eq!(*expected, index);
              assert_fun_bytecode(fun, &inner);
            }
            _ => assert!(false),
          }
        }
        _ => match &code[i] {
          ByteCodeTest::Code(byte_code) => {
            assert_eq!(&decoded_byte_code[i], byte_code);
          }
          _ => assert!(false),
        },
      }
    }
  }

  #[test]
  fn op_print() {
    let example = "(print 10)";

    let mut gc = Gc::new(Box::new(NativeStdIo::new()));
    let fun = test_compile(example, &mut gc);
    assert_simple_bytecode(
      fun,
      &vec![
        AlignedByteCode::Constant(0),
        AlignedByteCode::Print,
        AlignedByteCode::Nil,
        AlignedByteCode::Return,
      ],
    );
  }
}
