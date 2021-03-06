use spacelox_core::native::{NativeFun, NativeMeta, NativeResult};
use spacelox_core::{hooks::Hooks, value::{ArityKind, Value}};

const NATIVE_ASSERT_META: NativeMeta = NativeMeta::new("assert", ArityKind::Fixed(1));

#[derive(Clone, Debug)]
/// A native method to assert that for a boolean true value
pub struct NativeAssert {
  /// the assert meta data
  meta: Box<NativeMeta>,
}

impl NativeAssert {
  /// Construct a new instance of the native assert function
  ///
  /// ```
  /// use spacelox_lib::assert::assert::NativeAssert;
  ///
  /// let native_assert = NativeAssert::new();
  /// ```
  pub fn new() -> Self {
    Self {
      meta: Box::new(NATIVE_ASSERT_META),
    }
  }
}

impl NativeFun for NativeAssert {
  fn meta(&self) -> &NativeMeta {
    &self.meta
  }

  fn call(&self, _hooks: &Hooks, args: &[Value]) -> NativeResult {
    match args[0] {
      Value::Bool(b) => {
        if b {
          return NativeResult::Success(Value::Nil);
        }
        NativeResult::RuntimeError(format!("'assert' expected true received false."))
      }
      _ => NativeResult::RuntimeError(format!("'assert' expected a boolean value.")),
    }
  }
}

const NATIVE_ASSERTEQ_META: NativeMeta = NativeMeta::new("assertEq", ArityKind::Fixed(2));

#[derive(Clone, Debug)]
pub struct NativeAssertEq {
  meta: Box<NativeMeta>,
}

impl NativeAssertEq {
  /// Construct a new instance of the native assertEq function
  ///
  /// ```
  /// use spacelox_lib::assert::assert::NativeAssertEq;
  ///
  /// let native_assert = NativeAssertEq::new();
  /// ```
  pub fn new() -> Self {
    Self {
      meta: Box::new(NATIVE_ASSERTEQ_META),
    }
  }
}

impl NativeFun for NativeAssertEq {
  fn meta(&self) -> &NativeMeta {
    &self.meta
  }

  fn call(&self, _hooks: &Hooks, args: &[Value]) -> NativeResult {
    if args[0] == args[1] {
      return NativeResult::Success(Value::Nil);
    }

    NativeResult::RuntimeError(format!("{:?} and {:?} where not equal", args[0], args[1]))
  }
}

const NATIVE_ASSERTNE_META: NativeMeta = NativeMeta::new("assertNe", ArityKind::Fixed(2));

#[derive(Clone, Debug)]
pub struct NativeAssertNe {
  meta: Box<NativeMeta>,
}

impl NativeAssertNe {
  /// Construct a new instance of the native assertNe function
  ///
  /// ```
  /// use spacelox_lib::assert::assert::NativeAssertNe;
  ///
  /// let native_assert = NativeAssertNe::new();
  /// ```
  pub fn new() -> Self {
    Self {
      meta: Box::new(NATIVE_ASSERTNE_META),
    }
  }
}

impl NativeFun for NativeAssertNe {
  fn meta(&self) -> &NativeMeta {
    &self.meta
  }

  fn call(&self, _hooks: &Hooks, args: &[Value]) -> NativeResult {
    if args[0] != args[1] {
      return NativeResult::Success(Value::Nil);
    }

    NativeResult::RuntimeError(format!("{:?} and {:?} where equal", args[0], args[1]))
  }
}

mod test {
  #[cfg(test)]
  use super::*;
  #[cfg(test)]
  use crate::support::test_native_dependencies;

  #[cfg(test)]
  mod assert {
    use super::*;
    use crate::support::{TestContext, test_native_dependencies};

    #[test]
    fn new() {
      let assert = NativeAssert::new();

      assert_eq!(assert.meta.name, "assert");
      assert_eq!(assert.meta.arity, ArityKind::Fixed(1));
    }

    #[test]
    fn call() {
      let assert = NativeAssert::new();
      let gc = test_native_dependencies();
      let mut context = TestContext::new(&gc);
      let hooks = Hooks::new(&mut context);

      let values = &[Value::Bool(true)];

      let result = match assert.call(&hooks, values) {
        NativeResult::Success(res) => res,
        NativeResult::RuntimeError(_) => panic!(),
      };

      assert_eq!(result, Value::Nil);
    }
  }

  #[cfg(test)]
  mod assert_eq {
    use super::*;
    use crate::support::TestContext;

    #[test]
    fn new() {
      let assert_eq = NativeAssertEq::new();

      assert_eq!(assert_eq.meta.name, "assertEq");
      assert_eq!(assert_eq.meta.arity, ArityKind::Fixed(2));
    }

    #[test]
    fn call() {
      let assert_eq = NativeAssertEq::new();
      let gc = test_native_dependencies();
      let mut context = TestContext::new(&gc);
      let hooks = Hooks::new(&mut context);

      let values = &[Value::Number(10.5), Value::Number(10.5)];

      let result = match assert_eq.call(&hooks, values) {
        NativeResult::Success(res) => res,
        NativeResult::RuntimeError(_) => panic!(),
      };

      assert_eq!(result, Value::Nil);
    }
  }

  #[cfg(test)]
  mod assert_ne {
    use super::*;
    use crate::support::TestContext;

    #[test]
    fn new() {
      let assert_eq = NativeAssertNe::new();

      assert_eq!(assert_eq.meta.name, "assertNe");
      assert_eq!(assert_eq.meta.arity, ArityKind::Fixed(2));
    }

    #[test]
    fn call() {
      let assert_eq = NativeAssertNe::new();
      let gc = test_native_dependencies();
      let mut context = TestContext::new(&gc);
      let hooks = Hooks::new(&mut context);

      let values = &[Value::Number(10.5), Value::Nil];

      let result = match assert_eq.call(&hooks, values) {
        NativeResult::Success(res) => res,
        NativeResult::RuntimeError(_) => panic!(),
      };

      assert_eq!(result, Value::Nil);
    }
  }
}
