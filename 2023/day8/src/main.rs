use std::cmp::PartialEq;
use std::collections::HashMap;

/**

Exercise https://adventofcode.com/2023/day/8


**/

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    L,
    R,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
enum NodeLabel {
    AAA,
    BBB,
    CCC,
    DDD,
    EEE,
    ZZZ,
}

struct Destination {
    right: NodeLabel,
    left: NodeLabel,
}

impl Destination {
    fn new(left: NodeLabel, right: NodeLabel) -> Self {
        Destination { left, right }
    }
}

struct DesertMap {
    instructions: Vec<Direction>,
    map: HashMap<NodeLabel, Destination>,
}

impl DesertMap {
    fn new(instructions: Vec<Direction>, map: HashMap<NodeLabel, Destination>) -> Self {
        DesertMap { instructions, map }
    }

    fn calculate_steps_to_destination(&self, start_node: NodeLabel, end_node: NodeLabel) -> u32 {
        if start_node == end_node {
            return 0;
        }
        let mut current_node = start_node;
        let max_iterations = 20;
        let mut steps: u32 = 0;
        for _ in 0..max_iterations {
            for direction in &self.instructions {
                match *direction {
                    Direction::L => {
                        current_node = self.map.get(&current_node).unwrap().left;
                        steps += 1;
                    }
                    Direction::R => {
                        current_node = self.map.get(&current_node).unwrap().right;
                        steps += 1;
                    }
                }
                println!("current node {:?}", &current_node);

                if current_node == end_node {
                    println!("Reached destination: {:?}", &end_node);
                    return steps;
                }
            }
        }
        steps
    }

    fn calculate_steps_to_destination_recursive(
        &self,
        current_node: NodeLabel,
        end_node: NodeLabel,
        steps: u32,
        direction: &Direction,
    ) -> u32 {
        if current_node == end_node {
            return steps;
        }
        let next_direction: &Direction = self.instructions.get(((steps + 1) % 3) as usize).unwrap();
        match direction {
            Direction::L => {
                let next_node = self.map.get(&current_node).unwrap().left;
                self.calculate_steps_to_destination_recursive(
                    next_node,
                    end_node,
                    steps + 1,
                    next_direction,
                )
            }
            Direction::R => {
                let next_node = self.map.get(&current_node).unwrap().right;
                self.calculate_steps_to_destination_recursive(
                    next_node,
                    end_node,
                    steps + 1,
                    next_direction,
                )
            }
        }
    }
}

fn main() {
    println!("AOC 2023 day 8.");

    // LLR
    //
    // AAA = (BBB, BBB)
    // BBB = (AAA, ZZZ)
    // ZZZ = (ZZZ, ZZZ)
    let instructions = vec![Direction::L, Direction::L, Direction::R];
    let map = DesertMap::new(
        instructions.clone(),
        HashMap::from([
            (
                NodeLabel::AAA,
                Destination::new(NodeLabel::BBB, NodeLabel::BBB),
            ),
            (
                NodeLabel::BBB,
                Destination::new(NodeLabel::AAA, NodeLabel::ZZZ),
            ),
            (
                NodeLabel::ZZZ,
                Destination::new(NodeLabel::ZZZ, NodeLabel::ZZZ),
            ),
        ]),
    );

    let steps = map.calculate_steps_to_destination(NodeLabel::AAA, NodeLabel::ZZZ);
    println!("Steps to destination: {}", steps);
    assert_eq!(steps, 6);

    let steps = map.calculate_steps_to_destination_recursive(
        NodeLabel::AAA,
        NodeLabel::ZZZ,
        0,
        &instructions.get(0).unwrap(),
    );
    println!("Steps to destination: {}", steps);
    assert_eq!(steps, 6);
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
