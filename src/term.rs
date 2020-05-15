use regex::Regex;

#[derive(PartialEq, Copy, Clone)]
pub enum Kind {
	Const,
	Var,
}

pub struct Term {
	pub kind: Kind,
	pub value: i32,
}

impl Term {
	pub fn new(kind: Kind, value: i32) -> Term {
		Term { kind, value }
	}

	pub fn to_string(&self) -> String {
		self.value.to_string()
	}
}

pub fn get_constants(member: &str) -> Vec<Term> {
	let mut constants: Vec<Term> = Vec::new();
	let chars: Vec<char> = member.chars().collect();

	let re = Regex::new(r"\d+").unwrap();
	for mat in re.find_iter(member) {
		let value = mat.as_str().parse::<i32>().unwrap();

		if mat.end() < chars.len() {
			if chars[mat.end()] == '+' || chars[mat.end()] == '-' {
				if mat.start() == 0 {
					constants.push(Term::new(Kind::Const, value));
				} else if chars[mat.start() - 1] == '-' {
					constants.push(Term::new(Kind::Const, -value));
				} else if chars[mat.start() - 1] == '+' {
					constants.push(Term::new(Kind::Const, value));
				}
			}
		} else {
			if mat.start() == 0 {
				constants.push(Term::new(Kind::Const, value));
			} else if chars[mat.start() - 1] == '-' {
				constants.push(Term::new(Kind::Const, -value));
			} else if chars[mat.start() - 1] == '+' {
				constants.push(Term::new(Kind::Const, value));
			}
		}
	}

	constants
}

pub fn get_variables(member: &str) -> Vec<Term> {
	let mut variables: Vec<Term> = Vec::new();
	let chars: Vec<char> = member.chars().collect();

	let re = Regex::new(r"\d*x").unwrap();
	for mat in re.find_iter(member) {
		let value = mat.as_str().parse::<i32>().unwrap();

		if mat.start() > 0 {
			if chars[mat.start() - 1] == '-' {
				variables.push(Term::new(Kind::Var, -value));
			} else {
				variables.push(Term::new(Kind::Var, value));
			}
		} else {
			variables.push(Term::new(Kind::Var, value));
		}
	}

	variables
}
