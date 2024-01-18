use clap::{Parser, Command, Arg};

const NUMBER: &str = "Number";
const T_F: &str = "True/False";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
	#[clap(short, long, default_value_t = 8, value_name = NUMBER, display_order = 1)]
	length: u8,
	#[clap(short, long, value_parser, default_value_t = true, value_name = T_F, display_order = 2)]
	number_enabled: bool,
	#[clap(short, long, value_parser, default_value_t = false, value_name = T_F, display_order = 3)]
	capital_enabled: bool,
	#[clap(short, long, value_parser, default_value_t = false, value_name = T_F, display_order = 4)]
	special_enabled: bool,
}

fn main() {
	let args = Args::parse();

	let length: u8 = args.length;
	let number_enabled: bool = args.number_enabled;
	let capital_enabled: bool = args.capital_enabled;
	let special_enabled: bool = args.special_enabled;

	let mut char_pool: Vec<char> = Vec::new();

	char_pool.extend('a'..='z');
	if capital_enabled {
		char_pool.extend('A'..='Z');
	}
	if number_enabled {
		char_pool.extend('0'..='9');
	}
	if special_enabled {
		char_pool.extend("!@#$%^&*()".chars());
	}

	let mut password: String = String::new();

	for _ in 0..length {
		let random_index: usize = rand::random::<usize>() % char_pool.len();
		password.push(char_pool[random_index]);
	}

	println!("{}", password);
}
