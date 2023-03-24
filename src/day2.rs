pub struct Round {
    you: Move,
    elf: Move,
}

impl Round {
    fn points(&self) -> u32 {
        let move_point = self.you.value();
        if self.you == self.elf {return move_point + 3}
        if self.you == Move::Rock && self.elf == Move::Scissors {return move_point + 6}
        if self.you == Move::Paper && self.elf == Move::Rock {return move_point + 6}
        if self.you == Move::Scissors && self.elf == Move::Paper {return move_point + 6}
        move_point
    }
    fn points_round2(&self) -> u32 {
        if self.you == Move::Paper {return self.elf.value() + 3}
        if self.you == Move::Rock {
            if self.elf == Move::Paper {return 1}
            if self.elf == Move::Scissors {return 2}
            if self.elf == Move::Rock {return 3}
        }
        if self.you == Move::Scissors {
            if self.elf == Move::Paper {return 9}
            if self.elf == Move::Scissors {return 7}
            if self.elf == Move::Rock {return 8}
        }
        0
    }
}

#[derive(PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors
}

#[derive(Debug)]
struct InputError;

impl Move {
    fn from_str(s: &str,) -> Result<Move, InputError> {
        match s {
            "A" => Ok(Move::Rock),
            "B" => Ok(Move::Paper),
            "C" => Ok(Move::Scissors),
            "Y" => Ok(Move::Paper),
            "X" => Ok(Move::Rock),
            "Z" => Ok(Move::Scissors),
            _ => Err(InputError)
        }
    }
    fn value(&self) -> u32 {
        match self {
            Move::Rock => return 1,
            Move::Paper => return 2,
            Move::Scissors=> return 3,
        }
    } 
}


#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Round> {
    input
        .lines()
        .map(|l| {
            let mut round = l.trim().split(' ').map(|s| s);
            Round {
                elf: Move::from_str(round.next().unwrap()).unwrap(),
                you: Move::from_str(round.next().unwrap()).unwrap(),
            }
        }).collect()
    
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Vec<Round>) -> u32 {
    let mut total_score = 0;
    for round in input {
        total_score += round.points();
    }

    total_score
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Vec<Round>) -> u32 {
    let mut total_score = 0;
    for round in input {
        total_score += round.points_round2();
    }

    total_score
}
