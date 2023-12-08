use std::cmp::{max, Ordering};

pub struct Game {
    id: u8,
    extractions: Vec<Extraction>,
}

impl Game {
    pub fn new(id: u8, extractions: Vec<Extraction>) -> Self {
        Self { id, extractions }
    }

    pub fn id(&self) -> u8 {
        self.id
    }

    pub fn extractions(&self) -> &Vec<Extraction> {
        self.extractions.as_ref()
    }

    pub fn max_extraction(&self) -> Extraction {
        let max_blue = self.extractions.iter().map(Extraction::blue).max().unwrap();
        let max_red = self.extractions.iter().map(Extraction::red).max().unwrap();
        let max_green = self.extractions.iter().map(Extraction::green).max().unwrap();

        Extraction::new(max_red, max_blue, max_green)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub struct Extraction {
    red: u8,
    blue: u8,
    green: u8,
}

impl Ord for Extraction {

    fn cmp(&self, other: &Self) -> Ordering {
        self.red.cmp(&other.red)
            .then(self.blue.cmp(&other.blue))
            .then(self.green.cmp(&other.green))
    }
}

impl Extraction {
    pub fn new(red: u8, blue: u8, green: u8) -> Self {
        Self { red, blue, green }
    }

    pub fn red(&self) -> u8 {
        self.red
    }
    pub fn blue(&self) -> u8 {
        self.blue
    }
    pub fn green(&self) -> u8 {
        self.green
    }
}


#[test]
fn given_a_game_when_compute_max_extractions_then_match() {
    let game = Game::new(1, vec![Extraction::new(3, 4, 5),
                                 Extraction::new(5, 2, 6)]);

    let max_extraction = game.max_extraction();

    assert_eq!(max_extraction.green, 6);
    assert_eq!(max_extraction.red, 5);
    assert_eq!(max_extraction.blue, 4);
}

