use spacelox_core::managed::{Managed};
use spacelox_core::native::{NativeMeta, NativeMethod, NativeResult};
use spacelox_core::{hooks::Hooks, value::{ArityKind, Class, Value}};

pub const NIL_CLASS_NAME: &'static str = "Nil";
const NIL_STR: NativeMeta = NativeMeta::new("str", ArityKind::Fixed(0));

pub fn create_nil_class(hooks: &Hooks) -> Managed<Class> {
  let name = hooks.manage_str(String::from(NIL_CLASS_NAME));
  let mut class = hooks.manage(Class::new(name));

  class.add_method(
    hooks,
    hooks.manage_str(String::from(NIL_STR.name)),
    Value::NativeMethod(hooks.manage(Box::new(NilStr::new()))),
  );

  class
}

#[derive(Clone, Debug)]
struct NilStr {
  meta: Box<NativeMeta>,
}

impl NilStr {
  fn new() -> Self {
    Self {
      meta: Box::new(NIL_STR),
    }
  }
}

impl NativeMethod for NilStr {
  fn meta(&self) -> &NativeMeta {
    &self.meta
  }

  fn call(&self, hooks: &Hooks, this: Value, _args: &[Value]) -> NativeResult {
    NativeResult::Success(Value::String(hooks.manage_str(this.to_string())))
  }
}
