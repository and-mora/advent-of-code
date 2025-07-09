use std::collections::HashMap;

/**

Exercise https://adventofcode.com/2023/day/5

--- Day 5: If You Give A Seed A Fertilizer ---
You take the boat and find the gardener right where you were told he would be: managing a giant "garden" that looks more to you like a farm.

"A water source? Island Island is the water source!" You point out that Snow Island isn't receiving any water.

"Oh, we had to stop the water because we ran out of sand to filter it with! Can't make snow with dirty water. Don't worry, I'm sure we'll get more sand soon; we only turned off the water a few days... weeks... oh no." His face sinks into a look of horrified realization.

"I've been so busy making sure everyone here has food that I completely forgot to check why we stopped getting more sand! There's a ferry leaving soon that is headed over in that direction - it's much faster than your boat. Could you please go check it out?"

You barely have time to agree to this request when he brings up another. "While you wait for the ferry, maybe you can help us with our food production problem. The latest Island Island Almanac just arrived and we're having trouble making sense of it."

The almanac (your puzzle input) lists all of the seeds that need to be planted. It also lists what type of soil to use with each kind of seed, what type of fertilizer to use with each kind of soil, what type of water to use with each kind of fertilizer, and so on. Every type of seed, soil, fertilizer and so on is identified with a number, but numbers are reused by each category - that is, soil 123 and fertilizer 123 aren't necessarily related to each other.

For example:

seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
The almanac starts by listing which seeds need to be planted: seeds 79, 14, 55, and 13.

The rest of the almanac contains a list of maps which describe how to convert numbers from a source category into numbers in a destination category. That is, the section that starts with seed-to-soil map: describes how to convert a seed number (the source) to a soil number (the destination). This lets the gardener and his team know which soil to use with which seeds, which water to use with which fertilizer, and so on.

Rather than list every source number and its corresponding destination number one by one, the maps describe entire ranges of numbers that can be converted. Each line within a map contains three numbers: the destination range start, the source range start, and the range length.

Consider again the example seed-to-soil map:

50 98 2
52 50 48
The first line has a destination range start of 50, a source range start of 98, and a range length of 2. This line means that the source range starts at 98 and contains two values: 98 and 99. The destination range is the same length, but it starts at 50, so its two values are 50 and 51. With this information, you know that seed number 98 corresponds to soil number 50 and that seed number 99 corresponds to soil number 51.

The second line means that the source range starts at 50 and contains 48 values: 50, 51, ..., 96, 97. This corresponds to a destination range starting at 52 and also containing 48 values: 52, 53, ..., 98, 99. So, seed number 53 corresponds to soil number 55.

Any source numbers that aren't mapped correspond to the same destination number. So, seed number 10 corresponds to soil number 10.

So, the entire list of seed numbers and their corresponding soil numbers looks like this:

seed  soil
0     0
1     1
...   ...
48    48
49    49
50    52
51    53
...   ...
96    98
97    99
98    50
99    51
With this map, you can look up the soil number required for each initial seed number:

Seed number 79 corresponds to soil number 81.
Seed number 14 corresponds to soil number 14.
Seed number 55 corresponds to soil number 57.
Seed number 13 corresponds to soil number 13.
The gardener and his team want to get started as soon as possible, so they'd like to know the closest location that needs a seed. Using these maps, find the lowest location number that corresponds to any of the initial seeds. To do this, you'll need to convert each seed number through other categories until you can find its corresponding location number. In this example, the corresponding types are:

Seed 79, soil 81, fertilizer 81, water 81, light 74, temperature 78, humidity 78, location 82.
Seed 14, soil 14, fertilizer 53, water 49, light 42, temperature 42, humidity 43, location 43.
Seed 55, soil 57, fertilizer 57, water 53, light 46, temperature 82, humidity 82, location 86.
Seed 13, soil 13, fertilizer 52, water 41, light 34, temperature 34, humidity 35, location 35.
So, the lowest location number in this example is 35.

What is the lowest location number that corresponds to any of the initial seed numbers?


**/

