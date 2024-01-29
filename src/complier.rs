use crate::scanner::*;

pub fn compile(source: String) {
    let mut scanner = Scanner::new(source);
    loop {
        let token = scanner.scan_token();
        if token.line != scanner.line {
            print!("{}", token.line);
            scanner.line = token.line;
        }else{
            print!("   |");
        }

        print!("{:2} '{:.*} \n'", token.token_type, token.length, token.start);

        if token.token_type == TokenType::EOF {
            break;
        }
    }
}