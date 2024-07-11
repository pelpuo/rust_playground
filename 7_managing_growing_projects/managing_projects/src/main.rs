// use managing_projects::fridge;
pub mod fridge;

fn main() {
    let my_shelf = fridge::Shelf{level: 1, draw: true, side: 1};

    fridge::place(&my_shelf);
}
