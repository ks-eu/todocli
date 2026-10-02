
struct Cli {
    pattern: String,
    id: i32,
}

fn main() {
    let pattern = std::env::args().nth(1).expect("instruction not given");
    let id = std::env::args().nth(2).expect("id not given");

    let args = Cli {
        pattern: pattern,
        id: id.parse().expect("not a valid integer"),
    };

    println!("instruction: {:?}, id: {:?}", args.pattern, args.id)
}
