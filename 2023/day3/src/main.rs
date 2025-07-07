/**

Exercise https://adventofcode.com/2023/day/3

--- Day 3: Gear Ratios ---
You and the Elf eventually reach a gondola lift station; he says the gondola lift will take you up to the water source, but this is as far as he can bring you. You go inside.

It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.

"Aaah!"

You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working right now; it'll still be a while before I can fix it." You offer to help.

The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one. If you can add up all the part numbers in the engine schematic, it should be easy to work out which part is missing.

The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)

Here is an example engine schematic:

```
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
```

In this schematic, two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a part number; their sum is 4361.

Of course, the actual engine schematic is much larger. What is the sum of all of the part numbers in the engine schematic?
**/

/*

the best way of representing the engine schematic is a matrix of chars, where each string is a row of the schematic.
 */

fn main() {
    println!("AOC 2023 day 3.");

    // read input
    let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    // map to matrix of chars
    let array = map_to_2darray(input);

    // routine to compute sum of part numbers
    let mut total_sum = 0;
    for (i, row) in array.iter().enumerate() {
        let mut start_index;
        let mut end_index = 0;

        for (j, &c) in row.iter().enumerate() {
            // I will iterate all the row. For every digit found I will check whether is adjacent to a symbol.
            // if it is then I have to read the entire number.
            // To read the entire number I will need to find the start and end index of the number and save it or sum it

            if c == '.' {
                continue; // skip empty cells
            }
            // This is a trick to make the for skip to next iteration when we are in the situation that
            // a number has been found and summed but the j index is still in the range of the number.
            // This is needed to avoid double counting the same number.
            if j <= end_index {
                continue; // increment j until it reaches end_index
            }
            if c.is_numeric() {
                // check the adjacency to symbols
                let is_adjacent_to_symbol = [
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ]
                .iter()
                .any(|(di, dj)| {
                    let ni = i as isize + di;
                    let nj = j as isize + dj;
                    // check bounds
                    let bounds_valid =
                        ni >= 0 && nj >= 0 && ni < array.len() as isize && nj < row.len() as isize;
                    bounds_valid
                        && array[ni as usize][nj as usize] != '.'
                        && !array[ni as usize][nj as usize].is_numeric()
                });
                // println!(
                //     "value {} at index ({},{}) Is adjacent to symbol: {}",
                //     c, i, j, is_adjacent_to_symbol
                // );

                // if is adjacent to a symbol, then we need to read the entire number and skip over it
                if is_adjacent_to_symbol {
                    // to find the entire number, we need to find the start and end index of the number
                    // and then parse it from string
                    end_index = find_end_index_of_current_number(row, j);

                    start_index = find_start_index_of_current_number(row, end_index);

                    let number_to_sum = parse_number_from_row(row, start_index, end_index);

                    // println!("summing number: {}", number);
                    total_sum += number_to_sum; // add the number to the total sum

                    println!(
                        "Found number: {} from index {} to {}",
                        number_to_sum, start_index, end_index
                    );
                }
            }
        }
    }
    println!("Total sum of part numbers: {}", total_sum);
    assert_eq!(4361, total_sum, "The sum of part numbers is not correct");
}

fn parse_number_from_row(row: &Vec<char>, start_index: usize, end_index: usize) -> u32 {
    row[start_index..=end_index]
        .iter()
        .collect::<String>()
        .parse()
        .expect("could not parse number")
}

fn find_start_index_of_current_number(row: &Vec<char>, current_index: usize) -> usize {
    let mut start_index = current_index; // start with the current index
    for k in (0..current_index + 1).rev() {
        if row[k].is_numeric() {
            start_index = k; // update start index
        } else {
            break; // stop when we find a non-numeric character
        }
    }
    start_index
}

fn find_end_index_of_current_number(row: &Vec<char>, current_index: usize) -> usize {
    let mut end_index = current_index; // start with the current index
    for k in current_index..row.len() {
        if row[k].is_numeric() {
            end_index = k; // update end index
        } else {
            break; // stop when we find a non-numeric character
        }
    }
    end_index
}

fn map_to_2darray(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}
