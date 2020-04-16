use spacelox_core::managed::{Managed, Trace};
use spacelox_core::memory::Gc;
use spacelox_core::native::{NativeMeta, NativeMethod, NativeResult};
use spacelox_core::value::{ArityKind, Class, Value};

pub const MAP_CLASS_NAME: &'static str = "Map";

const MAP_STR: NativeMeta = NativeMeta::new("str", ArityKind::Fixed(0));
const MAP_SIZE: NativeMeta = NativeMeta::new("size", ArityKind::Fixed(0));
const MAP_HAS: NativeMeta = NativeMeta::new("has", ArityKind::Fixed(1));
const MAP_GET: NativeMeta = NativeMeta::new("get", ArityKind::Fixed(1));

pub fn create_map_class<C: Trace>(gc: &Gc, context: &C) -> Managed<Class> {
  let name = gc.manage_str(String::from(MAP_CLASS_NAME), context);
  let mut class = gc.manage(Class::new(name), context);

  class.methods.insert(
    gc.manage_str(String::from(MAP_SIZE.name), context),
    Value::NativeMethod(gc.manage(Box::new(MapSize::new()), context)),
  );

  class.methods.insert(
    gc.manage_str(String::from(MAP_STR.name), context),
    Value::NativeMethod(gc.manage(Box::new(MapStr::new()), context)),
  );

  class.methods.insert(
    gc.manage_str(String::from(MAP_HAS.name), context),
    Value::NativeMethod(gc.manage(Box::new(MapHas::new()), context)),
  );

  class.methods.insert(
    gc.manage_str(String::from(MAP_GET.name), context),
    Value::NativeMethod(gc.manage(Box::new(MapGet::new()), context)),
  );

  class
}

#[derive(Clone, Debug)]
struct MapStr {
  meta: Box<NativeMeta>,
}

impl MapStr {
  fn new() -> Self {
    Self {
      meta: Box::new(MAP_STR),
    }
  }
}

impl NativeMethod for MapStr {
  fn meta(&self) -> &NativeMeta {
    &self.meta
  }

  fn call(&self, gc: &Gc, context: &dyn Trace, this: Value, _args: &[Value]) -> NativeResult {
    NativeResult::Success(Value::String(gc.manage_str(this.to_string(), context)))
  }
}

#[derive(Clone, Debug)]
struct MapSize {
  meta: Box<NativeMeta>,
}

impl MapSize {
  fn new() -> Self {
    Self {
      meta: Box::new(MAP_SIZE),
    }
  }
}

impl NativeMethod for MapSize {
  fn meta(&self) -> &NativeMeta {
    &self.meta
  }

  fn call(&self, _gc: &Gc, _context: &dyn Trace, this: Value, _args: &[Value]) -> NativeResult {
    NativeResult::Success(Value::Number(this.to_map().len() as f64))
  }
}


#[derive(Clone, Debug)]
struct MapHas {
  meta: Box<NativeMeta>,
}

impl MapHas {
  fn new() -> Self {
    Self {
      meta: Box::new(MAP_HAS),
    }
  }
}

impl NativeMethod for MapHas {
  fn meta(&self) -> &NativeMeta {
    &self.meta
  }

  fn call(&self, _gc: &Gc, _context: &dyn Trace, this: Value, args: &[Value]) -> NativeResult {
    NativeResult::Success(Value::Bool(this.to_map().contains_key(&args[0])))
  }
}


#[derive(Clone, Debug)]
struct MapGet {
  meta: Box<NativeMeta>,
}

impl MapGet {
  fn new() -> Self {
    Self {
      meta: Box::new(MAP_GET),
    }
  }
}

impl NativeMethod for MapGet {
  fn meta(&self) -> &NativeMeta {
    &self.meta
  }

