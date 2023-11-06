use phf::phf_map;

type EvolutionTreePokemonNames = (&'static str, &'static str, Option<&'static str>);

const MAGNEMITE_TREE: EvolutionTreePokemonNames = ("Magnemite", "Magneton",Some("Magnezone"));

/**
 * This is for getting valid breeding pokemon of a specified genderless species
 */
pub const GENDERLESS_POKEMON_EVOLUTION_TREE: phf::Map<
    &'static str,
    EvolutionTreePokemonNames> = phf_map! {
    "Magnemite" => ,
};
