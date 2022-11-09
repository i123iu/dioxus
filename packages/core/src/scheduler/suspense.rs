use std::{
    cell::{Cell, RefCell},
    collections::HashSet,
    rc::Rc,
};

use super::{waker::RcWake, SchedulerMsg};
use crate::{
    innerlude::{Mutation, Mutations},
    Element, ScopeId,
};

use futures_util::Future;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct SuspenseId(pub usize);

pub type SuspenseContext = Rc<RefCell<SuspenseBoundary>>;

/// Essentially a fiber in React
pub struct SuspenseBoundary {
    pub id: ScopeId,
    pub waiting_on: HashSet<SuspenseId>,
    pub mutations: Mutations<'static>,
}

impl SuspenseBoundary {
    pub fn new(id: ScopeId) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            id,
            waiting_on: Default::default(),
            mutations: Mutations::new(0),
        }))
    }
}

pub struct SuspenseLeaf {
    pub id: SuspenseId,
    pub scope_id: ScopeId,
    pub tx: futures_channel::mpsc::UnboundedSender<SchedulerMsg>,
    pub notified: Cell<bool>,

    pub task: *mut dyn Future<Output = Element<'static>>,
}

impl RcWake for SuspenseLeaf {
    fn wake_by_ref(arc_self: &Rc<Self>) {
        arc_self.notified.set(true);
        _ = arc_self
            .tx
            .unbounded_send(SchedulerMsg::SuspenseNotified(arc_self.id));
    }
}
