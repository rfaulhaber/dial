use crate::parser::{Atom, Expr};
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Env {
    symbol_map: RefCell<HashMap<String, Expr>>,
    outer: Option<Box<Env>>,
}

impl Env {
    pub fn new() -> Env {
        Env {
            symbol_map: RefCell::new(HashMap::new()),
            outer: None,
        }
    }

    pub fn from_outer(env: Box<Env>) -> Env {
        Env {
            symbol_map: RefCell::new(HashMap::new()),
            outer: Some(env),
        }
    }

    pub fn set(&self, symbol: &String, expr: Expr) {
        self.symbol_map.borrow_mut().insert(symbol.clone(), expr);
    }

    pub fn get(&self, symbol: &String) -> Option<Expr> {
        match self.find(symbol) {
            Some(env) => env.symbol_map.borrow().get(symbol).cloned(),
            None => None,
        }
    }

    pub fn push_scope(&self) -> Env {
        let mut inner_env = Env::new();
        inner_env.outer = Some(Box::new(self.clone()));

        inner_env
    }

    pub fn pop_scope(&self) -> Option<Env> {
        match &self.outer {
            Some(scope) => Some(*scope.to_owned()),
            None => None,
        }
    }

    fn find(&self, symbol: &String) -> Option<&Env> {
        if self.contains_symbol(&symbol) {
            return Some(self);
        }

        match &self.outer {
            Some(env) => env.find(symbol),
            None => None,
        }
    }

    fn contains_symbol(&self, symbol: &String) -> bool {
        self.symbol_map.borrow().contains_key(symbol)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_contains_key() {
        let env = Env::new();

        env.set(&String::from("three"), Atom::Integer(3).into());

        assert!(env.contains_symbol(&String::from("three")));
    }

    #[test]
    fn test_find() {
        let outer_env = Env::new();
        let three_symbol = &String::from("three");
        outer_env.set(three_symbol, Atom::Integer(3).into());

        let boxed_outer = Box::new(outer_env);

        let mut inner_env = Env::new();
        inner_env.outer = Some(boxed_outer);

        let ret_env = inner_env.find(three_symbol);

        assert!(ret_env.is_some());
        assert!(ret_env.unwrap().contains_symbol(three_symbol));
    }

    #[test]
    fn test_get() {
        let outer_env = Env::new();
        let three_symbol = &String::from("three");
        outer_env.set(three_symbol, Atom::Integer(3).into());

        let boxed_outer = Box::new(outer_env);

        let mut inner_env = Env::new();
        inner_env.outer = Some(boxed_outer);

        let ret_val = inner_env.get(three_symbol);

        assert_eq!(ret_val, Some(Atom::Integer(3).into()));
    }

    #[test]
    fn new_from_outer() {
        let outer_env = Env::new();
        let three_symbol = &String::from("three");
        outer_env.set(three_symbol, Atom::Integer(3).into());

        let inner = Env::from_outer(Box::new(outer_env));

        let ret_val = inner.get(three_symbol);

        assert_eq!(ret_val, Some(Atom::Integer(3).into()));
    }
}
