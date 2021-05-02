use crate::val::Val;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub type Env = Rc<RefCell<EnvCore>>;

#[derive(Clone)]
pub struct EnvCore {
    bindings: HashMap<String, Val>,
    parent: Option<Env>,
}

impl EnvCore {
    pub fn extend(other: &Env) -> EnvCore {
        EnvCore {
            bindings: HashMap::new(),
            parent: Some(Rc::clone(other)),
        }
    }

    pub fn get(&mut self, name: String) -> Val {
        match self.bindings.get(&name) {
            Some(val) => val.clone(),
            None => match &self.parent {
                Some(parent) => parent.borrow_mut().get(name),
                None => Val::Nil,
            },
        }
    }

    pub fn set(&mut self, name: String, val: Val) {
        self.bindings.insert(name, val);
    }
}
