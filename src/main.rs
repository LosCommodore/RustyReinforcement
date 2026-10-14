mod lake;
use anyhow::Result;
use lake::Environment;
use lake::Lake;

const X: bool = true;
const O: bool = false;

fn main() -> Result<()> {
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

    let mut lake = Lake::new(&map, 4)?;
    let _obs = lake.reset();

    Ok(())
}
