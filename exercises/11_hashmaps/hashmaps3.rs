// A list of scores (one per line) of a soccer match is given. Each line is of
// the form "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"
// Example: "England,France,4,2" (England scored 4 goals, France 2).
//
// You have to build a scores table containing the name of the team, the total
// number of goals the team scored, and the total number of goals the team
// conceded.

/*
    Hint
    Hint 1: Use the `entry()` and `or_default()` methods of `HashMap` to insert the
            default value of `TeamScores` if a team doesn't exist in the table yet.

    Hint 2: If there is already an entry for a given key, the value returned by
            `entry()` can be updated based on the existing value.

    Learn more in The Book:
    https://doc.rust-lang.org/book/ch08-03-hash-maps.html#updating-a-value-based-on-the-old-value
*/

use std::collections::HashMap;

// A structure to store the goal details of a team.
#[derive(Default)]
struct TeamScores {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: &str) -> HashMap<&str, TeamScores> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores = HashMap::<&str, TeamScores>::new();

    fn update_scores<'a>(mut scores: HashMap<&'a str, TeamScores>, team_name: &'a str, team_score: u8, opponent_score: u8) -> HashMap<&'a str, TeamScores> {
        let entry = scores.entry(team_name).or_default();
        entry.goals_scored += team_score;
        entry.goals_conceded += opponent_score;
        scores
    }

    for line in results.lines() {
        let mut split_iterator = line.split(',');
        // NOTE: We use `unwrap` because we didn't deal with error handling yet.
        let team_1_name = split_iterator.next().unwrap();
        let team_2_name = split_iterator.next().unwrap();
        let team_1_score: u8 = split_iterator.next().unwrap().parse().unwrap();
        let team_2_score: u8 = split_iterator.next().unwrap().parse().unwrap();

        // TODO: Populate the scores table with the extracted details.
        // Keep in mind that goals scored by team 1 will be the number of goals
        // conceded by team 2. Similarly, goals scored by team 2 will be the
        // number of goals conceded by team 1.

        /*
            1. Determine if an entry exists for this team name
                a. If an entry doesn't exist, insert an entry with default values, then add the corresponding scores to each value
                b. If an entry exists, add the corresponding scores to each value
        */

        // let team_1_entry = scores.entry(team_1_name).or_default();
        // team_1_entry.goals_scored += team_1_score;
        // team_1_entry.goals_conceded += team_2_score;
        
        // let team_2_entry = scores.entry(team_2_name).or_default();
        // team_2_entry.goals_scored += team_2_score;
        // team_2_entry.goals_conceded += team_1_score;

        let team_1_updated_scores = update_scores(scores, team_1_name, team_1_score, team_2_score);
        let team_2_updated_scores = update_scores(team_1_updated_scores, team_2_name, team_2_score, team_1_score);

        scores = team_2_updated_scores;
    }

    scores
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    const RESULTS: &str = "England,France,4,2
France,Italy,3,1
Poland,Spain,2,0
Germany,England,2,1
England,Spain,1,0";

    #[test]
    fn build_scores() {
        let scores = build_scores_table(RESULTS);

        assert!(["England", "France", "Germany", "Italy", "Poland", "Spain"]
            .into_iter()
            .all(|team_name| scores.contains_key(team_name)));
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 6);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 3);
    }
}
