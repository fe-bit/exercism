struct HighScores {
    scores: Vec<u32>,
}

struct HighScore {
    score: u32,
}

impl HighScores {
    fn new() -> HighScores {
        HighScores {
            scores: vec![1, 5, 6, 7, 2, 10],
        }
    }

    fn highest_score(&self) -> &u32 {
        self.scores.iter().max().unwrap()
    }

    fn last_added_scores(&self) -> Vec<&u32> {
        let l = self.scores.len();
        vec![
            &self.scores[l - 1],
            &self.scores[l - 2],
            &self.scores[l - 3],
        ]
    }

    fn three_highest_scores(&self) -> Vec<&u32> {
        let mut results = vec![];
        for score in &self.scores {
            results.push(score);
        }
        results.sort_by(|a, b| b.cmp(a));
        vec![results[0], results[1], results[2]]
    }
}

fn main() {
    let highscores = HighScores::new();
    assert_eq!(highscores.highest_score(), &10);
    assert_eq!(highscores.last_added_scores(), vec![&10, &2, &7]);
    assert_eq!(highscores.three_highest_scores(), vec![&10, &7, &6]);
}
