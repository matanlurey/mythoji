use mythoji::Creature;
use strum::IntoEnumIterator;

fn main() {
    println!("mythoji::Creature::\n");

    for creature in Creature::iter() {
        println!("{:<25} = {}", format!("{:?}", creature), creature);
    }
}
