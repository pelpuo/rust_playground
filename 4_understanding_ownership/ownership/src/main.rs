fn main() {
    let s1 = String::from("hello");

    let (s1, len) = calculate_length(s1);
    println!("The length of '{s1}' from moving is {len}.");

    let len = calculate_length2(&s1);
    println!("The length of '{s1}' from passing reference is {len}.");

    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("After calling change, s2 becomes {s2}");

    let mut s2 = s1;
    change(&mut s2);
    println!("After calling change, s2 becomes {s2}");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn calculate_length2(s: &String) -> usize {
    let length = s.len(); // len() returns the length of a String

    length
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}