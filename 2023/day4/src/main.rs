/**

Exercise https://adventofcode.com/2023/day/4

--- Day 4: Scratchcards ---
The gondola takes you up. Strangely, though, the ground doesn't seem to be coming with you; you're not climbing a mountain. As the circle of Snow Island recedes below you, an entire new landmass suddenly appears above you! The gondola carries you to the surface of the new island and lurches into the station.

As you exit the gondola, the first thing you notice is that the air here is much warmer than it was on Snow Island. It's also quite humid. Is this where the water source is?

The next thing you notice is an Elf sitting on the floor across the station in what seems to be a pile of colorful square cards.

"Oh! Hello!" The Elf excitedly runs over to you. "How may I be of service?" You ask about water sources.

"I'm not sure; I just operate the gondola lift. That does sound like something we'd have, though - this is Island Island, after all! I bet the gardener would know. He's on a different island, though - er, the small kind surrounded by water, not the floating kind. We really need to come up with a better naming scheme. Tell you what: if you can help me with something quick, I'll let you borrow my boat and you can go visit the gardener. I got all these scratchcards as a gift, but I can't figure out what I've won."

The Elf leads you over to the pile of colorful cards. There, you discover dozens of scratchcards, all with their opaque covering already scratched off. Picking one up, it looks like each card has two lists of numbers separated by a vertical bar (|): a list of winning numbers and then a list of numbers you have. You organize the information into a table (your puzzle input).

As far as the Elf has been able to figure out, you have to figure out which of the numbers you have appear in the list of winning numbers. The first match makes the card worth one point and each match after the first doubles the point value of that card.

For example:

Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
In the above example, card 1 has five winning numbers (41, 48, 83, 86, and 17) and eight numbers you have (83, 86, 6, 31, 17, 9, 48, and 53). Of the numbers you have, four of them (48, 83, 17, and 86) are winning numbers! That means card 1 is worth 8 points (1 for the first match, then doubled three times for each of the three matches after the first).

Card 2 has two winning numbers (32 and 61), so it is worth 2 points.
Card 3 has two winning numbers (1 and 21), so it is worth 2 points.
Card 4 has one winning number (84), so it is worth 1 point.
Card 5 has no winning numbers, so it is worth no points.
Card 6 has no winning numbers, so it is worth no points.
So, in this example, the Elf's pile of scratchcards is worth 13 points.

Take a seat in the large pile of colorful cards. How many points are they worth in total?
**/

struct Scratchcard {
    winning_numbers: Vec<usize>,
    my_numbers: Vec<usize>,
}

impl Scratchcard {
    fn calculate_points(&self) -> usize {
        let matches = self.winning_numbers
            .iter()
            .filter(|num| self.my_numbers.contains(num))
            .count();
        self.calculate_points_internal(matches)
    }

    fn calculate_points_internal(&self, matches: usize) -> usize {
        if matches == 0 {
            0
        } else {
            2usize.pow((matches - 1) as u32)
        }
    }
}

fn main() {
    println!("AOC 2023 day 4.");
    
    let pile_of_scratchcards = vec![
        Scratchcard {
            winning_numbers: vec![41, 48, 83, 86, 17],
            my_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
        },
        Scratchcard {
            winning_numbers: vec![13, 32, 20, 16, 61],
            my_numbers: vec![61, 30, 68, 82, 17, 32, 24, 19],
        },
        Scratchcard {
            winning_numbers: vec![1, 21, 53, 59, 44],
            my_numbers: vec![69, 82, 63, 72, 16, 21, 14, 1],
        },
        Scratchcard {
            winning_numbers: vec![41, 92, 73, 84, 69],
            my_numbers: vec![59, 84, 76, 51, 58, 5, 54, 83],
        },
        Scratchcard {
            winning_numbers: vec![87, 83, 26, 28, 32],
            my_numbers: vec![88, 30, 70, 12, 93, 22, 82, 36],
        },
        Scratchcard {
            winning_numbers: vec![31,18 ,13 ,56 ,72],
            my_numbers: vec![74 ,77 ,10 ,23 ,35 ,67 ,36 ,11],
        },
    ];
    
    let total_points: usize = pile_of_scratchcards.iter()
        .map(|card| card.calculate_points())
        .sum();
    
    println!("Total points in pile of scratchcards is: {}", total_points);
    assert_eq!(total_points, 13);
}


