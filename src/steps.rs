use crate::term::{Kind, Term};

pub fn isolate(left: &mut Vec<Term>, right: &mut Vec<Term>) {
	for i in 0..right.len() {
		if right[i].kind == Kind::Var {
			right.pop();
			left.push(Term::new(Kind::Var, -right[i].value));
		}
	}
	for i in 0..left.len() {
		if left[i].kind == Kind::Var {
			left.pop();
			right.push(Term::new(Kind::Const, -left[i].value));
		}
	}
}

pub fn reduce(left: &Vec<Term>, right: &Vec<Term>) -> (Term, Term) {
	let mut result = 0;

	for term in left {
		result += term.value;
	}
	let result_left = Term::new(left[0].kind, result);

	result = 0;

	for term in right {
		result += term.value;
	}
	let result_right = Term::new(right[0].kind, result);

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
