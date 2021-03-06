use assert2::check;

fn main() {
	let mut vec = Vec::new();
	vec.push(1);

	check!(let Some(_) = Some(1).filter(|_| false), "waaah: {}", 10);

	{
		check!(&vec == &vec![]);
		eprintln!("This still executes!");
	}

	eprintln!("But this does not.");
}
