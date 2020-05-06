#[derive(Debug, PartialEq)]
pub enum Sexpr<'s> {
	Atom(Box<Atom<'s>>),
	List(Vec<Sexpr<'s>>),
}

#[derive(Debug, PartialEq)]
pub enum Atom<'a> {
	Nil,
	Bool(bool),
	Int(i64),
	Float(f64),
	Str(&'a str),
	Sym(&'a str),
	Keyword(&'a str),
	Vec(Vec<Atom<'a>>),
}

impl<'a> From<i64> for Atom<'a> {
	fn from(i: i64) -> Atom<'a> {
		Atom::Int(i)
	}
}

impl<'a> From<f64> for Atom<'a> {
	fn from(f: f64) -> Atom<'a> {
		Atom::Float(f)
	}
}

impl <'a> From<bool> for Atom<'a> {
	fn from(b: bool) -> Atom<'a> {
		Atom::Bool(b)
	}
}
