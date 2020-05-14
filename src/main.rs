use regex::Regex;
use std::env::args;

fn is_valid(equation: &String) -> bool {
	let vec_equals: Vec<_> = equation.match_indices("=").collect();

	let vec_x: Vec<char> = equation.chars().collect();
	let mut x_ok = false;
	for i in 0..vec_x.len() - 1 {
		if vec_x[i] == 'x' {
			if vec_x[i + 1] == '+'
				|| vec_x[i + 1] == '-'
				|| vec_x[i + 1] == '/'
				|| vec_x[i + 1] == '*'
				|| vec_x[i + 1] == '='
				|| vec_x[i + 1] == '('
				|| vec_x[i + 1] == ')'
			{
				x_ok = true;
			} else {
				break;
			}
		}
	}

	let re_sign = Regex::new(r"[+-]{2,}").unwrap();
	let re_chars = Regex::new(r"[^-\+*/=x0-9()]").unwrap();
	let re_main = Regex::new(r"([+-]?\d+|x)+=([+-]?\d+|[+-]?x)+").unwrap();

	if vec_equals.len() == 1
		&& !re_chars.is_match(equation)
		&& !re_sign.is_match(equation)
		&& re_main.is_match(equation)
		&& x_ok
	{
		true
	} else {
		println!("equation is not valid\nusage : ./solve equation_witout_spaces");
		false
	}
}

fn split_members(equation: &String) -> (&str, &str) {
	let terms: Vec<&str> = equation.split("=").collect();
	let left_member = terms[0];
	let right_member = terms[1];

	(left_member, right_member)
}

fn find_const(member: &str) -> Vec<(char, &str)> {
	let chars: Vec<char> = member.chars().collect();
	let mut constants: Vec<(char, &str)> = Vec::new();

	let re = Regex::new(r"\d+").unwrap();

	for mat in re.find_iter(member) {
		if mat.end() < chars.len() {
			if chars[mat.end()] == '+' || chars[mat.end()] == '-' {
				if mat.start() == 0 {
					constants.push(('+', mat.as_str()));
				} else if chars[mat.start() - 1] == '-' {
					constants.push(('-', mat.as_str()));
				} else if chars[mat.start() - 1] == '+' {
					constants.push(('+', mat.as_str()));
				}
			}
		} else {
			if mat.start() == 0 {
				constants.push(('+', mat.as_str()));
			} else if chars[mat.start() - 1] == '-' {
				constants.push(('-', mat.as_str()));
			} else if chars[mat.start() - 1] == '+' {
				constants.push(('+', mat.as_str()));
			}
		}
	}

	constants
}

fn find_order1_terms(member: &str) -> Vec<(char, &str)> {
	let chars: Vec<char> = member.chars().collect();
	let mut order1_terms: Vec<(char, &str)> = Vec::new();

	let re = Regex::new(r"\d*x").unwrap();

	for mat in re.find_iter(member) {
		if mat.start() > 0 {
			if chars[mat.start() - 1] == '-' {
				order1_terms.push(('-', mat.as_str()));
			} else {
				order1_terms.push(('+', mat.as_str()));
			}
		} else {
			order1_terms.push(('+', mat.as_str()));
		}
	}

	order1_terms
}

fn isolate<'a>(dest: &mut Vec<(char, &'a str)>, src: &mut Vec<(char, &'a str)>) {
	while !src.is_empty() {
		let item = src.pop().unwrap();

		if item.0 == '-' {
			dest.push(('+', item.1));
		} else {
			dest.push(('-', item.1));
		}
	}
}

fn reduce(terms: &mut Vec<(char, &str)>) -> String {
	let mut result = 0;
	let is_order1 = terms[0].1.contains("x");

	for term in terms {
		if is_order1 {
			if term.0 == '+' {
				match term.1[0..term.1.len() - 1].parse::<i32>() {
					Ok(value) => result += value,
					Err(_) => result += 1,
				}
			} else {
				match term.1[0..term.1.len() - 1].parse::<i32>() {
					Ok(value) => result -= value,
					Err(_) => result -= 1,
				}
			}
		} else {
			if term.0 == '+' {
				result += term.1.parse::<i32>().unwrap();
			} else {
				result -= term.1.parse::<i32>().unwrap();
			}
		}
	}

	if is_order1 {
		let mut result_str = result.to_string();
		result_str.push('x');
		result_str
	} else {
		result.to_string()
	}
}

fn main() {
	if args().len() != 2 {
		println!("usage : ./solve equation_without_spaces");
	} else {
		let equation = args().last().unwrap();

		if is_valid(&equation) {
			let (left_member, right_member) = split_members(&equation);

			let mut right_order1 = find_order1_terms(right_member);
			let mut left_order1 = find_order1_terms(left_member);

			isolate(&mut left_order1, &mut right_order1);

			let mut left_const = find_const(left_member);
			let mut right_const = find_const(right_member);

			isolate(&mut right_const, &mut left_const);

			let result_left = reduce(&mut left_order1);
			let result_right = reduce(&mut right_const);

			let mut result_full = String::new();
			result_full.push_str(result_left.as_str());
			result_full.push('=');
			result_full.push_str(result_right.as_str());
		}
	}
}
