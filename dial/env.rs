use super::{builtin, DialVal};
use std::{cell::RefCell, collections::HashMap};

macro_rules! define_builtin {
    ($name:literal, $fn:path, $env:ident) => {
        $env.insert(
            $name.into(),
            DialVal::Builtin {
                name: $name.into(),
                func: $fn,
            },
        )
    };
}

#[derive(Debug, Clone)]
pub struct Scope {
    symbol_map: RefCell<HashMap<String, DialVal>>,
}

impl Scope {
    fn new() -> Self {
        Scope {
            symbol_map: RefCell::new(HashMap::new()),
        }
    }

    fn get_value(&self, sym: String) -> Option<DialVal> {
        let map = self.symbol_map.borrow();
        map.get(&sym).and_then(|v| Some(v.clone()))
    }

    fn set_value(&self, sym: String, val: DialVal) {
        self.symbol_map.borrow_mut().insert(sym, val);
    }

    // helper for function creation
    fn bind(&self, symbols: Vec<String>, vals: Vec<DialVal>) {
        for (sym, val) in symbols.iter().zip(vals) {
            self.set_value(sym.clone(), val);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Env {
    pub(crate) stack: Vec<Scope>,
}

impl Default for Env {
    fn default() -> Self {
        let mut root = HashMap::new();

        define_builtin!("+", builtin::add, root);
        define_builtin!("-", builtin::sub, root);
        define_builtin!("*", builtin::mul, root);
        define_builtin!("/", builtin::div, root);
        define_builtin!("list", builtin::list, root);
        define_builtin!("list?", builtin::is_list, root);
        define_builtin!("empty?", builtin::is_empty, root);
        define_builtin!("count", builtin::count, root);
        define_builtin!("=", builtin::eq, root);

        let scope = Scope {
            symbol_map: RefCell::new(root),
        };

        Env { stack: vec![scope] }
    }
}

impl Env {
    pub fn get_value(&self, sym: String) -> Option<DialVal> {
        for stack in self.stack.iter().rev() {
            match stack.get_value(sym.clone()) {
                Some(v) => return Some(v.clone()),
                None => continue,
            }
        }

        None
    }

    pub fn set_value(&mut self, sym: String, val: DialVal) {
        self.stack.last_mut().unwrap().set_value(sym, val);
    }

    pub fn def_value(&mut self, sym: String, val: DialVal) {
        self.stack.first_mut().unwrap().set_value(sym, val);
    }

    pub fn new_scope(&mut self) {
        self.stack.push(Scope::new());
    }

    pub fn bind(&mut self, symbols: Vec<String>, vals: Vec<DialVal>) {
        self.stack.last_mut().unwrap().bind(symbols, vals);
    }

    pub fn drop_scopes(&mut self, count: usize) {
        if count > 0 {
            self.stack.remove(count);
        }
    }
}
