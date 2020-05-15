mod check;
mod steps;
mod term;

use std::env::args;
use term::{Kind, Term};

fn user_guide() {
	println!("\nyo wtf are you crazy\ngive me a real equation you stupid degenerate\n");
}

fn split_equation(equation: &String) -> (&str, &str) {
	let members: Vec<&str> = equation.split("=").collect();
	let left_member = members[0];
	let right_member = members[1];

	(left_member, right_member)
}

fn vecs_to_string(left: &Vec<Term>, right: &Vec<Term>) -> String {
	let mut full = String::new();

	for term in left {
		full.push_str(term.to_string().as_str());
		full.push(' ');
	}
	full.push_str("= ");
	for term in right {
		full.push_str(term.to_string().as_str());
		full.push(' ');
	}

	full
}

fn concat_results(left: &Term, right: &Term) -> String {
	let mut full = String::new();
	full.push_str(left.to_string().as_str());
	full.push_str(" = ");
	full.push_str(right.to_string().as_str());

	full
}

fn main() {
	if args().len() != 2 {
		user_guide();
	} else {
		let equation = args().last().unwrap();

		if check::is_valid(&equation) {
			let (left_member, right_member) = split_equation(&equation);

			let mut right_variables = term::get_variables(right_member);
			let mut left_variables = term::get_variables(left_member);

			steps::isolate(&mut left_variables, &mut right_variables);

			let mut left_const = term::get_constants(left_member);
			let mut right_const = term::get_constants(right_member);

			steps::isolate(&mut right_const, &mut left_const);
			if right_const.is_empty() {
				right_const.push(Term::new(Kind::Const, 0));
			}

			let full = vecs_to_string(&left_variables, &right_const);
			println!("\nafter isolating variables and constants : {}\n", full);

			let result_left = steps::reduce(&mut left_variables);
			let result_right = steps::reduce(&mut right_const);

			let result_full = concat_results(&result_left, &result_right);
			println!("after reducing the members : {}\n", result_full);

			steps::final_calcul(result_left, result_right);
		} else {
			user_guide();
		}
	}
}
