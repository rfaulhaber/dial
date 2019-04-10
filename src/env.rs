use super::values::DialValue;
use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::AsRef;

#[derive(Clone, Debug)]
pub struct Env {
	symbol_map: RefCell<HashMap<String, DialValue>>,
	outer: Option<Box<Env>>,
}

impl Env {
	pub fn new() -> Env {
		Env {
			symbol_map: RefCell::new(HashMap::new()),
			outer: None,
		}
	}

	pub fn set(&self, symbol: &String, value: DialValue) {
		self.symbol_map.borrow_mut().insert(symbol.clone(), value);
	}

	pub fn get(&self, symbol: &String) -> Option<DialValue> {
		match self.find(symbol) {
			Some(env) => {
				let symbol_map = env.symbol_map.borrow();
				let val = symbol_map.get(symbol);

				val.cloned()
			}
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

impl AsRef<Env> for Env {
	fn as_ref(&self) -> &Self {
		self
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_contains_key() {
		let env = Env::new();

		env.set(&String::from("three"), DialValue::Integer(3));

		assert!(env.contains_symbol(&String::from("three")));
	}

	#[test]
	fn test_find() {
		let outer_env = Env::new();
		let three_symbol = &String::from("three");
		outer_env.set(three_symbol, DialValue::Integer(3));

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
		outer_env.set(three_symbol, DialValue::Integer(3));

		let boxed_outer = Box::new(outer_env);

		let mut inner_env = Env::new();
		inner_env.outer = Some(boxed_outer);

		let ret_val = inner_env.get(three_symbol);

		assert_eq!(ret_val, Some(DialValue::Integer(3)));
	}
}
