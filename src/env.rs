use super::values::DialValue;
use std::collections::HashMap;

#[derive(Debug)]
// TODO figure out how to solve this
// NOTE this might help https://github.com/kanaka/mal/blob/master/rust/env.rs
pub struct Env {
	scopes: Vec<Scope>,
	current: usize,
}

impl Env {
	pub fn new() -> Env {
		Env {
			scopes: vec![Scope::new()],
			current: 0,
		}
	}

	pub fn push_stack(mut self) -> usize {
		self.scopes.push(Scope {
			parent_scope: self.current,
			layer: self.scopes.len(),
			data: HashMap::new(),
		});

		self.current = self.scopes.len();

		self.scopes.len()
	}

	pub fn set(&mut self, key: String, value: DialValue) {
		self.scopes[self.current].data.insert(key, value);
	}

	pub fn find(&mut self, key: String) -> Option<&Scope> {
		if self.scopes[self.current].data.contains_key(&key) {
			return Some(&self.scopes[self.current]);
		}

		for i in self.current..0 {
			match self.scopes.get(i) {
				Some(scope) => {
					if scope.data.contains_key(&key) {
						return Some(&scope);
					}
				}
				None => continue,
			}
		}

		None
	}

	pub fn get(self, key: String) -> Option<DialValue> {
		unimplemented!();
	}

	fn current_scope(&self) -> &Scope {
		&self.scopes[self.current]
	}
}

#[derive(Debug, PartialEq, Clone)]
pub struct Scope {
	parent_scope: usize,
	layer: usize,
	data: HashMap<String, DialValue>,
}

impl Scope {
	fn new() -> Scope {
		Scope {
			parent_scope: 0,
			layer: 0,
			data: HashMap::new(),
		}
	}

	fn add_scope(self) -> Scope {
		Scope {
			parent_scope: self.layer,
			layer: self.layer + 1,
			data: HashMap::new(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn set_should_set_value_in_scope() {
		let mut env = Env::new();
		let key = String::from("x");

		env.set(key.clone(), DialValue::Integer(2));

		assert!(env.scopes[env.current].data.contains_key(&key));
	}

	#[test]
	fn find_should_get_current_scope() {
		let mut env = Env::new();
		let key = String::from("x");

		let val = DialValue::Integer(2);

		env.set(key.clone(), val);

		let result = env.find(key);

		assert!(result.is_some());
		assert_eq!(env.current_scope(), result.unwrap());
	}

	#[test]
	fn find_should_get_parent_scope() {
		let mut env = Env::new();
		let key = String::from("x");

		let val = DialValue::Integer(2);

		env.set(key.clone(), val);

		let result = env.find(key);

		env.push_stack();

		assert!(result.is_some());
		assert_eq!(env.scopes[env.current], *result.unwrap());
	}
}
