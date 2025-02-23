use std::rc::Rc;

use crate::value::Value;

pub type EnvRef = Rc<Environment>;

#[derive(Clone)]
pub enum Environment where {
    Value {
        depth: usize,
        value: Value,
        parent: EnvRef,
    },
    Root
}

impl  Environment where {

    pub fn extend(environment: EnvRef, value: Value) -> EnvRef {
        Rc::new(Environment::Value { value, parent: Rc::clone(&environment), depth: environment.next_depth() })
    }

    pub fn next_depth(&self) -> usize {
        match self {
            Environment::Value { depth, value: _, parent: _ } => depth + 1,
            Environment::Root => 0,
        }
    }

    pub fn pop(environment: &EnvRef) -> EnvRef {
        match Rc::as_ref(environment) {
            Environment::Value { depth: _, value: _, parent } => Rc::clone(parent),
            Environment::Root => panic!("Popped the root Environment"),
        }
    }

    pub fn clone(environment: &EnvRef) -> EnvRef {
        Rc::clone(environment)
    }

    pub fn root() -> EnvRef {
        Rc::new(Environment::Root)
    }

    pub fn distance_from_top(&self, depth: usize) -> usize {
        match self {
            Environment::Value { depth: other, value: _, parent: _ } => other - depth,
            Environment::Root => depth + 1,
        }
    }

    pub fn from_bottom(&self, depth: usize) -> Value {
        self.from_top(self.distance_from_top(depth))
    }

    pub fn from_top(&self, depth: usize) -> Value {
        match self {
            Self::Root => panic!("Tried to get on environment root"),
            Self::Value { depth: _, value, parent } => if depth == 0 {
                value.clone()
            } else {
                parent.from_top(depth - 1)
            }
        }
    }
}