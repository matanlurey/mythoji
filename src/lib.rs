//! A collection of all emojis that might be used in a fantasy text-based game.
//!
//! # Examples
//!
//! ```
//! use mythoji::{Emoji, Gender, Person, Location, SkinTone};
//!
//! let castle = Location::Castle;
//! assert_eq!(castle.to_string(), "🏰");
//!
//! let female_elf = Emoji::Person(Person::Elf, SkinTone::Neutral, Gender::Female);
//! assert_eq!(female_elf.to_string(), "🧝‍♀️");
//! ```
//!
//! # Limitations
//!
//! Not all terminals support all emoji combinations, and it's non-trivial to detect support without
//! additional crates. It's recommended to try the examples, and be prepared to fall back to a less
//! fancy representation if the specific emojis don't work.
//!
//! # Features
//!
//! - `iter`: Enables the `EnumIter` derive macro for all enums. _Disabled_ by default.

use std::fmt::{Display, Formatter, Result};

#[cfg(feature = "iter")]
use strum_macros::EnumIter;

/// A collection of all emojis that might be used in a fantasy text-based game.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Emoji {
    /// Contains all person emojis that can be used with different genders and skin tones.
    Person(Person, SkinTone, Gender),

    /// Contains all other living emojis that don't fit in [`Person`].
    Creature(Creature),

    /// Contains all location emojis.
    Location(Location),

    /// Contains all item emojis.
    Item(Item),
}

impl Default for Emoji {
    fn default() -> Self {
        Self::Person(Default::default(), Default::default(), Default::default())
    }
}

impl Display for Emoji {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Emoji::Person(person, skin, gender) => {
                const ZWJ: char = '\u{200d}';
                const VARIATION_SELECTOR_16: char = '\u{fe0f}';

                let mut buffer = person.to_string();

                if gender != &Gender::Neutral {
                    buffer.push(ZWJ);
                    buffer.push_str(&gender.to_string());
                }
                if skin != &SkinTone::Neutral {
                    buffer.push(ZWJ);
                    buffer.push_str(&skin.to_string());
                }
                if gender != &Gender::Neutral || skin != &SkinTone::Neutral {
                    buffer.push(VARIATION_SELECTOR_16);
                }

                write!(f, "{}", buffer)?;
            }
            Emoji::Creature(creature) => write!(f, "{}", creature)?,
            Emoji::Location(location) => write!(f, "{}", location)?,
            Emoji::Item(item) => write!(f, "{}", item)?,
        };
        Ok(())
    }
}

/// Emojis that can be used with different genders and skin tones.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "iter", derive(EnumIter))]
pub enum Person {
    /// An artist, e.g. "👩‍🎨".
    Artist,

    /// A baby, e.g. "👶".
    Baby,

    /// A bald person, e.g. "🧑‍🦲".
    BaldPerson,

    /// A person with a beard, e.g. "🧔".
    BeardedPerson,

    /// A child, e.g. "🧒".
    Child,

    /// A fairy, e.g. "🧚".
    Fairy,

    /// An elf, e.g. "🧝".
    Elf,

    /// A genie, e.g. "🧞".
    Genie,

    /// A person with a head scarf, e.g. "🧕".
    HeardScarfPerson,

    /// A mage, e.g. "🧙".
    Mage,

    /// A mer-person, e.g. "🧜".
    MerPerson,

    /// An old person, e.g. "🧓".
    OldPerson,

    /// A person, e.g. "🧑".
    #[default]
    Person,

    /// A person of royalty, e.g. "🤴".
    Royalty,

    /// A person with a skull cap, e.g. "👲".
    SkullCapPerson,

    /// A person with a turban, e.g. "👳".
    TurbanPerson,

    /// A vampire, e.g. "🧛".
    Vampire,

    /// A zombie, e.g. "🧟".
    Zombie,
}

