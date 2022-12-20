use mythoji::Emoji;
use mythoji::{Gender, Person, SkinTone};
use strum::IntoEnumIterator;

fn main() {
    println!("mythoji::Person::\n");

    for person in Person::iter() {
        println!("{:<25} = {}", format!("{:?}", person), person);

        for skin in SkinTone::iter() {
            for gender in Gender::iter() {
                println!(
                    "  {:<23} = {}",
                    format!("{:?} + {:?}", skin, gender),
                    Emoji::Person(person, skin, gender)
                );
            }
        }
    }
}
