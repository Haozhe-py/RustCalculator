use std::io;
use std::io::Write;

mod parse_input;
mod ast;
mod calculator;

fn main()
{
	println!("Welcome to use RustCalculator. \nEnter \"quit\" to exit. ");
	loop {
		let mut input=String::new();
		print!("\n> ");
		io::stdout().flush().unwrap();
		io::stdin().read_line(&mut input).unwrap();
		let mut input = input.trim();
		if input=="quit" {break;}
		println!("{}", calculator::calc(&mut input));
	}
}
