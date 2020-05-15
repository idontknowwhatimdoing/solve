use crate::term::Term;

pub fn isolate(dest: &mut Vec<Term>, src: &mut Vec<Term>) {
	while !src.is_empty() {
		let term = src.pop().unwrap();

		if term.value > 0 {
			dest.push(Term::new(term.kind, -term.value));
		} else {
			dest.push(Term::new(term.kind, term.value));
		}
	}
}

// 2 vecs with 1 call
pub fn reduce(terms: &Vec<Term>) -> Term {
	let mut result = 0;

	for term in terms {
		result += term.value;
	}

	Term::new(terms[0].kind, result)
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
