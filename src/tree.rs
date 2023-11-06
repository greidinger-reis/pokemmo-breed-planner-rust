use crate::pokemon::Pokemon;
use phf::phf_map;
use std::collections::HashMap;

/* In Pokemmo, in breeding, you can only breed a pokemon couple once.
 You lose the parents on a breed, and receive the offspring.
 That's why we need a certain number of 31IV'd pokemons, and they are represented here by a, b, c, d, e.
*/
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PokemonBreederKind {
    A,
    B,
    C,
    D,
    E,
    Nature,
}

pub type PokemonBreederKindPositions = HashMap<Position, PokemonBreederKind>;

#[derive(Debug)]
pub struct PokemonBreedTreePosition {
    natured: PokemonBreederKindPositions,
    natureless: PokemonBreederKindPositions,
}

type PokemonBreedTreePositionMap = HashMap<u8, PokemonBreedTreePosition>;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position(u8, u8);

#[derive(Debug)]
pub struct PokemonBreedTree {
    generations: u8,
    pokemon_nodes: HashMap<Position, Pokemon>,
    final_pokemon: Pokemon,
    position_map: PokemonBreedTreePositionMap,
    breed_errors: Vec<Position>,
}

impl PokemonBreedTree {
    pub fn new(final_pokemon: Pokemon, generations: u8) -> PokemonBreedTree {
        let breed_errors = Vec::<Position>::new();
        let pokemon_nodes = HashMap::from([(Position(0, 0), final_pokemon.clone())]);
        let position_map = init_position_map();

        PokemonBreedTree {
            pokemon_nodes,
            generations,
            final_pokemon,
            position_map,
            breed_errors,
        }
    }
}

