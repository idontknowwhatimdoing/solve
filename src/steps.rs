pub fn isolate<'a>(dest: &mut Vec<(char, &'a str)>, src: &mut Vec<(char, &'a str)>) {
	while !src.is_empty() {
		let item = src.pop().unwrap();

		if item.0 == '-' {
			dest.push(('+', item.1));
		} else {
			dest.push(('-', item.1));
		}
	}
}

pub fn reduce(terms: &mut Vec<(char, &str)>) -> String {
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

fn approx_result(coef: &str, right: &str) -> f32 {
	let coef_value = coef.parse::<f32>().unwrap();
	let right_value = right.parse::<f32>().unwrap();

	right_value / coef_value
}

pub fn final_calcul(left: String, right: String) {
	if left.len() > 1 {
		let mut final_result = String::from("x = ");
		final_result.push_str(right.as_str());
		final_result.push('/');

		let coef = left.get(0..left.len() - 1).unwrap();
		if coef == "0" {
			println!("not solvable...\n");
			return;
		}
		final_result.push_str(coef);

		let approx = approx_result(coef, right.as_str());

		println!("solution : {}", final_result);
		println!("       <=> x = {}\n", approx);
	}
}
