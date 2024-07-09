fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("The value of x is: {x}");
	println!("The value of y is: {y}");
	println!("The value of z is: {z}\n");

    println!("The value of tup.0 is: {five_hundred}");
    println!("The value of tup.1 is: {six_point_four}");
    println!("The value of tup.2 is: {one}");

}
