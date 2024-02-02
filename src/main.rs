mod chunk;
mod compiler;
mod scanner;
mod value;
mod vm;
mod token;

use vm::*;

fn repl(vm: &mut vm::VM) {
    println!(
        r#" 
        █████╗ ██████╗  ██████╗
        ██╔══██╗██╔══██╗██╔════╝
        ███████║██████╔╝██║     
        ██╔══██║██╔══██╗██║     
        ██║  ██║██║  ██║╚██████╗
        ╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝ 
                [v1.1.0]
        "#
    ); 
    loop {
        print!("arc~> ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input = input.trim_end_matches('\n').to_string();

        //vm.interpret(input);
    }
}

fn eval(source: &str) {
    let mut vm = vm::VM::new();
    match vm.interpret(source.to_string()) {
        InterpretResult::CompileError => std::process::exit(65),
        InterpretResult::RuntimeError => std::process::exit(70),
        _ => (),
    }
}
fn main() {
    let mut vm = vm::VM::new();
    match std::env::args().len() {
        1 => repl(&mut vm),
        2 => {
            let args: Vec<String> = std::env::args().collect();
            let filename = &args[1];
            let source = std::fs::read_to_string(filename).unwrap();
            let _ = eval(&source);
        }
        _ => println!("Usage: arc [path]"),
    }
    vm.free();
}
