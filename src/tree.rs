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
}

type PokemonBreederQuantity = HashMap<PokemonBreederKind, u8>;

#[derive(Debug, Clone)]
struct BreedTreeFormation {
    natured: PokemonBreederQuantity,
    natureless: PokemonBreederQuantity,
}

//pub struct PokemonBreedTreeFormationPositions {
//natured:
//}

//pub const BREED_TREE_FORMATION_IVS_INFO : phf::Map<&'static str,

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position(u8, u8);

#[derive(Debug)]
pub struct PokemonBreedTree {
    generations: u8,
    nodes: HashMap<Position, Pokemon>,
    final_pokemon: Pokemon,
    position_map: HashMap<u8, BreedTreeFormation>,
}

impl PokemonBreedTree {
    pub fn new(final_pokemon: Pokemon, generations: u8) -> PokemonBreedTree {
        let nodes = vec![(Position(0, 0), final_pokemon.clone())]
            .into_iter()
            .collect::<HashMap<Position, Pokemon>>();

        let position_map = HashMap::from([
            (
                2,
                BreedTreeFormation {
                    natured: HashMap::from([
                        (PokemonBreederKind::A, 2),
                        (PokemonBreederKind::B, 1),
                        (PokemonBreederKind::C, 0),
                        (PokemonBreederKind::D, 0),
                        (PokemonBreederKind::E, 0),
                    ]),
                    natureless: HashMap::from([
                        (PokemonBreederKind::A, 1),
                        (PokemonBreederKind::B, 1),
                        (PokemonBreederKind::C, 0),
                        (PokemonBreederKind::D, 0),
                        (PokemonBreederKind::E, 0),
                    ]),
                },
            ),
            (
                3,
                BreedTreeFormation {
                    natured: HashMap::from([
                        (PokemonBreederKind::A, 4),
                        (PokemonBreederKind::B, 2),
                        (PokemonBreederKind::C, 1),
                        (PokemonBreederKind::D, 0),
                        (PokemonBreederKind::E, 0),
                    ]),
                    natureless: HashMap::from([
                        (PokemonBreederKind::A, 2),
                        (PokemonBreederKind::B, 1),
                        (PokemonBreederKind::C, 1),
                        (PokemonBreederKind::D, 0),
                        (PokemonBreederKind::E, 0),
                    ]),
                },
            ),
            (
                4,
                BreedTreeFormation {
                    natured: HashMap::from([
                        (PokemonBreederKind::A, 6),
                        (PokemonBreederKind::B, 5),
                        (PokemonBreederKind::C, 3),
                        (PokemonBreederKind::D, 1),
                        (PokemonBreederKind::E, 0),
                    ]),
                    natureless: HashMap::from([
                        (PokemonBreederKind::A, 3),
                        (PokemonBreederKind::B, 2),
                        (PokemonBreederKind::C, 2),
                        (PokemonBreederKind::D, 1),
                        (PokemonBreederKind::E, 0),
                    ]),
                },
            ),
            (
                5,
                BreedTreeFormation {
                    natured: HashMap::from([
                        (PokemonBreederKind::A, 11),
                        (PokemonBreederKind::B, 10),
                        (PokemonBreederKind::C, 6),
                        (PokemonBreederKind::D, 2),
                        (PokemonBreederKind::E, 2),
                    ]),
                    natureless: HashMap::from([
                        (PokemonBreederKind::A, 5),
                        (PokemonBreederKind::B, 5),
                        (PokemonBreederKind::C, 3),
                        (PokemonBreederKind::D, 2),
                        (PokemonBreederKind::E, 1),
                    ]),
                },
            ),
        ]);

        PokemonBreedTree {
            nodes,
            generations,
            position_map,
            final_pokemon,
        }
    }
}
