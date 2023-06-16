use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};
use std::collections::HashMap;


fn main() {
    let apple_map = HashMap::from([
        ("Red Delicious", 2),
        ("Kawana Apple", 5),
    ]);
    println!("The total amount of apples that everyone has is {:?}", apple_map);
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}