/**
There are 2 ways I can think of to represent the almanac:
- a hashmap for each category, where the key is the source category and the value is the destination category.
- an array of linked lists that already "resolves" the chain of categories, e.g. [0] is the seed, [1] is the soil, [2] is the fertilizer, etc.
    The data structure may as well be an array 2d
-

The first one is more flexible, but the second one is more efficient for lookups.
**/

struct Almanac {
    seeds: Vec<u8>,
    seed_to_soil: HashMap<u8, u8>,
    soil_to_fertilizer: HashMap<u8, u8>,
    fertilizer_to_water: HashMap<u8, u8>,
    water_to_light: HashMap<u8, u8>,
    light_to_temperature: HashMap<u8, u8>,
    temperature_to_humidity: HashMap<u8, u8>,
    humidity_to_location: HashMap<u8, u8>,
}

impl Almanac {
    fn new(input: String) -> Self {
        let input_splitted: Vec<&str> = input.split("\n\n").collect();
        let seeds = input_splitted[0]
            .replace("seeds: ", "")
            .split_whitespace()
            .map(|num| num.parse::<u8>().unwrap())
            .collect();
        println!("seeds: {:?}", seeds);
        let mut seed_to_soil: HashMap<u8, u8> = (0u8..100u8).map(|x| (x, x)).collect();
        input_splitted[1]
            .lines()
            .filter(|line| !line.starts_with("seed-to-soil map:"))
            .for_each(|line| {
                let seed_to_soil_values: Vec<u8> = line
                    .split_whitespace()
                    .map(|s| s.parse::<u8>().unwrap())
                    .collect();
                println!("seeds to soil {:?}", seed_to_soil_values);
                for (index, seed_value) in (seed_to_soil_values[1]
                    ..seed_to_soil_values[1] + seed_to_soil_values[2])
                    .enumerate()
                {
                    seed_to_soil.insert(seed_value, seed_to_soil_values[0] + index as u8);
                }
            });
        let mut soil_to_fertilizer: HashMap<u8, u8> = (0u8..100u8).map(|x| (x, x)).collect();
        input_splitted[2]
            .lines()
            .filter(|line| !line.starts_with("soil-to-fertilizer map:"))
            .for_each(|line| {
                let soil_to_fertilizer_values: Vec<u8> = line
                    .split_whitespace()
                    .map(|s| s.parse::<u8>().unwrap())
                    .collect();
                println!("soil to fertilizer {:?}", soil_to_fertilizer_values);
                for (index, soil_value) in (soil_to_fertilizer_values[1]
                    ..soil_to_fertilizer_values[1] + soil_to_fertilizer_values[2])
                    .enumerate()
                {
                    soil_to_fertilizer
                        .insert(soil_value, soil_to_fertilizer_values[0] + index as u8);
                }
            });

        let mut fertilizer_to_water: HashMap<u8, u8> = (0u8..100u8).map(|x| (x, x)).collect();
        input_splitted[3]
            .lines()
            .filter(|line| !line.starts_with("fertilizer-to-water map:"))
            .for_each(|line| {
                let fertilizer_to_water_values: Vec<u8> = line
                    .split_whitespace()
                    .map(|s| s.parse::<u8>().unwrap())
                    .collect();
                println!("fertilizer to water {:?}", fertilizer_to_water_values);
                for (index, fertilizer_value) in (fertilizer_to_water_values[1]
                    ..fertilizer_to_water_values[1] + fertilizer_to_water_values[2])
                    .enumerate()
                {
                    fertilizer_to_water.insert(
                        fertilizer_value,
                        fertilizer_to_water_values[0] + index as u8,
                    );
                }
            });

        let mut water_to_light: HashMap<u8, u8> = (0u8..100u8).map(|x| (x, x)).collect();
        input_splitted[4]
            .lines()
            .filter(|line| !line.starts_with("water-to-light map:"))
            .for_each(|line| {
                let water_to_light_values: Vec<u8> = line
                    .split_whitespace()
                    .map(|s| s.parse::<u8>().unwrap())
                    .collect();
                println!("water to light {:?}", water_to_light_values);
                for (index, water_value) in (water_to_light_values[1]
                    ..water_to_light_values[1] + water_to_light_values[2])
                    .enumerate()
                {
                    water_to_light.insert(water_value, water_to_light_values[0] + index as u8);
                }
            });

        let mut light_to_temperature: HashMap<u8, u8> = (0u8..100u8).map(|x| (x, x)).collect();
        input_splitted[5]
            .lines()
            .filter(|line| !line.starts_with("light-to-temperature map:"))
            .for_each(|line| {
                let water_to_light_values: Vec<u8> = line
                    .split_whitespace()
                    .map(|s| s.parse::<u8>().unwrap())
                    .collect();
                println!("water to light {:?}", water_to_light_values);
                for (index, water_value) in (water_to_light_values[1]
                    ..water_to_light_values[1] + water_to_light_values[2])
                    .enumerate()
                {
                    light_to_temperature
                        .insert(water_value, water_to_light_values[0] + index as u8);
                }
            });

        let mut temperature_to_humidity: HashMap<u8, u8> = (0u8..100u8).map(|x| (x, x)).collect();
        input_splitted[6]
            .lines()
            .filter(|line| !line.starts_with("temperature-to-humidity map:"))
            .for_each(|line| {
                let light_to_temperature_values: Vec<u8> = line
                    .split_whitespace()
                    .map(|s| s.parse::<u8>().unwrap())
                    .collect();
                println!("light to temperature {:?}", light_to_temperature_values);
                for (index, light_value) in (light_to_temperature_values[1]
                    ..light_to_temperature_values[1] + light_to_temperature_values[2])
                    .enumerate()
                {
                    temperature_to_humidity
                        .insert(light_value, light_to_temperature_values[0] + index as u8);
                }
            });

        let mut humidity_to_location: HashMap<u8, u8> = (0u8..100u8).map(|x| (x, x)).collect();
        input_splitted[7]
            .lines()
            .filter(|line| !line.starts_with("humidity-to-location map:"))
            .for_each(|line| {
                let temperature_to_humidity_values: Vec<u8> = line
                    .split_whitespace()
                    .map(|s| s.parse::<u8>().unwrap())
                    .collect();
                println!(
                    "temperature to humidity {:?}",
                    temperature_to_humidity_values
                );
                for (index, temperature_value) in (temperature_to_humidity_values[1]
                    ..temperature_to_humidity_values[1] + temperature_to_humidity_values[2])
                    .enumerate()
                {
                    humidity_to_location.insert(
                        temperature_value,
                        temperature_to_humidity_values[0] + index as u8,
                    );
                }
            });

        Self {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        }
    }

    fn get_location_for_seed(&self, seed: u8) -> Option<&u8> {
        self.seed_to_soil
            .get(&seed)
            .map(|soil| self.soil_to_fertilizer.get(soil).unwrap())
            .map(|fertilizer| self.fertilizer_to_water.get(fertilizer).unwrap())
            .map(|water| self.water_to_light.get(water).unwrap())
            .map(|light| self.light_to_temperature.get(light).unwrap())
            .map(|temperature| self.temperature_to_humidity.get(temperature).unwrap())
            .and_then(|humidity| self.humidity_to_location.get(humidity))
    }
}

fn main() {
    println!("AOC 2023 day 5.");

    let input_almanac = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    let almanac = Almanac::new(input_almanac.to_string());
    let lowest_position = almanac
        .seeds
        .iter()
        .map(|seed| almanac.get_location_for_seed(*seed).unwrap())
        .min()
        .unwrap();
    println!(
        "Lowest location number that corresponds to any of the initial seed numbers is: {:?}",
        lowest_position
    );
    assert_eq!(*lowest_position, 35);
}
