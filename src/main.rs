mod check;
mod steps;
mod term;

use std::env::args;
use term::Term;

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
		if term.is_positive() {
			full.push('+');
		}
		full.push_str(term.to_string().as_str());
		full.push(' ');
	}
	full.push_str("= ");
	for term in right {
		if term.is_positive() {
			full.push('+');
		}
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

			let mut terms_left = term::get_terms(left_member);
			let mut terms_right = term::get_terms(right_member);

			steps::isolate(&mut terms_left, &mut terms_right);

			println!(
				"\nafter isolating variables and constants : {}\n",
				vecs_to_string(&terms_left, &terms_right)
			);

			let (result_left, result_right) = steps::reduce(&terms_left, &terms_right);

			println!(
				"after reducing the members : {}\n",
				concat_results(&result_left, &result_right)
			);

			steps::final_calcul(result_left, result_right);
		} else {
			user_guide();
		}
	}
}
