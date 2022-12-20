use mythoji::Symbol;
use strum::IntoEnumIterator;

fn main() {
    println!("mythoji::Symbol::\n");

    for symbol in Symbol::iter() {
        println!("{:<25} = {}", format!("{:?}", symbol), symbol);
    }
}
