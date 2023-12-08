use std::str::FromStr;

use crate::game::{Extraction, Game};

mod game;

/**
Exercise https://adventofcode.com/2023/day/2
 **/

fn main() {
    println!("AOC Day 2.");

    let sample_input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    let sample_request = Extraction::new(12, 14, 13);

    let games = parse_list_games(sample_input);

    let games_compatible: Vec<&Game> = games.iter()
        .filter(|game| is_game_compatible(game, &sample_request))
        .collect();

    println!("with sample input and sample request, the compatible games are: ");
    games_compatible.iter().for_each(|game| println!("{}", game));
}

fn is_game_compatible(game: &Game, request: &Extraction) -> bool {
    &game.max_extraction() <= request
}

fn parse_list_games(games: &str) -> Vec<Game> {
    games.split('\n')
        .map(parse_single_game_row)
        .collect()
}

///
/// # Expected format:
///
/// ```
/// Game 1: 1 blue, 2 green, 3 red
/// ```
fn parse_single_game_row(game_string: &str) -> Game {
    // extract the first part of string 'Game x' by ':'
    let game_splitted: Vec<&str> = game_string.split(':').collect();
    let game_id: u8 = game_splitted.first().unwrap()
        .matches(char::is_numeric)
        .map(|value| u8::from_str(value).unwrap())
        .sum();

    // extract the rest of string aka extractions, by ';'
    let mut extractions: Vec<Extraction> = vec![];
    for extraction in game_splitted.last().unwrap().split(';') {
        extractions.push(parse_extraction(extraction));
    }

    Game::new(game_id, extractions)
}

///
/// Parse a string and return the representation of it as Extraction.
///
/// # Expected format:
/// ```
/// 1 blue, 2 green, 3 red
/// ```
///
fn parse_extraction(extraction_string: &str) -> Extraction {
    let string_splitted: Vec<&str> = extraction_string.split(',').collect();
    let mut blue_cubes = 0;
    let mut red_cubes = 0;
    let mut green_cubes = 0;
    for cube in string_splitted.iter().map(|str| str.trim()) {
        if cube.ends_with("blue") {
            blue_cubes = extract_number_from_string(cube);
        } else if cube.ends_with("red") {
            red_cubes = extract_number_from_string(cube);
        } else if cube.ends_with("green") {
            green_cubes = extract_number_from_string(cube);
        } else {
            // throw error
        }
    }
    Extraction::new(red_cubes, blue_cubes, green_cubes)
}

///
/// # Expected format:
/// ```
/// <number-multi-digits> <other string>
///
fn extract_number_from_string(string: &str) -> u8 {
    let str: String = string.matches(char::is_numeric)
        .map(|s| s.chars())
        .flatten()
        .collect();
    u8::from_str(str.as_str()).unwrap()
}

#[test]
fn given_a_game_and_a_request_when_check_compatibility_then_false() {
    let game = Game::new(1, vec![Extraction::new(12, 10, 5)]);
    let request = Extraction::new(10, 10, 5);

    let is_compatible = is_game_compatible(&game, &request);

    assert_eq!(is_compatible, false);
}

#[test]
fn given_a_bunch_of_games_as_string_when_parse_then_ok() {
    let str = "Game 1: 1 blue, 2 green, 3 red; 1 red, 2 green\n\
    Game 2: 2 blue, 3 green; 3 red";

    let parsed_games: Vec<Game> = parse_list_games(str);

    assert_eq!(parsed_games.len(), 2);
    assert_eq!(parsed_games.last().unwrap().id(), 2);
}

#[test]
fn given_a_game_as_string_when_parse_then_ok() {
    let str = "Game 1: 1 blue, 2 green, 3 red; 1 red, 2 green";

    let game = parse_single_game_row(str);

    assert_eq!(game.id(), 1);
    assert_eq!(game.extractions().len(), 2);
    assert_eq!(game.extractions().first().unwrap(), &Extraction::new(3, 1, 2));
    assert_eq!(game.extractions().last().unwrap(), &Extraction::new(1, 0, 2));
}


#[test]
fn given_3_extractions_when_parse_then_ok() {
    let str = "1 blue, 2 green, 3 red";

    let extraction = parse_extraction(str);

    assert_eq!(extraction.red(), 3);
    assert_eq!(extraction.blue(), 1);
    assert_eq!(extraction.green(), 2);
}

#[test]
fn given_extractions_2digit_when_parse_then_ok() {
    let str = "10 blue, 2 green, 3 red";

    let extraction = parse_extraction(str);

    assert_eq!(extraction.red(), 3);
    assert_eq!(extraction.blue(), 10);
    assert_eq!(extraction.green(), 2);
}

