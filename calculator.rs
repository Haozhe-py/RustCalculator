use parse_input;
use ast;
pub fn calc(inp:&str)->f64 {
	let tokens = parse_input::parse_input(inp).expect("REASON");
	let ast = ast::AST::build(&tokens);
	ast.expect("REASON").evaluate()
}

