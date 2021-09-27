use std::{cell::Ref, sync::Arc};


mod expression;

type int = isize;

struct Vec3 {
    x: int,
    y: int,
    z: int,
}

type Size = Vec3;

enum Ability {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

enum Save {
    Ability(Ability),
    Death,
    Base,
}

enum Skill {
    // Strength,
    Athletics,

    // Dexterity,
    Acrobatics,
    SleightOfHand,
    Stealth,

    // Intelligence,
    Arcana,
    History,
    Investigation,
    Nature,
    Religion,

    // Wisdom,
    AnimalHandling,
    Insight,
    Medicine,
    Perception,
    Survival,

    // Charisma,
    Deception,
    Intimidation,
    Performance,
    Persuasion,
}

enum Check {
    Skill(Skill),
    Ability(Ability),
    Spell,
    Const(int),
}


enum Roll {
    Constant(int),
    Complex { base: Ref<Roll>, modifier: int },
}

// impl Skill {
//     fn base_stat(&self) -> Ability {
//         match self {
//             Skill::Athletics => Ability::Strength,
//             Skill::Acrobatics => Ability::Dexterity,
//             Skill::SleightOfHand => Ability::Dexterity,
//             Skill::Stealth => Ability::Dexterity,
//             Skill::Arcana => Ability::Intelligence,
//             Skill::History => Ability::Intelligence,
//             Skill::Investigation => Ability::Intelligence,
//             Skill::Nature => Ability::Intelligence,
//             Skill::Religion => Ability::Intelligence,
//             Skill::AnimalHandling => Ability::Wisdom,
//             Skill::Insight => Ability::Wisdom,
//             Skill::Medicine => Ability::Wisdom,
//             Skill::Perception => Ability::Wisdom,
//             Skill::Survival => Ability::Wisdom,
//             Skill::Deception => Ability::Charisma,
//             Skill::Intimidation => Ability::Charisma,
//             Skill::Performance => Ability::Charisma,
//             Skill::Persuasion => Ability::Charisma,
//         }

//     }
// }

enum AttackBase {
    Ability(Ability),
    Spell,
    Const(int),
}

enum Range {
    Touch,
    Range(int),
    Sphere(int, int),
    Cone(int),
}

struct Item {
    name: String,
    weight: f64,
}

struct Stats {
    ac: int,
    max_hit_points: int,
}

enum StatusEffect {
    DamageImmunity(DamageKind),
    DamageResistance(DamageKind),
}

type Number = int;
enum DiceAndNumber {
    Dice(Dice),
    Number(Number),
}

enum Modifier {
    Init(DiceAndNumber),
    Atk(DiceAndNumber, Option<Range>),
    Ac(Number, Option<Range>),
    Dmg(DiceAndNumber, Option<Range>),
    Heal(DiceAndNumber),
    AdvCheck(Check),
    AdvDeath,
    DisDeath,
}
enum Conditions {
    Blinded,
    Charmed,
    Deafened,
    Encumbered,
    Frightened,
    Incorporeal,
    Intoxicated,
    Invisible,
    Paralyzed,
    Poisoned,
    Prone,
    Restrained,
    Stable,
    Stunned,
    Turned,
    Unconscious,
}

struct StatBlock {
    stats: Stats,
    attacks: Vec<Attack>,
}

struct Attack {
    name: String,
    description: String,

    range: Range,

    base: AttackBase,
    modifer: int,
    proficient: bool,
    magic_bonus: Option<int>,
    crit_range: Option<int>,

    damge: Vec<DamageDie>,

    save_throw: Ability,
    save_dc: AttackBase,
    save_effect: String,
}

struct Dice {
    sides: int,
    count: int,
}

enum DamageKind {
    Slashing,
    Piercing,
    Bludgeoning,
    Poison,
    Acid,
    Fire,
    Cold,
    Radiant,
    Necrotic,
    Lightning,
    Thunder,
    Force,
    Psychic,
}

struct Token {
    name: String,
    img: String,
    size: Size,
}

struct SavingThrow {}

struct DamageDie {
    die: Dice,
    /// extra die when you crit
    crit: Dice,
    kind: DamageKind,
}

struct Piece {
    name: String,
    token: Box<Token>,
    health: int,
    position: Vec3,
    stats: Stats,
    attacks: Vec<Attack>,
}

enum Action {
    Move(Box<Board>, Vec3),
    SetHealth(int),
    Remove,
}

struct Board {
    parent: Option<Box<Board>>,
    pieces: Vec<Piece>,
}

struct World {
    boards: Vec<Board>,
    actions: Vec<Action>,
}

fn main() {
    println!("Hello, world!");
}
