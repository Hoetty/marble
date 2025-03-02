use std::rc::Rc;

use crate::value::ValueRef;

pub type EnvRef = Rc<Environment>;

#[derive(Clone, Debug)]
pub enum Environment where {
    Value {
        value: ValueRef,
        parent: EnvRef,
    },
    Root
}

impl  Environment where {

    pub fn extend(environment: EnvRef, value: ValueRef) -> EnvRef {
        Rc::new(Environment::Value { value, parent: Rc::clone(&environment) })
    }

    pub fn pop(environment: &EnvRef) -> EnvRef {
        match Rc::as_ref(environment) {
            Environment::Value { value: _, parent } => Rc::clone(parent),
            Environment::Root => panic!("Popped the root Environment"),
        }
    }

    pub fn clone(environment: &EnvRef) -> EnvRef {
        Rc::clone(environment)
    }

    pub fn root() -> EnvRef {
        Rc::new(Environment::Root)
    }

    pub fn find(&self, depth: usize) -> ValueRef {
        match self {
            Self::Root => panic!("Tried to get on environment root"),
            Self::Value { value, parent } => if depth == 0 {
                ValueRef::clone(value)
            } else {
                parent.find(depth - 1)
            }
        }
    }
}