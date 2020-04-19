use spacelox_core::hooks::Hooks;
use spacelox_core::managed::Managed;
use spacelox_core::native::{NativeMeta, NativeMethod, NativeResult};
use spacelox_core::value::{ArityKind, Class, Value};

pub const STRING_CLASS_NAME: &'static str = "String";
const STRING_STR: NativeMeta = NativeMeta::new("str", ArityKind::Fixed(0));

pub fn create_string_class(hooks: &Hooks) -> Managed<Class> {
  let name = hooks.manage_str(String::from(STRING_CLASS_NAME));
  let mut class = hooks.manage(Class::new(name));

  class.add_method(
    hooks,
    hooks.manage_str(String::from(STRING_STR.name)),
    Value::NativeMethod(hooks.manage(Box::new(StringStr::new()))),
  );

  class
}

#[derive(Clone, Debug)]
struct StringStr {
  meta: Box<NativeMeta>,
}

impl StringStr {
  fn new() -> Self {
    Self {
      meta: Box::new(STRING_STR),
    }
  }
}

impl NativeMethod for StringStr {
  fn meta(&self) -> &NativeMeta {
    &self.meta
  }

  fn call(&self, _hooks: &Hooks, this: Value, _args: &[Value]) -> NativeResult {
    NativeResult::Success(Value::String(this.to_str()))
  }
}
