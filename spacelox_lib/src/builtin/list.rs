use spacelox_core::managed::Managed;
use spacelox_core::native::{NativeMeta, NativeMethod, NativeResult};
use spacelox_core::value::{ArityKind, Class, Value};
use spacelox_core::hooks::Hooks;


pub const LIST_CLASS_NAME: &'static str = "List";

const LIST_STR: NativeMeta = NativeMeta::new("str", ArityKind::Fixed(0));
const LIST_SIZE: NativeMeta = NativeMeta::new("size", ArityKind::Fixed(0));
const LIST_PUSH: NativeMeta = NativeMeta::new("push", ArityKind::Variadic(1));
const LIST_POP: NativeMeta = NativeMeta::new("pop", ArityKind::Fixed(0));
const LIST_REMOVE: NativeMeta = NativeMeta::new("remove", ArityKind::Fixed(1));
const LIST_INSERT: NativeMeta = NativeMeta::new("insert", ArityKind::Fixed(2));
const LIST_CLEAR: NativeMeta = NativeMeta::new("clear", ArityKind::Fixed(0));
const LIST_HAS: NativeMeta = NativeMeta::new("has", ArityKind::Fixed(1));

pub fn create_list_class(hooks: &Hooks) -> Managed<Class> {
  let name = hooks.manage_str(String::from(LIST_CLASS_NAME));
  let mut class = hooks.manage(Class::new(name));

  class.add_method(
    hooks,
    hooks.manage_str(String::from(LIST_SIZE.name)),
    Value::NativeMethod(hooks.manage(Box::new(ListSize::new()))),
  );

  class.add_method(
    hooks,
    hooks.manage_str(String::from(LIST_PUSH.name)),
    Value::NativeMethod(hooks.manage(Box::new(ListPush::new()))),
  );

  class.add_method(
    hooks,
    hooks.manage_str(String::from(LIST_POP.name)),
    Value::NativeMethod(hooks.manage(Box::new(ListPop::new()))),
  );

  class.add_method(
    hooks,
    hooks.manage_str(String::from(LIST_REMOVE.name)),
    Value::NativeMethod(hooks.manage(Box::new(ListRemove::new()))),
  );

  class.add_method(
    hooks,
    hooks.manage_str(String::from(LIST_INSERT.name)),
    Value::NativeMethod(hooks.manage(Box::new(ListInsert::new()))),
  );

  class.add_method(
    hooks,
    hooks.manage_str(String::from(LIST_STR.name)),
    Value::NativeMethod(hooks.manage(Box::new(ListStr::new()))),
  );

  class.add_method(
    hooks,
    hooks.manage_str(String::from(LIST_CLEAR.name)),
    Value::NativeMethod(hooks.manage(Box::new(ListClear::new()))),
  );

  class.add_method(
    hooks,
    hooks.manage_str(String::from(LIST_HAS.name)),
    Value::NativeMethod(hooks.manage(Box::new(ListHas::new()))),
  );

  class
}

#[derive(Clone, Debug)]
struct ListStr {
  meta: Box<NativeMeta>,
}

impl ListStr {
  fn new() -> Self {
    Self {
      meta: Box::new(LIST_STR),
    }
  }
}

impl NativeMethod for ListStr {
  fn meta(&self) -> &NativeMeta {
    &self.meta
  }

  fn call(&self, hooks: &Hooks, this: Value, _args: &[Value]) -> NativeResult {
    NativeResult::Success(Value::String(hooks.manage_str(this.to_string())))
  }
}

#[derive(Clone, Debug)]
struct ListSize {
  meta: Box<NativeMeta>,
}

impl ListSize {
  fn new() -> Self {
    Self {
      meta: Box::new(LIST_SIZE),
    }
  }
}

impl NativeMethod for ListSize {
  fn meta(&self) -> &NativeMeta {
    &self.meta
  }

  fn call(&self, _hooks: &Hooks, this: Value, _args: &[Value]) -> NativeResult {
    NativeResult::Success(Value::Number(this.to_list().len() as f64))
  }
}

#[derive(Clone, Debug)]
struct ListPush {
  meta: Box<NativeMeta>,
}

impl ListPush {
  fn new() -> Self {
    Self {
      meta: Box::new(LIST_PUSH),
    }
  }
}

