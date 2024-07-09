fn main() {
    let s = String::from("hello world");

    let slice = &s[0..8];
    
    println!("The slice is \'{slice}\'");

    let word = first_word(&s);
    println!("the first word is: \'{word}\'");

}


fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
