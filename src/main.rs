use anagram_solver::Config;
use std::env;

// TODO: Work with traits etc. Beautify it.
// Introduce a second argument. Slow & Fast. Slow will be trying all combinations against a HashMap.
fn main() {
    // collect the args.
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    anagram_solver::run(config);
}