impl Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                Self::Artist => "🧑‍🎨",
                Self::Baby => "👶",
                Self::BaldPerson => "🧑‍🦲",
                Self::BeardedPerson => "🧔",
                Self::Child => "🧒",
                Self::Elf => "🧝",
                Self::Fairy => "🧚",
                Self::Genie => "🧞",
                Self::HeardScarfPerson => "🧕",
                Self::Mage => "🧙",
                Self::MerPerson => "🧜",
                Self::OldPerson => "🧓",
                Self::Person => "🧑",
                Self::Royalty => "🤴",
                Self::SkullCapPerson => "👲",
                Self::TurbanPerson => "👳",
                Self::Vampire => "🧛",
                Self::Zombie => "🧟",
            }
        )
    }
}

/// Skin tones that can be used with certain emojis.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "iter", derive(EnumIter))]
pub enum SkinTone {
    /// Makes a skin toned emoji appear with a neutral skin tone, which is often "Simpsons yellow".
    #[default]
    Neutral,

    /// Makes a skin toned emoji appear with a light skin tone.
    Light,

    /// Makes a skin toned emoji appear with a medium light skin tone.
    MediumLight,

    /// Makes a skin toned emoji appear with a medium skin tone.
    Medium,

    /// Makes a skin toned emoji appear with a medium dark skin tone.
    MediumDark,

    /// Makes a skin toned emoji appear with a dark skin tone.
    Dark,
}

impl Display for SkinTone {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                Self::Neutral => "",
                Self::Light => "🏻",
                Self::MediumLight => "🏼",
                Self::Medium => "🏽",
                Self::MediumDark => "🏾",
                Self::Dark => "🏿",
            }
        )
    }
}

/// Genders that can be used with certain emojis.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "iter", derive(EnumIter))]
pub enum Gender {
    /// Makes a
    #[default]
    Neutral,

    /// Makes a gendered emoji appear male.
    Male,

    /// Makes a gendered emoji appear female.
    Female,
}

impl Display for Gender {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                Gender::Neutral => "",
                Gender::Male => "♂",
                Gender::Female => "♀",
            }
        )
    }
}

/// Emojis that can be used to represent a creature.
///
/// **NOTE**: All emojis are meant to represent the side view, not face, of the creature, _if_ able.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "iter", derive(EnumIter))]
pub enum Creature {
    /// An ant, e.g. "🐜".
    #[default]
    Ant,

    /// A bat, e.g. "🦇".
    Bat,

    /// A beetle, e.g. "🐞".
    Beetle,

    /// A bison, e.g. "🦬".
    Bison,

    /// A boar, e.g. "🐗".
    Boar,

    /// A bug, e.g. "🐛".
    Bug,

    /// A butterfly, e.g. "🦋".
    Butterfly,

    /// A camel, e.g. "🐫".
    Camel,

    /// A cat, e.g. "🐈".
    Cat,

    /// A cockroach, e.g. "🪳".
    Cockroach,

    /// A cow, e.g. "🐄".
    Cow,

    /// A crab, e.g. "🦀".
    Crab,

    /// A crocodile, e.g. "🐊".
    Crocodile,

    /// A deer, e.g. "🦌".
    Deer,

    /// A dog, e.g. "🐕".
    Dog,

    /// A dragon, e.g. "🐉".
    Dragon,

    /// An eagle, e.g. "🦅".
    Eagle,

    /// An elephant, e.g. "🐘".
    Elephant,

    /// A fish, e.g. "🐟".
    Fish,

    /// A ghost, e.g. "👻".
    Ghost,

    /// A goat, e.g. "🐐".
    Goat,

    /// A goblin, e.g. "👺".
    Goblin,

    /// A honeybee, e.g. "🐝".
    Honeybee,

    /// A horse, e.g. "🐎".
    Horse,

    /// A leopard, e.g. "🐆".
    Leopard,

    /// A llama, e.g. "🦙".
    Llama,