impl NativeMethod for ListPush {
  fn meta(&self) -> &NativeMeta {
    &self.meta
  }

  fn call(&self, _hooks: &Hooks, this: Value, args: &[Value]) -> NativeResult {
    this.to_list().extend(args);
    NativeResult::Success(Value::Nil)
  }
}

#[derive(Clone, Debug)]
struct ListPop {
  meta: Box<NativeMeta>,
}

impl ListPop {
  fn new() -> Self {
    Self {
      meta: Box::new(LIST_POP),
    }
  }
}

impl NativeMethod for ListPop {
  fn meta(&self) -> &NativeMeta {
    &self.meta
  }

  fn call(&self, _hooks: &Hooks, this: Value, _args: &[Value]) -> NativeResult {
    match this.to_list().pop() {
      Some(value) => NativeResult::Success(value),
      None => NativeResult::Success(Value::Nil),
    }
  }
}

#[derive(Clone, Debug)]
struct ListRemove {
  meta: Box<NativeMeta>,
}

impl ListRemove {
  fn new() -> Self {
    Self {
      meta: Box::new(LIST_REMOVE),
    }
  }
}

impl NativeMethod for ListRemove {
  fn meta(&self) -> &NativeMeta {
    &self.meta
  }

  fn call(&self, _hooks: &Hooks, this: Value, args: &[Value]) -> NativeResult {
    let index = args[0].to_num();
    let mut list = this.to_list();

    if index < 0.0 {
      return NativeResult::RuntimeError(format!("Cannot remove at index {}", index));
    }

    if index as usize >= list.len() {
      return NativeResult::RuntimeError(format!(
        "Cannot remove at index {}, list has size {}",
        index,
        list.len()
      ));
    }

    let result = list.remove(index as usize);
    NativeResult::Success(result)
  }
}

#[derive(Clone, Debug)]
struct ListInsert {
  meta: Box<NativeMeta>,
}

impl ListInsert {
  fn new() -> Self {
    Self {
      meta: Box::new(LIST_INSERT),
    }
  }
}

impl NativeMethod for ListInsert {
  fn meta(&self) -> &NativeMeta {
    &self.meta
  }

  fn call(&self, _hooks: &Hooks, this: Value, args: &[Value]) -> NativeResult {
    let index = args[0].to_num();
    let mut list = this.to_list();

    if index < 0.0 {
      return NativeResult::RuntimeError(format!("Cannot insert at index {}", index));
    }

    if index as usize > list.len() {
      return NativeResult::RuntimeError(format!(
        "Cannot insert at index {}, list has size {}",
        index,
        list.len()
      ));
    }

    list.insert(index as usize, args[1]);
    NativeResult::Success(Value::Nil)
  }
}

#[derive(Clone, Debug)]
struct ListClear {
  meta: Box<NativeMeta>,
}

impl ListClear {
  fn new() -> Self {
    Self {
      meta: Box::new(LIST_CLEAR),
    }
  }
}

impl NativeMethod for ListClear {
  fn meta(&self) -> &NativeMeta {
    &self.meta
  }

  fn call(&self, _hooks: &Hooks, this: Value, _args: &[Value]) -> NativeResult {
    this.to_list().clear();
    NativeResult::Success(Value::Nil)
  }
}

#[derive(Clone, Debug)]
struct ListHas {
  meta: Box<NativeMeta>,
}

impl ListHas {
  fn new() -> Self {
    Self {
      meta: Box::new(LIST_HAS),
    }
  }
}

impl NativeMethod for ListHas {
  fn meta(&self) -> &NativeMeta {
    &self.meta
  }

