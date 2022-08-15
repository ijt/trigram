use trigram::similarity;

fn main() {
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() != 1 + 2 {
        eprintln!("usage: similarity string1 string2");
        ::std::process::exit(1);
    }
    let a = args[1].as_str();
    let b = args[2].as_str();
    println!("{}", similarity(a, b));
}
