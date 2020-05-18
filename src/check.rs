use regex::Regex;

pub fn is_valid(equation: &String) -> bool {
	check_equals(&equation)
		&& check_syntax(&equation)
		&& !check_chars(&equation)
		&& !check_operators(&equation)
		&& check_x(&equation)
		&& check_boundaries(&equation)
}

fn check_equals(equation: &String) -> bool {
	let vec_equals: Vec<_> = equation.match_indices("=").collect();

	vec_equals.len() == 1
}

fn check_x(equation: &String) -> bool {
	let vec_x: Vec<char> = equation.chars().collect();
	let mut x_ok = false;

	for i in 0..vec_x.len() {
		if vec_x[i] == 'x' {
			if i < vec_x.len() - 1 {
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
					x_ok = false;
					break;
				}
			} else {
				x_ok = true;
			}
		}
	}

	x_ok
}

fn check_operators(equation: &String) -> bool {
	let re_operators = Regex::new(r"[+-/]{2,}").unwrap();

	re_operators.is_match(equation)
}

fn check_chars(equation: &String) -> bool {
	let re_chars = Regex::new(r"[^-\+/=x0-9()]").unwrap();

	re_chars.is_match(equation)
}

fn check_boundaries(equation: &String) -> bool {
	!(equation.starts_with('/') || equation.ends_with('/'))
}

fn check_syntax(equation: &String) -> bool {
	let re_main = Regex::new(r"([+-]?\d+|[+-]?x)+=([+-]?\d+|[+-]?x)+").unwrap();

	re_main.is_match(equation)
}
