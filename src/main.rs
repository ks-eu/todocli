use std::fs::File;
use std::env;

struct Cli {
    pattern: String,
    id: i32,
}

fn main() {
    let path : String = String::from("todo.txt");
    let pattern = env::args().nth(1).expect("instruction not given");
    let id = env::args().nth(2).expect("id not given"); 
 
    let _file = File::create_new(path).expect("file exists");
 
    let args = Cli {
        pattern: pattern,
        id: id.parse().expect("not a valid integer"),
    };

    println!("instruction: {:?}, id: {:?}", args.pattern, args.id)
}
