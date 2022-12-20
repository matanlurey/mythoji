use mythoji::Item;
use strum::IntoEnumIterator;

fn main() {
    println!("mythoji::Item::\n");

    for item in Item::iter() {
        println!("{:<25} = {}", format!("{:?}", item), item);
    }
}
