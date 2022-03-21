fn main() {
    fn main() {
        use std::collections::HashMap;
        let vec = Vec::new();
        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];

        let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    }
}
