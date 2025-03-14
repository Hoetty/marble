use std::sync::{Arc, LazyLock};

use crate::value::ValueRef;

pub type EnvRef = Arc<Environment>;

#[derive(Clone, Debug)]
pub enum Environment {
    Value {
        value: ValueRef,
        parent: EnvRef,
    },
    Root
}

impl Environment {

    #[allow(clippy::declare_interior_mutable_const)]
    pub const ROOT: LazyLock<EnvRef> = LazyLock::new(|| EnvRef::new(Environment::Root));

    pub fn extend(environment: EnvRef, value: ValueRef) -> EnvRef {
        EnvRef::new(Environment::Value { value, parent: EnvRef::clone(&environment) })
    }

    pub fn pop(environment: &EnvRef) -> EnvRef {
        match EnvRef::as_ref(environment) {
            Environment::Value { value: _, parent } => EnvRef::clone(parent),
            Environment::Root => panic!("Popped the root Environment"),
        }
    }

    pub fn root() -> EnvRef {
        #[allow(clippy::borrow_interior_mutable_const)]
        Self::ROOT.clone()
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