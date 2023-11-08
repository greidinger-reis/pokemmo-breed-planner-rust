#![allow(unused, dead_code)]
use crate::pokemon::{Pokemon, PokemonGender, PokemonIv, PokemonNature};
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

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position(u8, u8);

impl Position {
    pub fn get_partner_position(self) -> Position {
        let Position(row, col) = self;

        let partner_col = if col % 2 == 0 { col + 1 } else { col - 1 };

        Position(row, partner_col)
    }

    // Warn: this can return invalid positions
    // Always check if the rows returned here are bigger than the generaions nr
    pub fn get_parent_positions(self) -> (Position, Position) {
        let Position(row, col) = self;

        let parent_row = row + 1;
        let parent_col = col * 2;

        let parent1 = Position(parent_row, parent_col);
        let parent2 = Position(parent_row, parent_col + 1);

        (parent1, parent2)
    }
}

type PokemonBreedTreePositionMap = HashMap<u8, PokemonBreedTreePosition>;

//option fields because the initial state of nodes are empty, only ivs are set
#[derive(Debug,PartialEq, Eq, Clone)]
pub struct PokemonBreedTreeNode {
    pub pokemon: Option<Pokemon>,
    pub gender: Option<PokemonGender>,
    pub nature: Option<PokemonNature>,
    pub ivs: Vec<PokemonIv>,
}

type PokemonNodes = HashMap<Position, PokemonBreedTreeNode>;

#[derive(Debug)]
pub struct PokemonBreedTree {
    pub pokemon_nodes: PokemonNodes,
    pub breed_errors: Vec<Position>,
}

type FinalPokemonIvsMap = HashMap<PokemonBreederKind, PokemonIv>;

impl PokemonBreedTree {
    pub fn new(
        final_pokemon_node: &PokemonBreedTreeNode,
        final_pokemon_ivs_map: &FinalPokemonIvsMap,
    ) -> PokemonBreedTree {
        let final_pokemon = final_pokemon_node
            .pokemon
            .clone()
            .expect("This should exist");
        let mut pokemon_nodes = HashMap::from([(Position(0, 0), final_pokemon_node.clone())]);
        let last_row_map = init_last_row_mapping();
        let natured = final_pokemon_node.nature.is_some();
        let generations = if natured {
            (final_pokemon_node.ivs.len() + 1) as u8
        } else {
            final_pokemon_node.ivs.len() as u8
        };

        let last_row_breeders = 
            last_row_map.get(&generations).expect("This shouldn't happen. Tried to access last_row_map with an invalid generations number");

        init_pokemon_nodes(
            generations,
            &mut pokemon_nodes,
            if natured {
                &last_row_breeders.natured
            } else {
                &last_row_breeders.natureless
            },
            final_pokemon_node,
            final_pokemon_ivs_map,
        );

        PokemonBreedTree {
            pokemon_nodes,
            breed_errors: vec![],
        }
    }

    pub fn get_final_pokemon_node(&self) -> &PokemonBreedTreeNode {
        self.pokemon_nodes
            .get(&Position(0, 0))
            .expect("This should exist")
    }

    pub fn insert_pokemon(&mut self, position: Position, pokemon_node: PokemonBreedTreeNode) {
        self.pokemon_nodes.insert(position, pokemon_node);
    }
}

// Initialize the pokemon nodes based on the initial position_map and the final pokemon ivs & nature.
fn init_pokemon_nodes(
    generations: u8,
    pokemon_nodes: &mut PokemonNodes,
    last_row_breeders: &PokemonBreederKindPositions,
    final_pokemon_node: &PokemonBreedTreeNode,
    final_pokemon_ivs: &FinalPokemonIvsMap,
) {
    // initialize last row
    for (k, v) in last_row_breeders {
        match v {
            PokemonBreederKind::Nature => {
                pokemon_nodes.insert(
                    *k,
                    PokemonBreedTreeNode {
                        pokemon: None,
                        gender: None,
                        ivs: vec![],
                        nature: final_pokemon_node.nature,
                    },
                );
            }
            _ => {
                pokemon_nodes.insert(
                    *k,
                    PokemonBreedTreeNode {
                        pokemon: None,
                        gender: None,
                        ivs: vec![final_pokemon_ivs.get(v).expect("Should exist").clone()],
                        nature: None,
                    },
                );
            }
        };
    }

    // initialize the rest of the tree
    // start from the second to last row
    // stops on the first row where the final pokemon node is already set
    let mut row = generations - 2;
    while row > 0 {
        let mut col = 0;
        while col < (2 as u8).pow(row as u32) {
            let pos = Position(row, col);
            let (p1_pos, p2_pos) = pos.get_parent_positions();

            let invalid_positions = p1_pos.0 > generations || p2_pos.0 > generations;
            if invalid_positions {
                continue;
            }
            let p1_node = pokemon_nodes.get(&p1_pos).expect("Should exist");
            let p2_node = pokemon_nodes.get(&p2_pos).expect("Should exist");

            let mut ivs: Vec<PokemonIv> = vec![];
            ivs.extend(p1_node.ivs.clone());
            ivs.extend(p2_node.ivs.clone());

            let mut nature: Option<PokemonNature> = None;
            if p1_node.nature.is_some() {
                nature = p1_node.nature.clone();
            } else if p2_node.nature.is_some() {
                nature = p2_node.nature.clone();
            }

            pokemon_nodes.insert(
                pos,
                PokemonBreedTreeNode {
                    pokemon: None,
                    gender: None,
                    nature,
                    ivs,
                },
            );

            col = col + 1;
        }

        row = row - 1;
    }
}

