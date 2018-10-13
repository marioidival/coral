use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() == 2 {
        // execute the file
        println!("{:?}", args)
    } else {
        // open REPL
        println!("Hello, world!")
    }
}
