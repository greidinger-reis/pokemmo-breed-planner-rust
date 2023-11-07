#![allow(unused, dead_code)]
use crate::pokemon::{Pokemon, PokemonGender, PokemonIv};
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

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position(u8, u8);

impl Position {
    pub fn get_partner_position(self) -> Position {
        let Position(row, col) = self;

        let partner_col = if col % 2 == 0 { col + 1 } else { col - 1 };

        Position(row, partner_col)
    }
}

type PokemonBreedTreePositionMap = HashMap<u8, PokemonBreedTreePosition>;

//option fields because the initial state of nodes are empty, only ivs are set
#[derive(Debug)]
struct PokemonBreedTreeNode {
    pokemon: Option<Pokemon>, 
    gender: Option<PokemonGender>,
    ivs: Vec<PokemonIv>,
}

#[derive(Debug)]
pub struct PokemonBreedTree {
    pub pokemon_nodes: HashMap<Position, PokemonBreedTreeNode>,
    pub breed_errors: Vec<Position>,
}

impl PokemonBreedTree {
    pub fn new(
        final_pokemon: Pokemon,
        final_pokemon_ivs: HashMap<PokemonBreederKind, PokemonIv>,
    ) -> PokemonBreedTree {
        let final_pokemon_node = PokemonBreedTreeNode{
            // get all ivs from the final_pokemon_ivs and construct a Vec<PokemonIv>
            ivs: final_pokemon_ivs.values().cloned().collect(),
        }
        let breed_errors = Vec::<Position>::new();
        let mut pokemon_nodes = HashMap::from([(Position(0, 0),
            final_pokemon)]);
        let last_row_map = init_last_row_mapping();
        init_pokemon_nodes(&mut pokemon_nodes, &last_row_map,
            &final_pokemon, &final_pokemon_ivs);

        PokemonBreedTree {
            pokemon_nodes,
            breed_errors,
        }
    }

    pub fn get_final_pokemon_node(&self) -> &PokemonBreedTreeNode {
        self.pokemon_nodes.get(&Position(0, 0)).unwrap()
    }

    pub fn insert_pokemon(&self, position: Position, pokemon_node: PokemonBreedTreeNode) {
        self.pokemon_nodes.insert(position, pokemon_node);
    }
}

// Initialize the pokemon nodes based on the initial position_map and the final pokemon ivs & nature.
fn init_pokemon_nodes(
    pokemon_nodes: &mut HashMap<Position, PokemonBreedTreeNode>,
    position_map: &HashMap<u8, PokemonBreedTreePosition>,
    final_pokemon: &Pokemon,
    final_pokemon_ivs: &HashMap<PokemonBreederKind, PokemonIv>,
) {
    let generations = if final_pokemon.nature.is_some() {
        (final_pokemon_ivs.len() + 1) as u8
    } else {
        final_pokemon_ivs.len() as u8
    };

    //row starts at 1 because we already have the final pokemon at pos 0,0
    for row in 1..generations {
        for col in 0..((2u8.pow(row as u32)) - 1) {
            let position = Position(row, col);
            let pokemon_node = get_node_from_position_map(
                pokemon_nodes,
                position_map,
                position,
                final_pokemon_ivs,
            );
            pokemon_nodes.insert(position, pokemon_node);
        }
    }
    

}

fn get_node_from_position_map(
    pokemon_nodes: &mut HashMap<Position, PokemonBreedTreeNode>,
    position_map: &HashMap<u8, PokemonBreedTreePosition>,
    position: Position,
    final_pokemon_ivs: &HashMap<PokemonBreederKind, PokemonIv>
) -> PokemonBreedTreeNode {


}

fn init_last_row_mapping() -> HashMap<u8, PokemonBreedTreePosition> {
    let last_row = HashMap::<u8, PokemonBreedTreePosition>::from([
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
    last_row
}