fn init_position_map() -> PokemonBreedTreePositionMap {
    let position_map = HashMap::<u8, PokemonBreedTreePosition>::from([
        (
            2,
            PokemonBreedTreePosition {
                natured: HashMap::from([
                    (Position(2, 0), PokemonBreederKind::Nature),
                    (Position(2, 1), PokemonBreederKind::A),
                    (Position(2, 2), PokemonBreederKind::A),
                    (Position(2, 3), PokemonBreederKind::B),
                ]),
                natureless: HashMap::from([
                    (Position(1, 0), PokemonBreederKind::A),
                    (Position(1, 1), PokemonBreederKind::B),
                ]),
            },
        ),
        (
            3,
            PokemonBreedTreePosition {
                natured: HashMap::from([
                    (Position(3, 0), PokemonBreederKind::Nature),
                    (Position(3, 1), PokemonBreederKind::A),
                    (Position(3, 2), PokemonBreederKind::A),
                    (Position(3, 3), PokemonBreederKind::B),
                    (Position(3, 4), PokemonBreederKind::A),
                    (Position(3, 5), PokemonBreederKind::B),
                    (Position(3, 6), PokemonBreederKind::A),
                    (Position(3, 7), PokemonBreederKind::B),
                ]),
                natureless: HashMap::from([
                    (Position(2, 0), PokemonBreederKind::A),
                    (Position(2, 1), PokemonBreederKind::B),
                    (Position(2, 2), PokemonBreederKind::A),
                    (Position(2, 3), PokemonBreederKind::C),
                ]),
            },
        ),
        (
            4,
            PokemonBreedTreePosition {
                natured: HashMap::from([
                    (Position(4, 0), PokemonBreederKind::Nature),
                    (Position(4, 1), PokemonBreederKind::A),
                    (Position(4, 2), PokemonBreederKind::A),
                    (Position(4, 3), PokemonBreederKind::B),
                    (Position(4, 4), PokemonBreederKind::A),
                    (Position(4, 5), PokemonBreederKind::B),
                    (Position(4, 6), PokemonBreederKind::A),
                    (Position(4, 7), PokemonBreederKind::C),
                    (Position(4, 8), PokemonBreederKind::A),
                    (Position(4, 9), PokemonBreederKind::B),
                    (Position(4, 10), PokemonBreederKind::A),
                    (Position(4, 11), PokemonBreederKind::C),
                    (Position(4, 12), PokemonBreederKind::B),
                    (Position(4, 13), PokemonBreederKind::C),
                    (Position(4, 14), PokemonBreederKind::B),
                    (Position(4, 15), PokemonBreederKind::D),
                ]),
                natureless: HashMap::from([
                    (Position(3, 0), PokemonBreederKind::A),
                    (Position(3, 1), PokemonBreederKind::B),
                    (Position(3, 2), PokemonBreederKind::A),
                    (Position(3, 3), PokemonBreederKind::C),
                    (Position(3, 4), PokemonBreederKind::B),
                    (Position(3, 5), PokemonBreederKind::C),
                    (Position(3, 6), PokemonBreederKind::B),
                    (Position(3, 7), PokemonBreederKind::D),
                ]),
            },
        ),
        (
            5,
            PokemonBreedTreePosition {
                natured: HashMap::from([
                    (Position(5, 0), PokemonBreederKind::A),
                    (Position(5, 1), PokemonBreederKind::B),
                    (Position(5, 2), PokemonBreederKind::A),
                    (Position(5, 3), PokemonBreederKind::C),
                    (Position(5, 4), PokemonBreederKind::B),
                    (Position(5, 5), PokemonBreederKind::C),
                    (Position(5, 6), PokemonBreederKind::B),
                    (Position(5, 7), PokemonBreederKind::D),
                    (Position(5, 8), PokemonBreederKind::B),
                    (Position(5, 9), PokemonBreederKind::C),
                    (Position(5, 10), PokemonBreederKind::B),
                    (Position(5, 11), PokemonBreederKind::D),
                    (Position(5, 12), PokemonBreederKind::C),
                    (Position(5, 13), PokemonBreederKind::D),
                    (Position(5, 14), PokemonBreederKind::C),
                    (Position(5, 15), PokemonBreederKind::E),
                    (Position(5, 16), PokemonBreederKind::Nature),
                    (Position(5, 17), PokemonBreederKind::B),
                    (Position(5, 18), PokemonBreederKind::B),
                    (Position(5, 19), PokemonBreederKind::C),
                    (Position(5, 20), PokemonBreederKind::B),
                    (Position(5, 21), PokemonBreederKind::C),
                    (Position(5, 22), PokemonBreederKind::B),
                    (Position(5, 23), PokemonBreederKind::D),
                    (Position(5, 24), PokemonBreederKind::B),
                    (Position(5, 25), PokemonBreederKind::C),
                    (Position(5, 26), PokemonBreederKind::B),
                    (Position(5, 27), PokemonBreederKind::D),
                    (Position(5, 28), PokemonBreederKind::C),
                    (Position(5, 29), PokemonBreederKind::D),
                    (Position(5, 30), PokemonBreederKind::C),
                    (Position(5, 31), PokemonBreederKind::E),
                ]),
                natureless: HashMap::from([
                    (Position(4, 0), PokemonBreederKind::A),
                    (Position(4, 1), PokemonBreederKind::B),
                    (Position(4, 2), PokemonBreederKind::A),
                    (Position(4, 3), PokemonBreederKind::C),
                    (Position(4, 4), PokemonBreederKind::B),
                    (Position(4, 5), PokemonBreederKind::C),
                    (Position(4, 6), PokemonBreederKind::B),
                    (Position(4, 7), PokemonBreederKind::D),
                    (Position(4, 8), PokemonBreederKind::B),
                    (Position(4, 9), PokemonBreederKind::C),
                    (Position(4, 10), PokemonBreederKind::B),
                    (Position(4, 11), PokemonBreederKind::D),
                    (Position(4, 12), PokemonBreederKind::C),
                    (Position(4, 13), PokemonBreederKind::D),
                    (Position(4, 14), PokemonBreederKind::C),
                    (Position(4, 15), PokemonBreederKind::E),
                ]),
            },
        ),
    ]);
    position_map
}
