use kanaria::str::UCSStr;
use std::env;
use std::fs;
use std::io::Write;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() < 2 {
        exit(0);
    }
    let target = &args[1];
    match fs::read_to_string(target) {
        Ok(content) => {
            let res = UCSStr::from_str(&content).wide().to_string();

            let mut f = fs::File::create("output.txt").unwrap();
            let _ = f.write_all(res.as_bytes());
        }
        Err(err) => {
            eprintln!("Error reading file: {}", err);
        }
    }
}
