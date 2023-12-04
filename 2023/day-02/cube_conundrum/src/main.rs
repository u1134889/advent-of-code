use std::{cmp::max, str::FromStr, fs};

fn main() {
    let mut sum = 0u64;
    let input = read_input();
    for line in input.lines() {
        let gs = GameSummary::from_str(line).expect("line should be parsed as a game summary");
        let max_summary = gs.round_summaries.iter().fold(RoundSummary{red: 0, green: 0, blue: 0}, max_round_summary);
        sum = calculate_next_sum_part_two(sum, max_summary, gs);
    }
    println!("total: {}", sum);
}
fn read_input() -> String {
    fs::read_to_string("D:\\SideProjects\\advent-of-code\\2023\\day-02\\cube_conundrum\\resources\\input.txt").expect("couldn't read input file")
}
fn max_round_summary(accumulator: RoundSummary, next: &RoundSummary) -> RoundSummary {
    RoundSummary {
        red: max(accumulator.red, next.red),
        green: max(accumulator.green, next.green),
        blue: max(accumulator.blue, next.blue),
    }
}
fn valid_game(summary: RoundSummary) -> bool {
    summary.red <= 12 && summary.green <= 13 && summary.blue <= 14
}
fn calculate_next_sum_part_one(sum: u64, max_summary: RoundSummary, curr_gs: GameSummary) -> u64 {
    if valid_game(max_summary) {
        sum + curr_gs.game_tag
    } else {
        sum
    }
}
fn calculate_next_sum_part_two(sum: u64, max_summary: RoundSummary, _curr_gs: GameSummary) -> u64 {
    sum + max_summary.power()
}

struct GameSummary {
    game_tag: u64,
    round_summaries: Vec<RoundSummary>
}
impl FromStr for GameSummary {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(":").expect("didn't contain a colon");

        let game_tag = parse_game_tag(left);
        let round_summaries = parse_round_summaries(right.trim());

        Ok(GameSummary {game_tag, round_summaries})
    }
    
}

fn parse_game_tag(game_info: &str) -> u64 {
    let (game, tag) = game_info.split_once(" ").expect("didn't contain a space");
    assert_eq!(game,"Game", "should have started with 'Game'");
    tag.parse::<u64>().expect("should have followed the pattern 'Game <number>'")
}

fn parse_round_summaries(round_info: &str) -> Vec<RoundSummary> {
    let mut summaries = Vec::new();
    for part in round_info.split(";") {
        let one_summary = part.trim().parse::<RoundSummary>().expect("could not part a round");
        summaries.push(one_summary);
    }
    summaries
}

#[derive(Default, Debug, PartialEq, Eq)]
struct RoundSummary {
    red: u64,
    green: u64,
    blue: u64
}
impl RoundSummary {
    fn from_rgb(rgb: (u64, u64, u64)) -> Self {
        RoundSummary { red: rgb.0, green: rgb.1, blue: rgb.2 }
    }
    fn power(&self) -> u64 {
        self.red * self.green * self.blue
    }
}
impl FromStr for RoundSummary {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut summary = RoundSummary::default();
        let message = "describing a color should be in the pattern '<number> <color>'";
        for color_summary in s.split(", ") {
            let (num, color) = color_summary.split_once(" ").expect(message);
            match color {
                "red" => summary.red = num.parse().expect(message),
                "green" => summary.green = num.parse().expect(message),
                "blue" => summary.blue = num.parse().expect(message),
                _ => return Err(String::from("an invalid color token was provided"))
            }
        }
        Ok(summary)
    }
}

#[cfg(test)]
mod tests {
    use crate::{GameSummary, parse_game_tag, RoundSummary};

    #[test]
    fn gs_parses_game_tag() {
        let result = "Game 1: 1 green, 4 blue; 1 blue, 2 green, 1 red".parse::<GameSummary>();
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.game_tag, 1);
    }
    #[test]
    fn individual_parse_tag() {
        let result = 22;
        assert_eq!(parse_game_tag("Game 22"), result);
    }
    #[test]
    fn gs_parses_first_summary() {
        let result = "Game 1: 1 green, 4 blue; 1 blue, 2 green, 1 red".parse::<GameSummary>();
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.round_summaries.first().is_some());
        assert_eq!(result.round_summaries.first().unwrap(), &RoundSummary::from_rgb((0, 1, 4)));
    }
    #[test]
    fn gs_parses_all_summaries() {
        let result = "Game 1: 1 green, 4 blue; 1 blue, 2 green, 1 red".parse::<GameSummary>();
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.round_summaries.first().is_some());
        assert_eq!(result.round_summaries.first().unwrap(), &RoundSummary::from_rgb((0, 1, 4)));
        assert_eq!(result.round_summaries.get(1).unwrap(), &RoundSummary::from_rgb((1, 2, 1)));
    }
    #[test]
    fn power_works() {
        assert_eq!(RoundSummary::from_rgb((4, 2, 6)).power(), 48);
        assert_eq!(RoundSummary::from_rgb((20, 13, 6)).power(), 1560);
    }
//     #[test]
//     fn experiment() {
//         assert!("hi" == "hi")
//     }
}
