use std::fs::File;

fn main() {
    let greeting_file = match File::open("hello123.txt"){
        Ok(result) => result,
        Err(e) => panic!("{}", e)
    };
}