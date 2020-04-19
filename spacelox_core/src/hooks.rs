

use crate::{managed::{Manage, Trace, Managed}, memory::Gc};

pub struct Hooks<'a> {
  context: &'a dyn HookContext,
}

impl<'a> Hooks<'a> {
  pub fn new(context: &'a mut dyn HookContext) -> Hooks<'a> {
    Hooks {
      context,
    }
  }

  pub fn manage<T: 'static + Manage>(&self, data: T) -> Managed<T> {
    self.context.gc().manage(data, self.context)
  }

  pub fn manage_str(&self, string: String) -> Managed<String> {
    self.context.gc().manage_str(string, self.context)
  }

  pub fn clone_managed<T: 'static + Manage + Clone>(
    &self,
    managed: Managed<T>,
  ) -> Managed<T> {
    self.context.gc().clone_managed(managed, self.context)
  }

  pub fn resize<T: 'static + Manage, R, F: Fn(&mut T) -> R>(
    &self,
    managed: &mut T,
    action: F,
  ) -> R {
    self.context.gc().resize(managed, self.context, action)
  }
}

pub trait HookContext: Trace {
  fn gc(&self) -> &Gc;
}

pub struct NoContext<'a> {
  gc: &'a Gc,
}

impl<'a> NoContext<'a> {
  pub fn new(gc: &'a Gc) -> Self {
    Self {
      gc
    }
  }
}

impl<'a> Trace for NoContext<'a> {
    fn trace(&self) -> bool { false }
    fn trace_debug(&self, _stdio: &dyn crate::io::StdIo) -> bool { false }
}

impl<'a> HookContext for NoContext<'a> {
    fn gc(&self) -> &Gc { self.gc }
}
