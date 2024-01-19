use clap::Parser;
use rand::Rng;

#[derive(Parser, Debug)]
#[clap(
	name = "pwgen",
	version = "1.1.0",
	author = "Rhelvetican",
	long_about = env!("CARGO_PKG_DESCRIPTION"),
	about = "A simple password generator."
)]
struct Cli {
	#[clap(long, short, action, default_value = "8")]
	length: u8,
	#[clap(long, short, action)]
	number_enabled: bool,
	#[clap(long, short, action)]
	capital_enabled: bool,
	#[clap(long, short, action)]
	special_enabled: bool,
}

fn main() {
	let args = Cli::parse();

	let l: u8 = args.length;
	let n: bool = args.number_enabled;
	let c: bool = args.capital_enabled;
	let s: bool = args.special_enabled;

	let mut pool: Vec<char> = Vec::new();

	pool.extend('a'..='z');

	if n {
		pool.extend('0'..='9');
	}

	if c {
		pool.extend('A'..='Z');
	}

	if s {
		pool.extend("!@#$%^&*()".chars());
	}

	let mut rng = rand::thread_rng();

	let mut password = String::new();

	for _ in 0..=l {
		let i = rng.gen_range(0..pool.len());
		password.push(pool[i]);
	}

	println!("{}", password);
}
