use super::{builtin, DialVal};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Clone)]
pub struct Env {
    symbol_map: RefCell<HashMap<String, DialVal>>,
    scope: Option<Rc<Env>>,
}

impl Default for Env {
    fn default() -> Self {
        let mut root = HashMap::new();

        root.insert(
            "+".into(),
            DialVal::Builtin {
                name: "+".into(),
                func: builtin::add,
            },
        );

        root.insert(
            "-".into(),
            DialVal::Builtin {
                name: "-".into(),
                func: builtin::sub,
            },
        );

        root.insert(
            "*".into(),
            DialVal::Builtin {
                name: "*".into(),
                func: builtin::mul,
            },
        );

        root.insert(
            "/".into(),
            DialVal::Builtin {
                name: "/".into(),
                func: builtin::div,
            },
        );

        Env {
            symbol_map: RefCell::new(root),
            scope: None,
        }
    }
}

impl Env {
    pub fn with_scope(scope: Env) -> Env {
        Env {
            symbol_map: RefCell::new(HashMap::new()),
            scope: Some(Rc::new(scope)),
        }
    }

    pub fn get_value(&self, sym: String) -> Option<DialVal> {
        let map = self.symbol_map.borrow();

        let res = map.get(&sym);

        match res {
            Some(val) => Some(val.clone()),
            None => match &self.scope {
                Some(scope) => scope.get_value(sym),
                None => None,
            },
        }
    }

    pub fn set_value(&self, sym: String, val: DialVal) {
        self.symbol_map.borrow_mut().insert(sym, val);
    }
}
