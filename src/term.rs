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
		if self.kind == Kind::Const {
			self.value.to_string()
		} else {
			let mut ret = String::new();
			ret.push_str(self.value.to_string().as_str());
			ret.push('x');
			ret
		}
	}

	pub fn is_positive(&self) -> bool {
		self.value >= 0
	}
}

fn add_const(terms: &mut Vec<Term>, chars: &Vec<char>, start: usize, value: i32) {
	if start == 0 {
		terms.push(Term::new(Kind::Const, value));
	} else if chars[start - 1] == '-' {
		terms.push(Term::new(Kind::Const, -value));
	} else {
		terms.push(Term::new(Kind::Const, value));
	}
}

pub fn get_constants(member: &str) -> Vec<Term> {
	let mut constants: Vec<Term> = Vec::new();
	let chars: Vec<char> = member.chars().collect();

	let re = Regex::new(r"\d+").unwrap();
	for mat in re.find_iter(member) {
		let value = mat.as_str().parse::<i32>().unwrap();

		if value == 0 {
			continue;
		}

		if mat.end() < chars.len() {
			if chars[mat.end()] == '+' || chars[mat.end()] == '-' {
				add_const(&mut constants, &chars, mat.start(), value);
			}
		} else {
			add_const(&mut constants, &chars, mat.start(), value);
		}
	}

	constants
}

pub fn get_variables(member: &str) -> Vec<Term> {
	let mut variables: Vec<Term> = Vec::new();
	let chars: Vec<char> = member.chars().collect();

	let re = Regex::new(r"\d*x").unwrap();
	for mat in re.find_iter(member) {
		let value;

		if mat.as_str().len() == 1 {
			value = 1;
		} else {
			let last = mat.as_str().len() - 1;
			value = mat.as_str()[0..last].parse::<i32>().unwrap();

			if value == 0 {
				continue;
			}
		}

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

// fn add_term(terms: &mut Vec<Term>, is_var: bool, is_pos: bool, value: i32) {
// 	if is_var && is_pos {
// 		terms.push(Term::new(Kind::Var, value));
// 	} else if is_var && !is_pos {
// 		terms.push(Term::new(Kind::Var, -value));
// 	} else if !is_var && is_pos {
// 		terms.push(Term::new(Kind::Const, value));
// 	} else if !is_var && !is_pos {
// 		terms.push(Term::new(Kind::Const, -value));
// 	}
// }

// pub fn get_terms(equation: &String) -> Vec<Term> {
// 	let mut terms = Vec::new();
// 	let chars: Vec<char> = equation.chars().collect();

// 	let re = Regex::new(r"\d+x?|x").unwrap();
// 	for mat in re.find_iter(equation) {
// 		let is_var = mat.as_str().contains("x");

// 		let value;
// 		if is_var {
// 			if mat.as_str().len() == 1 {
// 				value = 1;
// 			} else {
// 				let last = mat.as_str().len() - 1;
// 				value = mat.as_str()[0..last].parse::<i32>().unwrap();

// 				if value == 0 {
// 					continue;
// 				}
// 			}
// 		} else {
// 			value = mat.as_str().parse::<i32>().unwrap();

// 			if value == 0 {
// 				continue;
// 			}
// 		}

// 		let is_pos = if mat.start() > 0 && chars[mat.start() - 1] == '-' {
// 			false
// 		} else {
// 			true
// 		};

// 		add_term(&mut terms, is_var, is_pos, value);
// 	}

// 	terms
// }
