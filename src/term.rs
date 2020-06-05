use regex::Regex;

#[derive(PartialEq, Copy, Clone)]
pub enum Kind {
	Const,
	Var,
}

pub struct Term {
	pub kind: (Kind, Kind),
	pub num: i32,
	pub den: i32,
}

impl Term {
	pub fn new(kind: (Kind, Kind), num: i32, den: i32) -> Term {
		Term { kind, num, den }
	}

	pub fn to_string(&self) -> String {
		let mut ret = String::new();
		let is_var_num = self.kind.0 == Kind::Var;
		let is_var_den = self.kind.1 == Kind::Var;

		if self.den == 1 {
			add_str(&mut ret, self.num, is_var_num);
		} else if self.den == -1 {
			add_str(&mut ret, -self.num, is_var_num);
		} else {
			add_str(&mut ret, self.num, is_var_num);
			ret.push('/');
			add_str(&mut ret, self.den, is_var_den);
		}

		ret
	}

	pub fn is_positive(&self) -> bool {
		if self.num > 0 && self.den > 0 || self.num < 0 && self.den < 0 {
			true
		} else {
			false
		}
	}

	pub fn is_var(&self) -> bool {
		self.kind.0 == Kind::Var || self.kind.1 == Kind::Var
	}
}

fn add_str(ret: &mut String, val: i32, is_var: bool) {
	if !is_var {
		ret.push_str(val.to_string().as_str());
	} else {
		ret.push_str(val.to_string().as_str());
		ret.push('x');
	}
}

fn add_term(terms: &mut Vec<Term>, is_var: (bool, bool), num: i32, den: i32) {
	if is_var.0 && is_var.1 {
		terms.push(Term::new((Kind::Const, Kind::Const), num, den));
	} else if is_var.0 && !is_var.1 {
		terms.push(Term::new((Kind::Var, Kind::Const), num, den));
	} else if !is_var.0 && is_var.1 {
		terms.push(Term::new((Kind::Const, Kind::Var), num, den));
	} else if !is_var.0 && !is_var.1 {
		terms.push(Term::new((Kind::Const, Kind::Const), num, den));
	}
}

fn get_int_terms(member: &str, mut terms: &mut Vec<Term>, chars: &Vec<char>) {
	let re = Regex::new(r"-?\d+x?|-?x").unwrap();
	for mat in re.find_iter(member) {
		if chars[mat.end()] != '/' && chars[mat.start() - 1] != '/' && chars[mat.start() - 2] != '/'
		{
			let is_var = mat.as_str().contains("x");

			let value;
			if is_var {
				if mat.as_str().len() == 1 {
					value = 1;
				} else if mat.as_str().len() == 2 && chars[mat.start()] == '-' {
					value = -1;
				} else {
					let last = mat.as_str().len() - 1;
					value = mat.as_str()[0..last].parse::<i32>().unwrap();
				}
			} else {
				value = mat.as_str().parse::<i32>().unwrap();
			}
			if value == 0 {
				continue;
			}

			add_term(&mut terms, (is_var, false), value, 1);
		}
	}
}

fn get_float_terms(member: &str, mut terms: &mut Vec<Term>, chars: &Vec<char>) {
	let re = Regex::new(r"(-?\d+x?|-?x)\/(-?\d+x?|-?x)").unwrap();
	for mat in re.captures_iter(member) {
		let num_mat = mat.get(1).unwrap();
		let is_var_num = num_mat.as_str().contains("x");
		let num;
		if is_var_num {
			if num_mat.as_str().len() == 1 {
				num = 1;
			} else if num_mat.as_str().len() == 2 && chars[num_mat.start()] == '-' {
				num = -1;
			} else {
				let last = num_mat.as_str().len() - 1;
				num = num_mat.as_str()[0..last].parse::<i32>().unwrap();
			}
		} else {
			num = num_mat.as_str().parse::<i32>().unwrap();
		}

		let den_mat = mat.get(2).unwrap();
		let is_var_den = den_mat.as_str().contains("x");
		let den;
		if is_var_den {
			if den_mat.as_str().len() == 1 {
				den = 1;
			} else if den_mat.as_str().len() == 2 && chars[den_mat.start()] == '-' {
				den = -1;
			} else {
				let last = den_mat.as_str().len() - 1;
				den = den_mat.as_str()[0..last].parse::<i32>().unwrap();
			}
		} else {
			den = den_mat.as_str().parse::<i32>().unwrap();
		}

		add_term(&mut terms, (is_var_num, is_var_den), num, den);
	}
}

pub fn get_terms(member: &str) -> Vec<Term> {
	let mut terms = Vec::new();
	let chars: Vec<char> = member.chars().collect();

	get_int_terms(member, &mut terms, &chars);
	get_float_terms(member, &mut terms, &chars);

	terms
}
