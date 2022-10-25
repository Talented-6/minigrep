use minigrep::Config;
use std::process;

fn main() {
    // env::args_os() // 处理可能有错误的参数输入
    // let args: Vec<String> = std::env::args().collect();

    let config = Config::new(std::env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing argument: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}
