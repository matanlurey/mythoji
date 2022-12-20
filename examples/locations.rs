use mythoji::Location;
use strum::IntoEnumIterator;

fn main() {
    println!("mythoji::Location::\n");

    for location in Location::iter() {
        println!("{:<25} = {}", format!("{:?}", location), location);
    }
}