// This type represents what the last row of pokemon iv's should be, depending on the nr of
// generations
type LastRowMapping = HashMap<u8, PokemonBreedTreePosition>;

fn init_last_row_mapping() -> LastRowMapping {
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{pokemon::{PokemonIv, Pokemon, PokemonType, PokemonEggGroup}, tree::Position};

    use super::{PokemonBreedTree, PokemonBreedTreeNode, PokemonBreederKind};

    #[test]
    fn pokemon_breed_tree_constructor_2iv_test() {
        let final_ivs = [PokemonIv::Attack, PokemonIv::Speed].to_vec();
        let final_pokemon = Pokemon{
            number: 6,
            name: "Charizard".to_string(),
            types: (PokemonType::Fire, Some(PokemonType::Flying)),
            egg_groups: (PokemonEggGroup::Monster, Some(PokemonEggGroup::Dragon)),
            percentage_male: 87.5.to_string(),
        };
        let final_pokemon_node = PokemonBreedTreeNode{
            ivs: final_ivs.clone(),
            pokemon:Some(final_pokemon),
            gender: None,
            nature: None,
        };
        let final_iv_map = HashMap::from([(PokemonBreederKind::A, PokemonIv::Attack), (PokemonBreederKind::B, PokemonIv::Speed)]);
        let tree = PokemonBreedTree::new(&final_pokemon_node,&final_iv_map);

        assert_eq!(tree.pokemon_nodes.len(),3);
        assert_eq!(tree.get_final_pokemon_node(),&final_pokemon_node);
        assert_eq!(tree.pokemon_nodes.get(&Position(1,0)).expect("Should exist").ivs,vec![PokemonIv::Attack]);
        assert_eq!(tree.pokemon_nodes.get(&Position(1,1)).expect("Should exist").ivs,vec![PokemonIv::Speed]);
    }

    #[test]
    fn pokemon_breed_tree_constructor_3iv_test() {
        let final_ivs = [PokemonIv::Attack, PokemonIv::Speed,PokemonIv::HP].to_vec();
        let final_pokemon = Pokemon{
            number: 6,
            name: "Charizard".to_string(),
            types: (PokemonType::Fire, Some(PokemonType::Flying)),
            egg_groups: (PokemonEggGroup::Monster, Some(PokemonEggGroup::Dragon)),
            percentage_male: 87.5.to_string(),
        };
        let final_pokemon_node = PokemonBreedTreeNode{
            ivs: final_ivs.clone(),
            pokemon:Some(final_pokemon),
            gender: None,
            nature: None,
        };
        let final_iv_map = HashMap::from([(PokemonBreederKind::A, PokemonIv::Attack), (PokemonBreederKind::B, PokemonIv::Speed),
        (PokemonBreederKind::C, PokemonIv::HP)]);
        let tree = PokemonBreedTree::new(&final_pokemon_node,&final_iv_map);

        assert_eq!(tree.pokemon_nodes.len(),7);
        assert_eq!(tree.get_final_pokemon_node(),&final_pokemon_node);
        assert_eq!(tree.pokemon_nodes.get(&Position(2,0)).expect("Should exist").ivs,vec![PokemonIv::Attack]);
        assert_eq!(tree.pokemon_nodes.get(&Position(2,1)).expect("Should exist").ivs,vec![PokemonIv::Speed]);
        assert_eq!(tree.pokemon_nodes.get(&Position(2,2)).expect("Should exist").ivs,vec![PokemonIv::HP]);
        assert_eq!(tree.pokemon_nodes.get(&Position(1,0)).expect("Should exist").ivs,vec![PokemonIv::Attack, PokemonIv::Speed]);
        assert_eq!(tree.pokemon_nodes.get(&Position(1,1)).expect("Should exist").ivs,vec![PokemonIv::Attack, PokemonIv::HP]);
    }
}
