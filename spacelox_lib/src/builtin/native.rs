use spacelox_core::hooks::Hooks;
use spacelox_core::managed::Managed;
use spacelox_core::native::{NativeMeta, NativeMethod, NativeResult};
use spacelox_core::value::{ArityKind, Class, Value};

pub const NATIVE_CLASS_NAME: &'static str = "Native";

const NATIVE_NAME: NativeMeta = NativeMeta::new("name", ArityKind::Fixed(0));

pub fn create_native_class(hooks: &Hooks) -> Managed<Class> {
  let name = hooks.manage_str(String::from(NATIVE_CLASS_NAME));
  let mut class = hooks.manage(Class::new(name));

  class.add_method(
    hooks,
    hooks.manage_str(String::from(NATIVE_NAME.name)),
    Value::NativeMethod(hooks.manage(Box::new(NativeName::new()))),
  );

  class
}

#[derive(Clone, Debug)]
struct NativeName {
  meta: Box<NativeMeta>,
}

impl NativeName {
  fn new() -> Self {
    Self {
      meta: Box::new(NATIVE_NAME),
    }
  }
}

impl NativeMethod for NativeName {
  fn meta(&self) -> &NativeMeta {
    &self.meta
  }

  fn call(&self, hooks: &Hooks, this: Value, _args: &[Value]) -> NativeResult {
    NativeResult::Success(Value::String(
      hooks.manage_str(String::from(this.to_native_fun().meta().name)),
    ))
  }
}
