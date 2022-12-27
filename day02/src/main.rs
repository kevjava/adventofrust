use std::{fs, io::Error, fmt};

#[derive(Debug)]
enum RPSOutcome {
    WIN,
    DRAW,
    LOSE,
    INVALID,
}

fn from_outcome_code(code: &str) -> RPSOutcome {
    match code {
        "X" => RPSOutcome::LOSE,
        "Y" => RPSOutcome::DRAW,
        "Z" => RPSOutcome::WIN,
        _ => RPSOutcome::INVALID,
    }
}

#[derive(Debug)]
enum RPSPlay {
    ROCK,
    PAPER,
    SCISSORS,
    INVALID,
}

/// Parses an RPSPlay enum value from a player code.
fn from_player_code(code: &str) -> RPSPlay {
    match code {
        "X" => RPSPlay::ROCK,
        "Y" => RPSPlay::PAPER,
        "Z" => RPSPlay::SCISSORS,
        _ => RPSPlay::INVALID
    }
}

/// Parses an RPSPlay enum value from an opponent code.
fn from_opponent_code(code: &str) -> RPSPlay {
    match code {
        "A" => RPSPlay::ROCK,
        "B" => RPSPlay::PAPER,
        "C" => RPSPlay::SCISSORS,
        _ => RPSPlay::INVALID
    }
}


#[derive(Debug)]
struct Round {
    player: RPSPlay,
    opponent: RPSPlay,
    outcome: RPSOutcome,
}

impl Round {
    /**
    Creates a new [`Round`].
    */
    fn calculate_outcome(player: &RPSPlay, opponent: &RPSPlay) -> RPSOutcome {
        match player {
            RPSPlay::ROCK => {
                match opponent {
                    RPSPlay::ROCK => RPSOutcome::DRAW,
                    RPSPlay::PAPER => RPSOutcome::LOSE,
                    RPSPlay::SCISSORS => RPSOutcome::WIN,
                    _ => RPSOutcome::INVALID,
                }
            },
            RPSPlay::PAPER => {
                match opponent {
                    RPSPlay::ROCK => RPSOutcome::WIN,
                    RPSPlay::PAPER => RPSOutcome::DRAW,
                    RPSPlay::SCISSORS => RPSOutcome::LOSE,
                    _ => RPSOutcome::INVALID,
                }
            },
            RPSPlay::SCISSORS => {
                match opponent {
                    RPSPlay::ROCK => RPSOutcome::LOSE,
                    RPSPlay::PAPER => RPSOutcome::WIN,
                    RPSPlay::SCISSORS => RPSOutcome::DRAW,
                    _ => RPSOutcome::INVALID,
                }
            },
            _ => { RPSOutcome::INVALID }
        }
    }

    fn score(self: &Round) -> i32 {
        let score_for_play = match self.player {
            RPSPlay::ROCK => 1,
            RPSPlay::PAPER => 2,
            RPSPlay::SCISSORS => 3,
            _ => 0
        };
        let score_for_win = match self.outcome {
            RPSOutcome::WIN => 6,
            RPSOutcome::DRAW => 3,
            RPSOutcome::LOSE => 0,
            _ => 0
        };

        score_for_play + score_for_win
    }
}

impl fmt::Display for RPSOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::WIN => "WIN",
            Self::DRAW => "DRAW",
            Self::LOSE => "LOSE",
            _ => "INVALID"
        })
    }
}

impl fmt::Display for RPSPlay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::PAPER => "PAPER",
            Self::ROCK => "ROCK",
            Self::SCISSORS => "SCISSORS",
            Self::INVALID => "INVALID"
        })
    }
}

impl fmt::Display for Round {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(Player: {}, Opponent: {})", self.player, self.opponent)
    }
}

#[derive(Debug)]
struct GameByInputs {
    rounds: Vec<Round>,
}

impl GameByInputs {
    fn new(lines: &Vec<&str>) -> GameByInputs {
        let mut rounds: Vec<Round> = vec![];
        for line in lines {
            let parts: Vec<&str> = line.split(" ").collect();
            let player = from_player_code(parts[1]);
            let opponent = from_opponent_code(parts[0]);
            let round = Round {
                outcome: Round::calculate_outcome(&player, &opponent),
                player,
                opponent,
            };
            rounds.push(round);
        }

        GameByInputs {
            rounds,
        }
    }

    fn score(self: GameByInputs) -> i32 {
        let mut score_sum = 0; // FIXME: This is SO verbose.
        // for round in self.rounds {
        for (_i, round) in self.rounds.iter().enumerate() {
            let score = round.score();
            score_sum += score;
            // println!("Round {i}: {round}, score {score}. Cumulative: {score_sum}");
        }
        score_sum
    }
}

struct GameByOutcomes {
    rounds: Vec<Round>,
}

impl GameByOutcomes {
    fn new(lines: Vec<&str>) -> GameByOutcomes {
        let mut rounds: Vec<Round> = vec![];
        for line in lines {
            let parts: Vec<&str> = line.split(" ").collect();
            let outcome = from_outcome_code(parts[1]);
            let opponent = from_opponent_code(parts[0]);
            let player = get_optimal_play(&opponent, &outcome);
            let round = Round {
                player,
                opponent,
                outcome,
            };
            rounds.push(round);
        }

        GameByOutcomes {
            rounds,
        }
    }

    fn score(self: GameByOutcomes) -> i32 {
        let mut score_sum = 0; // FIXME: This is SO verbose.
        // for round in self.rounds {
        for (_i, round) in self.rounds.iter().enumerate() {
            let score = round.score();
            // let outcome = &round.outcome;
            score_sum += score;
            // println!("Round {i}: {round}, score {score}, outcome {outcome}. Cumulative: {score_sum}");
        }
        score_sum
    }
}

fn get_optimal_play(opponent: &RPSPlay, outcome: &RPSOutcome) -> RPSPlay {
    match opponent {
        RPSPlay::ROCK => match outcome {
            RPSOutcome::WIN => RPSPlay::PAPER,
            RPSOutcome::DRAW => RPSPlay::ROCK,
            RPSOutcome::LOSE => RPSPlay::SCISSORS,
            _ => RPSPlay::INVALID
        },
        RPSPlay::PAPER => match outcome {
            RPSOutcome::WIN => RPSPlay::SCISSORS,
            RPSOutcome::DRAW => RPSPlay::PAPER,
            RPSOutcome::LOSE => RPSPlay::ROCK,
            _ => RPSPlay::INVALID
        },
        RPSPlay::SCISSORS => match outcome {
            RPSOutcome::WIN => RPSPlay::ROCK,
            RPSOutcome::DRAW => RPSPlay::SCISSORS,
            RPSOutcome::LOSE => RPSPlay::PAPER,
            _ => RPSPlay::INVALID
        },
        _ => { RPSPlay::INVALID }
    }
}

/// Returns an expect-able file result given a filename.
fn open_file(filename: &str) -> Result<String, Error> {
    return fs::read_to_string(filename);
}

/// Takes in a string, splits it by newlines, returns a Vec with the lines.
fn split_lines(file_contents: &str) -> Vec<&str> {
    return file_contents
        .split("\n")
        .collect();
}

/// Day 2 of Advent of Code 2022.
fn main() {
    let input_file = open_file("input.txt")
        .expect("Couldn't open the file.");
    let contents = split_lines(&input_file);

    // Parse lines into rounds of a game. 
    let game = GameByInputs::new(&contents);

    // Score the game.
    let score = game.score();
    println!("Game score: {score}");

    let game2 = GameByOutcomes::new(contents);
    let score2 = game2.score();
    println!("Game 2 score: {score2}");
}
