#![allow(unused, dead_code)]
use phf::phf_map;

type GenderlessPokemonEvoTree = (u16, u16, Option<u16>);

const MAGNEMITE_TREE: GenderlessPokemonEvoTree = (81, 82, Some(462));
const STARYU_TREE: GenderlessPokemonEvoTree = (120, 121, None);
const BRONZOR_TREE: GenderlessPokemonEvoTree = (436, 437, None);
const BELDUM_TREE: GenderlessPokemonEvoTree = (374, 375, Some(376));
const BALTOY_TREE: GenderlessPokemonEvoTree = (343, 344, None);
const VOLTORB_TREE: GenderlessPokemonEvoTree = (100, 101, None);
const PORYGON_TREE: GenderlessPokemonEvoTree = (137, 233, Some(474));
const KLINK_TREE: GenderlessPokemonEvoTree = (599, 600, Some(601));
const GOLETT_TREE: GenderlessPokemonEvoTree = (622, 623, None);

/**
 * This is for getting valid breeding pokemon of a specified genderless species
 */
pub const GENDERLESS_POKEMON_EVOLUTION_TREE: phf::Map<u16, GenderlessPokemonEvoTree> = phf_map! {
    81u16 => MAGNEMITE_TREE,
    82u16 => MAGNEMITE_TREE,
    462u16 => MAGNEMITE_TREE,
    120u16 => STARYU_TREE,
    121u16 => STARYU_TREE,
    436u16 => BRONZOR_TREE,
    437u16 => BRONZOR_TREE,
    374u16 => BELDUM_TREE,
    375u16 => BELDUM_TREE,
    376u16 => BELDUM_TREE,
    343u16 => BALTOY_TREE,
    344u16 => BALTOY_TREE,
    100u16 => VOLTORB_TREE,
    101u16 => VOLTORB_TREE,
    137u16 => PORYGON_TREE,
    233u16 => PORYGON_TREE,
    474u16 => PORYGON_TREE,
    599u16 => KLINK_TREE,
    600u16 => KLINK_TREE,
    601u16 => KLINK_TREE,
    622u16 => GOLETT_TREE,
    623u16 => GOLETT_TREE,
};
