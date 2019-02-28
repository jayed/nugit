use std::env;
use std::path::Path;


fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    init(&path)
}

fn init(x: &String) -> Result<(, bool)> {
    let path = Path::new(x);
    match path {
        Err(e) => println!("{:?}", e),
        _ => ()
        }
}

#[test]
    fn init() {
       assert_eq!(path.to_str(), Some(".git"));
    }
