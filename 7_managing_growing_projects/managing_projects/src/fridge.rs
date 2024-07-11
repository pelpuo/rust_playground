pub struct Shelf{
    pub level:i32,
    pub draw:bool,
    pub side:i32,
}

pub fn place(loc: &Shelf){
    let draw_val = true;
    if loc.draw == draw_val{
        println!("Food placed in draw");
    }else{
        println!("Food placed on shelf");
    }

    println!("food is on shelf {}", loc.level);
}
