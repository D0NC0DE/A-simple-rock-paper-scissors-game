extern crate rust_beginner_projects;
use rust_beginner_projects::rock_paper_scissors;

// If you are on a macOS or Linux machine, try running:
//
// $ yes "rock" | cargo run --example rock_paper_scissors
// and then hit Ctrl-C after a few seconds
//
// to see how many times you can beat the computer by only using "rock"


fn main() {

    rock_paper_scissors::play();
}