    /// A mammoth, e.g. "🦣".
    Mammoth,

    /// A mouse, e.g. "🐁".
    Mouse,

    /// An ogre, e.g. "👹".
    Ogre,

    /// A pig, e.g. "🐖".
    Pig,

    /// A rabbit, e.g. "🐇".
    Rabbit,

    /// A ram, e.g. "🐏".
    Ram,

    /// A rat, e.g. "🐀".
    Rat,

    /// A rhinoceros, e.g. "🦏".
    Rhinoceros,

    /// A scorpion, e.g. "🦂".
    Scorpion,

    /// A shark, e.g. "🦈".
    Shark,

    /// A snake, e.g. "🐍".
    Snake,

    /// A spider, e.g. "🕷".
    Spider,

    /// A tiger, e.g. "🐅".
    Tiger,

    /// A tropical fish, e.g. "🐠".
    TropicalFish,

    /// A water buffalo, e.g. "🐃".
    WaterBuffalo,

    /// A wolf, e.g. "🐺".
    Wolf,
}

impl Display for Creature {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                Self::Ant => "🐜",
                Self::Bat => "🦇",
                Self::Beetle => "🐞",
                Self::Bison => "🦬",
                Self::Boar => "🐗",
                Self::Bug => "🐛",
                Self::Butterfly => "🦋",
                Self::Camel => "🐫",
                Self::Cat => "🐈",
                Self::Cockroach => "🪳",
                Self::Cow => "🐄",
                Self::Crab => "🦀",
                Self::Crocodile => "🐊",
                Self::Deer => "🦌",
                Self::Dog => "🐕",
                Self::Dragon => "🐉",
                Self::Eagle => "🦅",
                Self::Elephant => "🐘",
                Self::Fish => "🐟",
                Self::Ghost => "👻",
                Self::Goat => "🐐",
                Self::Goblin => "👺",
                Self::Honeybee => "🐝",
                Self::Horse => "🐎",
                Self::Leopard => "🐆",
                Self::Llama => "🦙",
                Self::Mammoth => "🦣",
                Self::Mouse => "🐁",
                Self::Ogre => "👹",
                Self::Pig => "🐖",
                Self::Rabbit => "🐇",
                Self::Ram => "🐏",
                Self::Rat => "🐀",
                Self::Rhinoceros => "🦏",
                Self::Scorpion => "🦂",
                Self::Shark => "🦈",
                Self::Snake => "🐍",
                Self::Spider => "🕷",
                Self::Tiger => "🐅",
                Self::TropicalFish => "🐠",
                Self::WaterBuffalo => "🐃",
                Self::Wolf => "🐺",
            }
        )
    }
}

/// Emojis that can be used to represent a location.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "iter", derive(EnumIter))]
pub enum Location {
    /// A sailboat, e.g. "⛵".
    BoatSail,

    /// A classic building, e.g. "🏛".
    BuildingClassic,

    /// A campsite, e.g. "🏕".
    Campsite,

    /// A canoe, e.g. "🛶".
    Canoe,

    /// A castle, e.g. "🏰".
    Castle,

    /// A Japanese-style castle, e.g. "🏯".
    CastleJapanese,

    /// A cave, e.g. "🕳".
    Cave,

    /// A desert, e.g. "🏜".
    Desert,

    /// A hut, e.g. "🛖".
    Hut,

    /// A mountain, e.g. "⛰".
    Mountain,

    /// A mountain in the snow, e.g. "🏔".
    MountainSnow,

    /// An oasis, e.g. "🏜".
    Oasis,

    /// A palace, e.g. "🏯".
    Palace,

    /// A tent, e.g. "⛺".
    Tent,

    /// A deciduous tree, e.g. "🌳".
    TreeDeciduous,

    /// An evergreen tree, e.g. "🌲".
    TreeEvergreen,

    /// A palm tree, e.g. "🌴".
    TreePalm,

