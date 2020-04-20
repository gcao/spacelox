use spacelox_core::managed::{Managed};
use spacelox_core::native::{NativeMeta, NativeMethod, NativeResult};
use spacelox_core::{hooks::Hooks, value::{ArityKind, Class, Value}};

pub const FUN_CLASS_NAME: &'static str = "Fun";

const FUN_NAME: NativeMeta = NativeMeta::new("name", ArityKind::Fixed(0));

pub fn create_fun_class(hooks: &Hooks) -> Managed<Class> {
  let name = hooks.manage_str(String::from(FUN_CLASS_NAME));
  let mut class = hooks.manage(Class::new(name));

  class.add_method(
    hooks,
    hooks.manage_str(String::from(FUN_NAME.name)),
    Value::NativeMethod(hooks.manage(Box::new(FunName::new()))),
  );

  class
}

#[derive(Clone, Debug)]
struct FunName {
  meta: Box<NativeMeta>,
}

impl FunName {
  fn new() -> Self {
    Self {
      meta: Box::new(FUN_NAME),
    }
  }
}

impl NativeMethod for FunName {
  fn meta(&self) -> &NativeMeta {
    &self.meta
  }

  fn call(&self, _hooks: &Hooks,  this: Value, _args: &[Value]) -> NativeResult {
    NativeResult::Success(Value::String(this.to_fun().name))
  }
}
