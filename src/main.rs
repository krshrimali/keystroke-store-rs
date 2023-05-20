mod kafka_producer;
mod word_combinator;
// mod keystroke_fetcher;

fn main() {
    word_combinator::NextStep::start();
}
