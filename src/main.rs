mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;


pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    // todo!()
    let mut a : [u8; 4] = [2, 3, 4, 5];
    let _b = a;
    a[0] = 6;
    for x in a.into_iter() {
        println!("{}", x);
    }
    a[1] = 9;
    // println!("fefe{:?}", str::from_utf8(&b).unwrap());
    // println!("{}", b);

    use std::str;

    // some bytes, in a stack-allocated array
    let sparkle_heart : [u8; 4] = [240, 159, 146, 150];

    // We know these bytes are valid, so just use `unwrap()`.
    let sparkle_heart = str::from_utf8(&sparkle_heart).unwrap();

    assert_eq!("💖", sparkle_heart);
    println!("cerer{}", sparkle_heart);
    Ok(())
}