  fn call(&self, _gc: &Gc, _context: &dyn Trace, this: Value, args: &[Value]) -> NativeResult {
    match this.to_map().get(&args[0]) {
      Some(value) => NativeResult::Success(*value),
      None => NativeResult::Success(Value::Nil),
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[cfg(test)]
  mod str {
    use super::*;
    use fnv::FnvHashMap;
    use crate::support::test_native_dependencies;

    #[test]
    fn new() {
      let map_str = MapStr::new();
  
      assert_eq!(map_str.meta.name, "str");
      assert_eq!(map_str.meta.arity, ArityKind::Fixed(0));
    }
  
    #[test]
    fn call() {
      let map_str = MapStr::new();
      let (gc, context) = test_native_dependencies();
      let values = &[];

      let mut map = FnvHashMap::default();
      map.insert(Value::Nil, Value::Nil);
      let this = gc.manage(map, &*context);
  
      let result = map_str.call(&gc, &*context, Value::Map(this), values);
      match result {
        NativeResult::Success(r) => {
          assert_eq!(&*r.to_str(), "{ nil: nil }")
        },
        NativeResult::RuntimeError(_) => assert!(false)
      }
    }
  }

  #[cfg(test)]
  mod size {
    use super::*;
    use fnv::FnvHashMap;
    use crate::support::test_native_dependencies;

    #[test]
    fn new() {
      let map_str = MapSize::new();
  
      assert_eq!(map_str.meta.name, "size");
      assert_eq!(map_str.meta.arity, ArityKind::Fixed(0));
    }
  
    #[test]
    fn call() {
      let map_str = MapSize::new();
      let (gc, context) = test_native_dependencies();
      let values = &[];

      let mut map = FnvHashMap::default();
      map.insert(Value::Nil, Value::Nil);
      let this = gc.manage(map, &*context);
  
      let result = map_str.call(&gc, &*context, Value::Map(this), values);
      match result {
        NativeResult::Success(r) => {
          assert_eq!(r.to_num(), 1.0)
        },
        NativeResult::RuntimeError(_) => assert!(false)
      }
    }
  }

  #[cfg(test)]
  mod has {
    use super::*;
    use fnv::FnvHashMap;
    use crate::support::test_native_dependencies;

    #[test]
    fn new() {
      let map_has = MapHas::new();
  
      assert_eq!(map_has.meta.name, "has");
      assert_eq!(map_has.meta.arity, ArityKind::Fixed(1));
    }
  
    #[test]
    fn call() {
      let map_has = MapHas::new();
      let (gc, context) = test_native_dependencies();

      let mut map = FnvHashMap::default();
      map.insert(Value::Nil, Value::Nil);
      let this = gc.manage(map, &*context);
  
      let result = map_has.call(&gc, &*context, Value::Map(this), &[Value::Nil]);
      match result {
        NativeResult::Success(r) => {
          assert_eq!(r.to_bool(), true)
        },
        NativeResult::RuntimeError(_) => assert!(false)
      }

      let result = map_has.call(&gc, &*context, Value::Map(this), &[Value::Bool(false)]);
      match result {
        NativeResult::Success(r) => {
          assert_eq!(r.to_bool(), false)
        },
        NativeResult::RuntimeError(_) => assert!(false)
      }
    }
  }

  #[cfg(test)]
  mod get {
    use super::*;
    use fnv::FnvHashMap;
    use crate::support::test_native_dependencies;

    #[test]
    fn new() {
      let map_get = MapGet::new();
  
      assert_eq!(map_get.meta.name, "get");
      assert_eq!(map_get.meta.arity, ArityKind::Fixed(1));
    }
  
    #[test]
    fn call() {
      let map_get = MapGet::new();
      let (gc, context) = test_native_dependencies();

      let mut map = FnvHashMap::default();
      map.insert(Value::Nil, Value::Bool(false));
      let this = gc.manage(map, &*context);
  
      let result = map_get.call(&gc, &*context, Value::Map(this), &[Value::Nil]);
      match result {
        NativeResult::Success(r) => {
          assert_eq!(r.to_bool(), false)
        },
        NativeResult::RuntimeError(_) => assert!(false)
      }

      let result = map_get.call(&gc, &*context, Value::Map(this), &[Value::Bool(true)]);
      match result {
        NativeResult::Success(r) => {
          assert!(r.is_nil())
        },
        NativeResult::RuntimeError(_) => assert!(false)
      }
    }
  }
}