    /// A volcano, e.g. "🌋".
    Volcano,
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                Self::BoatSail => "⛵",
                Self::BuildingClassic => "🏛",
                Self::Campsite => "🏕",
                Self::Canoe => "🛶",
                Self::Castle => "🏰",
                Self::CastleJapanese => "🏯",
                Self::Cave => "🕳",
                Self::Desert => "🏜",
                Self::Hut => "🛖",
                Self::Mountain => "⛰",
                Self::MountainSnow => "🏔",
                Self::Oasis => "🏜",
                Self::Palace => "🏯",
                Self::Tent => "⛺",
                Self::TreeDeciduous => "🌳",
                Self::TreeEvergreen => "🌲",
                Self::TreePalm => "🌴",
                Self::Volcano => "🌋",
            }
        )
    }
}

/// Emojis that can be used to represent an item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "iter", derive(EnumIter))]
pub enum Item {
    /// An amulet, e.g. "🧿".
    Amulet,

    /// An axe, e.g. "🪓".
    Axe,

    /// A bag, e.g. "🎒".
    Bag,

    /// A bandage, e.g. "🩹".
    Bandage,

    /// A bed, e.g. "🛏".
    Bed,

    /// A beer, e.g. "🍺".
    Beer,

    /// A drop of blood, e.g. "🩸".
    BloodDrop,

    /// A bomb, e.g. "💣".
    Bomb,

    /// A closed book, e.g. "📕".
    BookClosed,

    /// An open book, e.g. "📖".
    BookOpen,

    /// A boomerang, e.g. "🪃".
    Boomerang,

    /// A bow and arrow, e.g. "🏹".
    BowAndArrow,

    /// A brick, e.g. "🧱".
    Brick,

    /// A candle, e.g. "🕯".
    Candle,

    /// A coat, e.g. "🧥".
    Coat,

    /// A coffin, e.g. "⚰️".
    Coffin,

    /// A coin, e.g. "🪙".
    Coin,

    /// A crown, e.g. "👑".
    Crown,

    /// A crystal ball, e.g. "🔮".
    CrystalBall,

    /// A dagger, e.g. "🗡".
    Dagger,

    /// A dart, e.g. "🎯".
    Dart,

    /// A door, e.g. "🚪".
    Door,

    /// A black flag, e.g. "🏴".
    FlagBlack,

    /// A triangular flag, e.g. "🚩".
    FlagTriangle,

    /// A firecracker, e.g. "🧨".
    Firecracker,

    /// A gemstone, e.g. "💎".
    GemStone,

    /// A grave, e.g. "🪦".
    Grave,

    /// A hammer, e.g. "🔨".
    Hammer,

    /// A hammer and pick, e.g. "⚒️".
    HammerAndPick,

    /// A red heart, e.g. "❤️".
    HeartRed,

    /// A hourglass that is done, e.g. "⌛".
    HourglassDone,

    /// A hourglass that is not done, e.g. "⏳".
    HourglassNotDone,

    /// A jar, e.g. "🏺".
    Jar,

    /// A key, e.g. "🗝️".
    Key,

    /// A leaf, e.g. "🍃".
    Leaf,

    /// A fallen leaf, e.g. "🍂".
    LeafFallen,

    /// A maple leaf, e.g. "🍁".
    LeafMaple,

    /// A map, e.g. "🗺".
    Map,

    /// A meat on a bone, e.g. "🍖".
    MeatOnBone,

    /// A cut of meat, e.g. "🥩".
    MeatCut,

    /// A pickaxe, e.g. "⛏".
    Pick,

    /// A poultry leg, e.g. "🍗".
    PoultryLeg,

    /// Prayer beads, e.g. "📿".
    PrayerBeads,

    /// A red envelope, e.g. "🧧".
    RedEnvelope,

    /// A red lantern, e.g. "🏮".
    RedLantern,

    /// A rock, e.g. "🪨".
    Rock,

