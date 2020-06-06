use crate::term::{Kind, Term};

pub fn isolate(left: &mut Vec<Term>, right: &mut Vec<Term>) {
	for i in 0..right.len() {
		if right[i].is_var() {
			left.push(Term::new(right[i].kind, -right[i].num, right[i].den));
			right.remove(i);
		}
	}
	for i in 0..left.len() {
		if !left[i].is_var() {
			right.push(Term::new(left[i].kind, -left[i].num, left[i].den));
			left.remove(i);
		}
	}
}

pub fn reduce(left: &Vec<Term>, right: &Vec<Term>) -> (Term, Term) {
	let mut result = 0;

	for term in left {
		result += term.num;
	}
	let result_left = Term::new(left[0].kind, result, 1);

	result = 0;

	for term in right {
		result += term.num;
	}
	let result_right = Term::new(right[0].kind, result, 1);

	(result_left, result_right)
}

pub fn final_calcul(left: Term, right: Term) {
	let mut final_result = String::from("x = ");

	final_result.push_str(right.to_string().as_str());
	final_result.push('/');

	if left.value == 0 {
		println!("not solvable...\n");
		return;
	}

	let last = left.to_string().len() - 1;
	final_result.push_str(&left.to_string().as_str()[0..last]);

	println!("solution : {}", final_result);
	println!(
		"       <=> x = {}\n",
		right.value as f32 / left.value as f32
	);
}
