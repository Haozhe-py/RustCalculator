use std::io;
use std::io::Write;

mod parse_input;
mod ast;
mod calculator;

fn main()
{
	println!("Welcome to use RustCalculator. \nEnter \"quit\" to exit. ");
	println!("\nThe following operators are supported:\n+ - * / % ^ ( )\n\nPlease DON'T enter any expressions with syntax error, or this program may panic. DON'T use \"=\" at the end of the expression.\n");
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
