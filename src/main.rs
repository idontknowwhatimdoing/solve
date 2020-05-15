mod check;
mod steps;
mod term;

use std::env::args;

fn user_guide() {
	println!("\nyo wtf are you crazy\ngive me a real equation you stupid degenerate\n");
}

fn split_equation(equation: &String) -> (&str, &str) {
	let members: Vec<&str> = equation.split("=").collect();
	let left_member = members[0];
	let right_member = members[1];

	(left_member, right_member)
}

fn vecs_to_string(left: &Vec<(char, &str)>, right: &Vec<(char, &str)>) -> String {
	let mut full = String::new();

	for (c, s) in left {
		full.push(*c);
		full.push_str(s);
		full.push(' ');
	}
	full.push_str("= ");
	for (c, s) in right {
		full.push(*c);
		full.push_str(s);
		full.push(' ');
	}

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
				right_const.push(('+', "0"));
			}

			let full = vecs_to_string(&left_variables, &right_const);
			println!("\nafter isolating variables and constants : {}\n", full);

			let result_left = steps::reduce(&mut left_variables);
			let result_right = steps::reduce(&mut right_const);

			let mut result_full = String::new();
			result_full.push_str(result_left.as_str());
			result_full.push_str(" = ");
			result_full.push_str(result_right.as_str());

			println!("after reducing the members : {}\n", result_full);

			steps::final_calcul(result_left, result_right);
		} else {
			user_guide();
		}
	}
}