    /// A scroll, e.g. "📜".
    Scroll,

    /// A shield, e.g. "🛡".
    Shield,

    /// Swords crossed, e.g. "⚔️".
    SwordsCrossed,

    /// A trident, e.g. "🔱".
    Trident,

    /// An urn, e.g. "⚱️".
    Urn,

    /// A wand, e.g. "🪄".
    Wand,

    /// A water drop, e.g. "💧".
    WaterDrop,
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                Self::Amulet => "🧿",
                Self::Axe => "🪓",
                Self::Bag => "🎒",
                Self::Bandage => "🩹",
                Self::Bed => "🛏",
                Self::Beer => "🍺",
                Self::BloodDrop => "🩸",
                Self::Bomb => "💣",
                Self::BookClosed => "📕",
                Self::BookOpen => "📖",
                Self::Boomerang => "🪃",
                Self::BowAndArrow => "🏹",
                Self::Brick => "🧱",
                Self::Candle => "🕯",
                Self::Coat => "🧥",
                Self::Coffin => "⚰️",
                Self::Coin => "🪙",
                Self::Crown => "👑",
                Self::CrystalBall => "🔮",
                Self::Dagger => "🗡",
                Self::Dart => "🎯",
                Self::Door => "🚪",
                Self::FlagBlack => "🏴",
                Self::FlagTriangle => "🚩",
                Self::Firecracker => "🧨",
                Self::GemStone => "💎",
                Self::Grave => "🪦",
                Self::Hammer => "🔨",
                Self::HammerAndPick => "⚒️",
                Self::HeartRed => "❤️",
                Self::HourglassDone => "⌛",
                Self::HourglassNotDone => "⏳",
                Self::Jar => "🏺",
                Self::Key => "🗝️",
                Self::Leaf => "🍃",
                Self::LeafFallen => "🍂",
                Self::LeafMaple => "🍁",
                Self::Map => "🗺",
                Self::MeatOnBone => "🍖",
                Self::MeatCut => "🥩",
                Self::Pick => "⛏",
                Self::PoultryLeg => "🍗",
                Self::PrayerBeads => "📿",
                Self::RedEnvelope => "🧧",
                Self::RedLantern => "🏮",
                Self::Rock => "🪨",
                Self::Scroll => "📜",
                Self::Shield => "🛡",
                Self::SwordsCrossed => "⚔️",
                Self::Trident => "🔱",
                Self::Urn => "⚱️",
                Self::Wand => "🪄",
                Self::WaterDrop => "💧",
            }
        )
    }
}

/// Emojis that can be used to represent a symbol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "iter", derive(EnumIter))]
pub enum Symbol {
    /// A symbol of anger, e.g. "💢".
    Anger,

    /// A symbol of a comet, e.g. "☄️".
    Comet,

    /// A symbol of a cyclone, e.g. "🌀".
    Cyclone,

    /// A symbol of fire, e.g. "🔥".
    Fire,

    /// A symbol of electricity, e.g. "⚡".
    Electricity,

    /// A symbol of two exclamations, e.g. "‼️".
    ExclamationDouble,

    /// A symbol of an exclamation and a question mark, e.g. "⁉️".
    ExclamationWithQuestion,

    /// A symbol of a red exclamation, e.g. "❗".
    ExclamationRed,

    /// A symbol of a white exclamation, e.g. "❕".
    ExclamationWhite,

    /// A symbol of a female, e.g. "♀️".
    GenderFemale,

    /// A symbol of a male, e.g. "♂️".
    GenderMale,

    /// A symbol of a red question, e.g. "❓".
    QuestionRed,

    /// A symbol of a white question, e.g. "❔".
    QuestionWhite,

    /// A symbol of sparkles, e.g. "✨".
    Sparkles,

    /// A speech bubble, e.g. "💬".
    SpeechBubble,

    /// A speech bubble with an angry face, e.g. "🗯️".
    SpeechBubbleAngry,

