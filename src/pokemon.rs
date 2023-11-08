#![allow(unused, dead_code)]
#[derive(Debug, Clone,PartialEq,Eq)]
pub enum PokemonType {
    Fire,
    Water,
    Grass,
    Electric,
    Flying,
    Normal,
    Bug,
    Poison,
    Ground,
    Rock,
    Fighting,
    Psychic,
    Ghost,
    Ice,
    Dragon,
    Dark,
    Steel,
}

#[derive(Debug, Copy, Clone,PartialEq, Eq)]
pub enum PokemonNature {
    Hardy,
    Lonely,
    Brave,
    Adamant,
    Naughty,
    Bold,
    Docile,
    Relaxed,
    Impish,
    Lax,
    Timid,
    Hasty,
    Serious,
    Jolly,
    Naive,
    Modest,
    Mild,
    Quiet,
    Bashful,
    Rash,
    Calm,
    Gentle,
    Sassy,
    Careful,
    Quirky,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum PokemonEggGroup {
    Monster,
    WaterA,
    WaterB,
    WaterC,
    Bug,
    Flying,
    Field,
    Fairy,
    Plant,
    Humanoid,
    Mineral,
    Chaos,
    Ditto,
    Dragon,
    CannotBreed,
    Genderless,
}

#[derive(Debug, Clone,PartialEq, Eq)]
pub enum PokemonIv {
    HP,
    Attack,
    Defense,
    SpecialAttack,
    SpecialDefense,
    Speed,
}

#[derive(Debug, Clone,PartialEq, Eq)]
pub enum PokemonGender {
    Female,
    Male,
    Genderless,
}

#[derive(Debug,PartialEq, Eq, Clone)]
pub struct Pokemon {
    pub number: u16,
    pub name: String,
    pub types: (PokemonType, Option<PokemonType>),
    pub egg_groups: (PokemonEggGroup, Option<PokemonEggGroup>),
    pub percentage_male: String,
}

impl Pokemon {
    pub fn egg_groups_include(&self, egg_group: &PokemonEggGroup) -> bool {
        match self.egg_groups.1 {
            Some(ref egg_group2) => self.egg_groups.0 == *egg_group || *egg_group2 == *egg_group,
            None => self.egg_groups.0 == *egg_group,
        }
    }

    pub fn percentage_male_as_f32(&self) -> f32 {
        self.percentage_male.parse::<f32>().expect("This shouldn't happen")
    }
}