  fn call(&self, _hooks: &Hooks, this: Value, args: &[Value]) -> NativeResult {
    NativeResult::Success(Value::Bool(this.to_list().contains(&args[0])))
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[cfg(test)]
  mod str {
    use super::*;
    use crate::support::{TestContext, test_native_dependencies};

    #[test]
    fn new() {
      let list_str = ListStr::new();

      assert_eq!(list_str.meta.name, "str");
      assert_eq!(list_str.meta.arity, ArityKind::Fixed(0));
    }

    #[test]
    fn call() {
      let list_str = ListStr::new();
      let gc = test_native_dependencies();
      let mut context = TestContext::new(&gc);
      let hooks = Hooks::new(&mut context);

      let values = &[];

      let list = vec![Value::Nil, Value::Number(10.0)];
      let this = hooks.manage(list);

      let result = list_str.call(&hooks, Value::List(this), values);
      match result {
        NativeResult::Success(r) => assert_eq!(&*r.to_str(), "[nil, 10]"),
        NativeResult::RuntimeError(_) => assert!(false),
      }
    }
  }

  #[cfg(test)]
  mod size {
    use super::*;
    use crate::support::{TestContext, test_native_dependencies};
    use spacelox_core::hooks::Hooks;

    #[test]
    fn new() {
      let list_size = ListSize::new();

      assert_eq!(list_size.meta.name, "size");
      assert_eq!(list_size.meta.arity, ArityKind::Fixed(0));
    }

    #[test]
    fn call() {
      let list_size = ListSize::new();
      let gc = test_native_dependencies();
      let mut context = TestContext::new(&gc);
      let hooks = Hooks::new(&mut context);

      let values = &[];

      let list = vec![Value::Nil, Value::Number(10.0)];
      let this = hooks.manage(list);

      let result = list_size.call(&hooks, Value::List(this), values);
      match result {
        NativeResult::Success(r) => assert_eq!(r.to_num(), 2.0),
        NativeResult::RuntimeError(_) => assert!(false),
      }
    }
  }

  #[cfg(test)]
  mod push {
    use super::*;
    use crate::support::{TestContext, test_native_dependencies};

    #[test]
    fn new() {
      let list_push = ListPush::new();

      assert_eq!(list_push.meta.name, "push");
      assert_eq!(list_push.meta.arity, ArityKind::Variadic(1));
    }

    #[test]
    fn call() {
      let list_push = ListPush::new();
      let gc = test_native_dependencies();
      let mut context = TestContext::new(&gc);
      let hooks = Hooks::new(&mut context);

      let list = vec![Value::Nil, Value::Number(10.0)];
      let this = hooks.manage(list);
      let list_value = Value::List(this);

      let result = list_push.call(&hooks, list_value, &[Value::Bool(false)]);
      match result {
        NativeResult::Success(r) => {
          assert_eq!(r, Value::Nil);
          assert_eq!(list_value.to_list().len(), 3);
          assert_eq!(list_value.to_list()[2], Value::Bool(false));
        }
        NativeResult::RuntimeError(_) => assert!(false),
      }

      let result = list_push.call(
        &hooks,
        Value::List(this),
        &[Value::Number(10.3), Value::Nil],
      );
      match result {
        NativeResult::Success(r) => {
          assert_eq!(r, Value::Nil);
          assert_eq!(list_value.to_list().len(), 5);
          assert_eq!(list_value.to_list()[3], Value::Number(10.3));
          assert_eq!(list_value.to_list()[4], Value::Nil);
        }
        NativeResult::RuntimeError(_) => assert!(false),
      }
    }
  }

  #[cfg(test)]
  mod pop {
    use super::*;
    use crate::support::{TestContext, test_native_dependencies};

    #[test]
    fn new() {
      let list_pop = ListPop::new();

      assert_eq!(list_pop.meta.name, "pop");
      assert_eq!(list_pop.meta.arity, ArityKind::Fixed(0));
    }

    #[test]
    fn call() {
      let list_pop = ListPop::new();
      let gc = test_native_dependencies();
      let mut context = TestContext::new(&gc);
      let hooks = Hooks::new(&mut context);

      let list = vec![Value::Bool(true)];
      let this = hooks.manage(list);
      let list_value = Value::List(this);

      let result = list_pop.call(&hooks, list_value, &[]);
      match result {
        NativeResult::Success(r) => {
          assert_eq!(r.to_bool(), true);
          assert_eq!(this.len(), 0);
        }
        NativeResult::RuntimeError(_) => assert!(false),
      }

      let result = list_pop.call(&hooks, list_value, &[]);
      match result {
        NativeResult::Success(r) => {
          assert!(r.is_nil());
          assert_eq!(this.len(), 0);
        }
        NativeResult::RuntimeError(_) => assert!(false),
      }
    }
  }

  #[cfg(test)]
  mod remove {
    use super::*;
    use crate::support::{TestContext, test_native_dependencies};

    #[test]
    fn new() {
      let list_remove = ListRemove::new();

      assert_eq!(list_remove.meta.name, "remove");
      assert_eq!(list_remove.meta.arity, ArityKind::Fixed(1));
    }

    #[test]
    fn call() {
      let list_remove = ListRemove::new();
      let gc = test_native_dependencies();
      let mut context = TestContext::new(&gc);
      let hooks = Hooks::new(&mut context);

      let list = vec![Value::Nil, Value::Number(10.0), Value::Bool(true)];
      let this = hooks.manage(list);
      let list_value = Value::List(this);

      let result = list_remove.call(&hooks, list_value, &[Value::Number(1.0)]);
      match result {
        NativeResult::Success(r) => {
          assert_eq!(r.to_num(), 10.0);
          assert_eq!(this.len(), 2);
        }
        NativeResult::RuntimeError(_) => assert!(false),
      }

      let result = list_remove.call(&hooks, list_value, &[Value::Number(-1.0)]);
      match result {
        NativeResult::Success(_) => assert!(false),
        NativeResult::RuntimeError(_) => assert!(true),
      }

      let result = list_remove.call(&hooks, list_value, &[Value::Number(10.0)]);
      match result {
        NativeResult::Success(_) => assert!(false),
        NativeResult::RuntimeError(_) => assert!(true),
      }
    }
  }

  #[cfg(test)]
  mod insert {
    use super::*;
    use crate::support::{TestContext, test_native_dependencies};

    #[test]
    fn new() {
      let list_insert = ListInsert::new();

      assert_eq!(list_insert.meta.name, "insert");
      assert_eq!(list_insert.meta.arity, ArityKind::Fixed(2));
    }

    #[test]
    fn call() {
      let list_insert = ListInsert::new();
      let gc = test_native_dependencies();
      let mut context = TestContext::new(&gc);
      let hooks = Hooks::new(&mut context);

      let list = vec![Value::Nil, Value::Number(10.0), Value::Bool(true)];
      let this = hooks.manage(list);
      let list_value = Value::List(this);

      let result = list_insert.call(
        &hooks,
        list_value,
        &[Value::Number(1.0), Value::Bool(false)],
      );
      match result {
        NativeResult::Success(r) => {
          assert!(r.is_nil());
          assert_eq!(this[1], Value::Bool(false));
          assert_eq!(this.len(), 4);
        }
        NativeResult::RuntimeError(_) => assert!(false),
      }

      let result = list_insert.call(&hooks, list_value, &[Value::Number(-1.0)]);
      match result {
        NativeResult::Success(_) => assert!(false),
        NativeResult::RuntimeError(_) => assert!(true),
      }

      let result = list_insert.call(&hooks, list_value, &[Value::Number(10.0)]);
      match result {
        NativeResult::Success(_) => assert!(false),
        NativeResult::RuntimeError(_) => assert!(true),
      }
    }
  }

  #[cfg(test)]
  mod clear {
    use super::*;
    use crate::support::{TestContext, test_native_dependencies};

    #[test]
    fn new() {
      let list_clear = ListClear::new();

      assert_eq!(list_clear.meta.name, "clear");
      assert_eq!(list_clear.meta.arity, ArityKind::Fixed(0));
    }

    #[test]
    fn call() {
      let list_clear = ListClear::new();
      let gc = test_native_dependencies();
      let mut context = TestContext::new(&gc);
      let hooks = Hooks::new(&mut context);

      let list = vec![Value::Nil, Value::Number(10.0), Value::Bool(true)];
      let this = hooks.manage(list);
      let list_value = Value::List(this);

      let result = list_clear.call(&hooks, list_value, &[]);
      match result {
        NativeResult::Success(r) => {
          assert!(r.is_nil());
          assert_eq!(this.len(), 0);
        }
        NativeResult::RuntimeError(_) => assert!(false),
      }

      let result = list_clear.call(&hooks, list_value, &[]);
      match result {
        NativeResult::Success(r) => {
          assert!(r.is_nil());
          assert_eq!(this.len(), 0);
        }
        NativeResult::RuntimeError(_) => assert!(false),
      }
    }
  }
}
