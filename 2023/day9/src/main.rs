/**

Exercise https://adventofcode.com/2023/day/9


**/

struct Sequence {
    numbers: Vec<u32>,
}

impl Sequence {
    fn new(numbers: Vec<u32>) -> Self {
        Sequence { numbers }
    }
    fn predict_next_value(&self) -> Option<u32> {
        if self.numbers.len() < 2 {
            return None; // Not enough data to predict
        }
        // obtain a new array of differences until the last array is all zeros.
        // Store all the differences arrays. They will be needed to calculate the next value.
        let mut arrays: Vec<Vec<u32>> = vec![];
        arrays.push(self.numbers.clone());
        
        loop {
            if arrays.last().unwrap().iter().all(|x| *x == 0) {
                break; // Stop if the last array is all zeros
            }

            let last_array = arrays.last().unwrap();
            let mut current_array = vec![];
            for i in 0..last_array.len() - 1 {
                let diff = last_array[i + 1] - last_array[i];

                current_array.push(diff);
            }
            // println!("{:?}", current_array);
            arrays.push(current_array);
        }

        // predict the last value of all the obtained arrays iterating bottom up
        // add the last value of the current array to the previous difference number to predict the next value of the current array
        let mut previous_difference_number = 0;
        let mut predicted_value = 0;
        for array in arrays.iter().rev() {
            if array.iter().all(|x| *x == 0) {
                continue; // Skip arrays that are all zeros
            }
            let current_last_value = array.last().unwrap();
            predicted_value = current_last_value + previous_difference_number;
            previous_difference_number = predicted_value;

            // println!("{:?}", predicted_value);
        }

        Some(predicted_value)
    }
}

struct OasisReport {
    sequences: Vec<Sequence>,
}

impl OasisReport {
    fn calculate_prediction_sum(&self) -> u32 {
        self.sequences
            .iter()
            .filter_map(|seq| seq.predict_next_value())
            .sum()
    }
}

fn main() {
    println!("AOC 2023 day 9.");

    // input value
    // 0 3 6 9 12 15
    // 1 3 6 10 15 21
    // 10 13 16 21 30 45

    let sequences = vec![
        Sequence::new(vec![0, 3, 6, 9, 12, 15]),
        Sequence::new(vec![1, 3, 6, 10, 15, 21]),
        Sequence::new(vec![10, 13, 16, 21, 30, 45]),
    ];
    let report = OasisReport { sequences };
    let prediction_sum = report.calculate_prediction_sum();

    println!(
        "The sum of the next predicted values is: {}",
        prediction_sum
    );

    assert_eq!(prediction_sum, 114);
}

#[cfg(test)]
mod tests {

    // #[test]
    // fn test_hand_ordering_less() {
    //     let hand1 = Hand::new("32T3K", 0);
    //     let hand2 = Hand::new("T55J5", 0);
    //     assert_eq!(hand1 > hand2, false);
    // }
}
