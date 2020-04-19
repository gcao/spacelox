use spacelox_core::io::{StdIo, NativeStdIo};
use spacelox_core::managed::Trace;
use spacelox_core::{hooks::HookContext, memory::{Gc, NoGc}};

pub struct TestContext<'a> {
  gc: &'a Gc,
  no_gc: NoGc,
}

impl<'a> TestContext<'a> {
  pub fn new(gc: &'a Gc) -> Self { 
    Self {
      gc,
      no_gc: NoGc()
    }
  }
}

#[cfg(test)]
impl<'a> HookContext for TestContext<'a> {
  fn gc(&self) -> &Gc { self.gc }
}

#[cfg(test)]
impl<'a> Trace for TestContext<'a> {
  fn trace(&self) -> bool {
    self.no_gc.trace()
  }

  fn trace_debug(&self, stdio: &dyn StdIo) -> bool {
    self.no_gc.trace_debug(stdio)
  }
}

#[cfg(test)]
pub fn test_native_dependencies() -> Box<Gc> {
  Box::new(Gc::new(Box::new(NativeStdIo())))
}

