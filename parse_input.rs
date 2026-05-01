fn is_num(c:char)->bool{
	match c {
		'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'|'0'|'.'=>true,
		_=>false
	}
}

fn is_oper(c:char)->bool{
	match c{
		'+'|'-'|'*'|'/'|'%'|'('|')'|'^'=>true,
		_=>false
	}
}

#[derive(Clone)]
pub struct Token {
	pub num:f64,
	pub oper:char,
	pub is_num:bool
}

fn is_unary(tokens:&Vec<Token>)->bool {
	tokens.is_empty() || {
		let last = &tokens[tokens.len()-1];
		(!last.is_num)&&(last.oper != ')')
	}
}

pub fn parse_input(input: &str) -> Result<Vec<Token>, String> {
    let mut has_num = false;
    let mut number = String::new();
    let mut result: Vec<Token> = Vec::new();
    
    for c in input.chars() {  // 注意 input 需要调用 chars()
        if c.is_whitespace() {
            // 处理空格：如果正在累积数字则先提交
            if has_num {
                let num = number.parse::<f64>()
                    .map_err(|_e| format!("无效数字: {}", number))?;
                result.push(Token { num, oper: '\0', is_num: true });
                number.clear();
                has_num = false;
            }
        } else if number.is_empty()&&(c=='-' || c=='+')&&is_unary(&result) {
	number.push(c);
	has_num = true;
	}  else if is_num(c) {
            number.push(c);
            has_num = true;
        } else if is_oper(c) {
            if has_num {
                let num = number.parse::<f64>()
                    .map_err(|_e| format!("无效数字: {}", number))?;
                result.push(Token { num, oper: '\0', is_num: true });
                number.clear();
                has_num = false;
            }
            result.push(Token { num: 0.0, oper: c, is_num: false });
        } else {
            return Err(format!("非法字符: '{}'", c));
        }
    }
    
    // 处理末尾的数字
    if has_num {
        let num = number.parse::<f64>()
            .map_err(|_e| format!("无效数字: {}", number))?;
        result.push(Token { num, oper: '\0', is_num: true });
    }
    
    Ok(result)
}

