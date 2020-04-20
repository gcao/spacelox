use spacelox_core::managed::{Managed};
use spacelox_core::native::{NativeMeta, NativeMethod, NativeResult};
use spacelox_core::{hooks::Hooks, value::{ArityKind, Class, Value}};

pub const NUMBER_CLASS_NAME: &'static str = "Nil";
const NUMBER_STR: NativeMeta = NativeMeta::new("str", ArityKind::Fixed(0));

pub fn create_number_class(hooks: &Hooks) -> Managed<Class> {
  let name = hooks.manage_str(String::from(NUMBER_CLASS_NAME));
  let mut class = hooks.manage(Class::new(name));

  class.add_method(
    hooks,
    hooks.manage_str(String::from(NUMBER_STR.name)),
    Value::NativeMethod(hooks.manage(Box::new(NumberStr::new()))),
  );

  class
}

#[derive(Clone, Debug)]
struct NumberStr {
  meta: Box<NativeMeta>,
}

impl NumberStr {
  fn new() -> Self {
    Self {
      meta: Box::new(NUMBER_STR),
    }
  }
}

impl NativeMethod for NumberStr {
  fn meta(&self) -> &NativeMeta {
    &self.meta
  }

  fn call(&self, hook: &Hooks, this: Value, _args: &[Value]) -> NativeResult {
    NativeResult::Success(Value::String(hook.manage_str(this.to_string())))
  }
}