    /// A snowflake, e.g. "❄️".
    Snowflake,

    /// A "zzz" symbol, e.g. "💤".
    Zzz,
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                Self::Anger => "💢",
                Self::Comet => "☄️",
                Self::Cyclone => "🌀",
                Self::Fire => "🔥",
                Self::Electricity => "⚡",
                Self::ExclamationDouble => "‼️",
                Self::ExclamationWithQuestion => "⁉️",
                Self::ExclamationRed => "❗",
                Self::ExclamationWhite => "❕",
                Self::GenderFemale => "♀️",
                Self::GenderMale => "♂️",
                Self::QuestionRed => "❓",
                Self::QuestionWhite => "❔",
                Self::Sparkles => "✨",
                Self::SpeechBubble => "💬",
                Self::SpeechBubbleAngry => "🗯️",
                Self::Snowflake => "❄️",
                Self::Zzz => "💤",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item() {
        assert_eq!(Item::Amulet.to_string(), "🧿");
        assert_eq!(Item::Axe.to_string(), "🪓");
        assert_eq!(Item::Bag.to_string(), "🎒");
        assert_eq!(Item::Bandage.to_string(), "🩹");
        assert_eq!(Item::Bed.to_string(), "🛏");
        assert_eq!(Item::Beer.to_string(), "🍺");
        assert_eq!(Item::BloodDrop.to_string(), "🩸");
        assert_eq!(Item::Bomb.to_string(), "💣");
        assert_eq!(Item::BookClosed.to_string(), "📕");
        assert_eq!(Item::BookOpen.to_string(), "📖");
        assert_eq!(Item::Boomerang.to_string(), "🪃");
        assert_eq!(Item::BowAndArrow.to_string(), "🏹");
        assert_eq!(Item::Brick.to_string(), "🧱");
        assert_eq!(Item::Candle.to_string(), "🕯");
        assert_eq!(Item::Coat.to_string(), "🧥");
        assert_eq!(Item::Coffin.to_string(), "⚰️");
        assert_eq!(Item::Coin.to_string(), "🪙");
        assert_eq!(Item::Crown.to_string(), "👑");
        assert_eq!(Item::CrystalBall.to_string(), "🔮");
        assert_eq!(Item::Dagger.to_string(), "🗡");
        assert_eq!(Item::Dart.to_string(), "🎯");
        assert_eq!(Item::Door.to_string(), "🚪");
        assert_eq!(Item::FlagBlack.to_string(), "🏴");
        assert_eq!(Item::FlagTriangle.to_string(), "🚩");
        assert_eq!(Item::Firecracker.to_string(), "🧨");
        assert_eq!(Item::GemStone.to_string(), "💎");
        assert_eq!(Item::Grave.to_string(), "🪦");
        assert_eq!(Item::Hammer.to_string(), "🔨");
        assert_eq!(Item::HammerAndPick.to_string(), "⚒️");
        assert_eq!(Item::HeartRed.to_string(), "❤️");
        assert_eq!(Item::HourglassDone.to_string(), "⌛");
        assert_eq!(Item::HourglassNotDone.to_string(), "⏳");
        assert_eq!(Item::Jar.to_string(), "🏺");
        assert_eq!(Item::Key.to_string(), "🗝️");
        assert_eq!(Item::Leaf.to_string(), "🍃");
        assert_eq!(Item::LeafFallen.to_string(), "🍂");
        assert_eq!(Item::LeafMaple.to_string(), "🍁");
        assert_eq!(Item::Map.to_string(), "🗺");
        assert_eq!(Item::MeatOnBone.to_string(), "🍖");
        assert_eq!(Item::MeatCut.to_string(), "🥩");
        assert_eq!(Item::Pick.to_string(), "⛏");
        assert_eq!(Item::PoultryLeg.to_string(), "🍗");
        assert_eq!(Item::PrayerBeads.to_string(), "📿");
        assert_eq!(Item::RedEnvelope.to_string(), "🧧");
        assert_eq!(Item::RedLantern.to_string(), "🏮");
        assert_eq!(Item::Rock.to_string(), "🪨");
        assert_eq!(Item::Scroll.to_string(), "📜");
        assert_eq!(Item::Shield.to_string(), "🛡");
        assert_eq!(Item::SwordsCrossed.to_string(), "⚔️");
        assert_eq!(Item::Trident.to_string(), "🔱");
        assert_eq!(Item::Urn.to_string(), "⚱️");
        assert_eq!(Item::Wand.to_string(), "🪄");
        assert_eq!(Item::WaterDrop.to_string(), "💧");
    }

