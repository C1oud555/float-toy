use std::env;

fn main() {
    let args = env::args();
    let values: Vec<_> = args
        .skip(1)
        .map(|s| u32::from_str_radix(&s, 16).expect("not a valid hex number"))
        .collect();
    let res: Vec<_> = values.into_iter().map(f32::from_bits).collect();

    println!("{res:?}");
}
