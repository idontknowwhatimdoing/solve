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

fn display_vecs(left: &Vec<Term>, right: &Vec<Term>) {
	for term in left {
		print!("{} ", *term);
	}
	print!("= ");
	for term in right {
		print!("{} ", *term);
	}
	println!("\n");
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

			println!("\nafter isolating variables and constants :");
			display_vecs(&terms_left, &terms_right);

			let (result_left, result_right) = steps::reduce(&terms_left, &terms_right);

			println!(
				"after reducing the members : {} = {}\n",
				result_left, result_right
			);

			steps::final_calcul(result_left, result_right);
		} else {
			user_guide();
		}
	}
}