    #[test]
    fn test_symbol() {
        assert_eq!(Symbol::Anger.to_string(), "💢");
        assert_eq!(Symbol::Comet.to_string(), "☄️");
        assert_eq!(Symbol::Cyclone.to_string(), "🌀");
        assert_eq!(Symbol::Fire.to_string(), "🔥");
        assert_eq!(Symbol::Electricity.to_string(), "⚡");
        assert_eq!(Symbol::ExclamationDouble.to_string(), "‼️");
        assert_eq!(Symbol::ExclamationWithQuestion.to_string(), "⁉️");
        assert_eq!(Symbol::ExclamationRed.to_string(), "❗");
        assert_eq!(Symbol::ExclamationWhite.to_string(), "❕");
        assert_eq!(Symbol::GenderFemale.to_string(), "♀️");
        assert_eq!(Symbol::GenderMale.to_string(), "♂️");
        assert_eq!(Symbol::QuestionRed.to_string(), "❓");
        assert_eq!(Symbol::QuestionWhite.to_string(), "❔");
        assert_eq!(Symbol::Sparkles.to_string(), "✨");
        assert_eq!(Symbol::SpeechBubble.to_string(), "💬");
        assert_eq!(Symbol::SpeechBubbleAngry.to_string(), "🗯️");
        assert_eq!(Symbol::Snowflake.to_string(), "❄️");
        assert_eq!(Symbol::Zzz.to_string(), "💤");
    }

    #[test]
    fn test_location() {
        assert_eq!(Location::BoatSail.to_string(), "⛵");
        assert_eq!(Location::BuildingClassic.to_string(), "🏛");
        assert_eq!(Location::Campsite.to_string(), "🏕");
        assert_eq!(Location::Canoe.to_string(), "🛶");
        assert_eq!(Location::Castle.to_string(), "🏰");
        assert_eq!(Location::CastleJapanese.to_string(), "🏯");
        assert_eq!(Location::Cave.to_string(), "🕳");
        assert_eq!(Location::Desert.to_string(), "🏜");
        assert_eq!(Location::Hut.to_string(), "🛖");
        assert_eq!(Location::Mountain.to_string(), "⛰");
        assert_eq!(Location::MountainSnow.to_string(), "🏔");
        assert_eq!(Location::Oasis.to_string(), "🏜");
        assert_eq!(Location::Palace.to_string(), "🏯");
        assert_eq!(Location::Tent.to_string(), "⛺");
        assert_eq!(Location::TreeDeciduous.to_string(), "🌳");
        assert_eq!(Location::TreeEvergreen.to_string(), "🌲");
        assert_eq!(Location::TreePalm.to_string(), "🌴");
        assert_eq!(Location::Volcano.to_string(), "🌋");
    }

