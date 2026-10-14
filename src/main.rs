mod lake;
use lake::Lake;

const X: bool = true;
const O: bool = false;

fn main() {
    println!("Hello, world!");

    #[rustfmt::skip]
    let map: Vec<_> = [
        [O, O, O, O],
        [O, X, O, X], 
        [O, O, O, X], 
        [X, O, O, O]
        ]
        .iter()
        .flatten()
        .cloned()
        .collect();

    let lake = Lake::new(&map, 4);
}
