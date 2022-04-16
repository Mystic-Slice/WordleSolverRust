const GAMES: &str = include_str!("../answerWords.txt");

fn main() {
    let w = WordleSolverRust::Wordle::new();
    for answer in GAMES.split_whitespace() {
        let guesser = WordleSolverRust::algorithms::Naive::new();
        w.play(answer, guesser);
    }
}