    #[test]
    fn test_person() {
        assert_eq!(Person::Artist.to_string(), "🧑‍🎨");
        assert_eq!(Person::Baby.to_string(), "👶");
        assert_eq!(Person::BaldPerson.to_string(), "🧑‍🦲");
        assert_eq!(Person::BeardedPerson.to_string(), "🧔");
        assert_eq!(Person::Child.to_string(), "🧒");
        assert_eq!(Person::Elf.to_string(), "🧝");
        assert_eq!(Person::Fairy.to_string(), "🧚");
        assert_eq!(Person::Genie.to_string(), "🧞");
        assert_eq!(Person::HeardScarfPerson.to_string(), "🧕");
        assert_eq!(Person::Mage.to_string(), "🧙");
        assert_eq!(Person::MerPerson.to_string(), "🧜");
        assert_eq!(Person::OldPerson.to_string(), "🧓");
        assert_eq!(Person::Person.to_string(), "🧑");
        assert_eq!(Person::Royalty.to_string(), "🤴");
        assert_eq!(Person::SkullCapPerson.to_string(), "👲");
        assert_eq!(Person::TurbanPerson.to_string(), "👳");
        assert_eq!(Person::Vampire.to_string(), "🧛");
        assert_eq!(Person::Zombie.to_string(), "🧟");
    }

    #[test]
    fn test_creature() {
        assert_eq!(Creature::Ant.to_string(), "🐜");
        assert_eq!(Creature::Bat.to_string(), "🦇");
        assert_eq!(Creature::Beetle.to_string(), "🐞");
        assert_eq!(Creature::Bison.to_string(), "🦬");
        assert_eq!(Creature::Boar.to_string(), "🐗");
        assert_eq!(Creature::Bug.to_string(), "🐛");
        assert_eq!(Creature::Butterfly.to_string(), "🦋");
        assert_eq!(Creature::Camel.to_string(), "🐫");
        assert_eq!(Creature::Cat.to_string(), "🐈");
        assert_eq!(Creature::Cockroach.to_string(), "🪳");
        assert_eq!(Creature::Cow.to_string(), "🐄");
        assert_eq!(Creature::Crab.to_string(), "🦀");
        assert_eq!(Creature::Crocodile.to_string(), "🐊");
        assert_eq!(Creature::Deer.to_string(), "🦌");
        assert_eq!(Creature::Dog.to_string(), "🐕");
        assert_eq!(Creature::Dragon.to_string(), "🐉");
        assert_eq!(Creature::Eagle.to_string(), "🦅");
        assert_eq!(Creature::Elephant.to_string(), "🐘");
        assert_eq!(Creature::Fish.to_string(), "🐟");
        assert_eq!(Creature::Ghost.to_string(), "👻");
        assert_eq!(Creature::Goat.to_string(), "🐐");
        assert_eq!(Creature::Goblin.to_string(), "👺");
        assert_eq!(Creature::Honeybee.to_string(), "🐝");
        assert_eq!(Creature::Horse.to_string(), "🐎");
        assert_eq!(Creature::Leopard.to_string(), "🐆");
        assert_eq!(Creature::Llama.to_string(), "🦙");
        assert_eq!(Creature::Mammoth.to_string(), "🦣");
        assert_eq!(Creature::Mouse.to_string(), "🐁");
        assert_eq!(Creature::Ogre.to_string(), "👹");
        assert_eq!(Creature::Pig.to_string(), "🐖");
        assert_eq!(Creature::Rabbit.to_string(), "🐇");
        assert_eq!(Creature::Ram.to_string(), "🐏");
        assert_eq!(Creature::Rat.to_string(), "🐀");
        assert_eq!(Creature::Rhinoceros.to_string(), "🦏");
        assert_eq!(Creature::Scorpion.to_string(), "🦂");
        assert_eq!(Creature::Shark.to_string(), "🦈");
        assert_eq!(Creature::Snake.to_string(), "🐍");
        assert_eq!(Creature::Spider.to_string(), "🕷");
        assert_eq!(Creature::Tiger.to_string(), "🐅");
        assert_eq!(Creature::TropicalFish.to_string(), "🐠");
        assert_eq!(Creature::WaterBuffalo.to_string(), "🐃");
        assert_eq!(Creature::Wolf.to_string(), "🐺");
    }
}
