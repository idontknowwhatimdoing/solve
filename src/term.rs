use regex::Regex;

pub fn get_constants(member: &str) -> Vec<(char, &str)> {
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

pub fn get_variables(member: &str) -> Vec<(char, &str)> {
	let chars: Vec<char> = member.chars().collect();
	let mut variables: Vec<(char, &str)> = Vec::new();

	let re = Regex::new(r"\d*x").unwrap();

	for mat in re.find_iter(member) {
		if mat.start() > 0 {
			if chars[mat.start() - 1] == '-' {
				variables.push(('-', mat.as_str()));
			} else {
				variables.push(('+', mat.as_str()));
			}
		} else {
			variables.push(('+', mat.as_str()));
		}
	}

	variables
}
