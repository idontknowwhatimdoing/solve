use regex::Regex;
use std::env::args;

fn is_valid(equation: &String) -> bool {
	let re_sign = Regex::new(r"[+-]{2,}").unwrap();
	let re_alpha = Regex::new(r"[^-\+*/=x0-9()]").unwrap();
	let re_main = Regex::new(r"([+-]?\d+|x)+=([+-]?\d+|x)+").unwrap();

	if !re_alpha.is_match(equation) && !re_sign.is_match(equation) && re_main.is_match(equation) {
		println!("ok");
		true
	} else {
		println!("equation is not valid\nusage : ./solve equation_witout_spaces");
		false
	}
}

fn main() {
	if args().len() != 2 {
		println!("usage : ./solve equation_without_spaces");
	} else {
		let equation = args().last().unwrap();

		if is_valid(&equation) {}
	}
}
