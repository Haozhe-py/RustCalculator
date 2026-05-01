
use parse_input::Token;


pub struct AST {
	token:Token,
	children:Vec<AST>,
	computed:bool,
	value:f64
}

impl AST {
	pub fn build(tokens:&Vec<Token>)->Option<AST> {
		let mut pos=0;
		Self::parse_expr(tokens, &mut pos)
	}
	fn parse_expr(tokens:&Vec<Token>, pos : &mut usize)->Option<AST> {
		let mut left = Self::parse_term(tokens, pos)?;
		while *pos<tokens.len() {
			let cur_tok = &tokens[*pos];
			if !cur_tok.is_num && (cur_tok.oper == '+' || cur_tok.oper == '-'){
				*pos += 1;
				let right = Self::parse_term(tokens, pos)?;
				left = AST {
					token:cur_tok.clone(),
					children:vec![left, right],
					computed:false, value:0.0
				};
			} else {
				break;
			}
		}
		Some(left)
	}
	fn parse_term(tokens:&Vec<Token>, pos : &mut usize)->Option<AST> {
		let mut left = Self::parse_fac(tokens, pos)?;
		while *pos<tokens.len() {
			let cur_tok = &tokens[*pos];
			if !cur_tok.is_num && (cur_tok.oper == '*' || cur_tok.oper == '/' || cur_tok.oper == '%'){
				*pos += 1;
				let right = Self::parse_fac(tokens, pos)?;
				left = AST {
					token:cur_tok.clone(),
					children:vec![left, right],
					computed:false, value:0.0
				};
			} else {
				break;
			}
		}
		Some(left)
	}
	fn parse_fac(tokens:&Vec<Token>, pos : &mut usize)->Option<AST> {
		let mut left = Self::parse_block(tokens, pos)?;
		while *pos<tokens.len() {
			let cur_tok = &tokens[*pos];
			if !cur_tok.is_num && cur_tok.oper == '^' {
				*pos += 1;
				let right = Self::parse_block(tokens, pos)?;
				left = AST {
					token:cur_tok.clone(),
					children:vec![left, right],
					computed:false, value:0.0
				};
			} else {
				break;
			}
		}
		Some(left)
	}
	fn parse_block(tokens:&Vec<Token>, pos : &mut usize)->Option<AST> {
		if *pos>=tokens.len() {return None;}
		let tok = &tokens[*pos];
		if tok.is_num {
			*pos += 1;
			Some(AST {
				token:tok.clone(), children:Vec::new(), computed:true, value:tok.num
			})
		} else if tok.oper == '(' {
			*pos += 1;
			let ast = Self::parse_expr(tokens, pos)?;
			if *pos<tokens.len() && tokens[*pos].oper == ')' {
				*pos += 1;
				Some(ast)
			} else {
				None
			}
		} else {None}
	}
	pub fn evaluate(self : &mut AST)->f64 {
		if self.computed {return self.value;}
		if self.token.is_num {self.computed=true;self.value=self.token.num;return self.token.num;}
		if self.children.len()!=2 {return 0.0;} // Error

		let left = self.children[0].evaluate();
		let right = self.children[1].evaluate();

		self.value = match self.token.oper {
			'+'=>left + right,
			'-'=>left - right,
			'*'=>left * right,
			'/'=>left / right,
			'%'=>left % right,
			'^'=>left.powf(right),
			_  => panic!("Unexpected operator: {}", self.token.oper)
		};
		self.computed = true;
		self.value
	}
}


