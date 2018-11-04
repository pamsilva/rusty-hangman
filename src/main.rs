
mod utils;
mod word_source;

fn main() {
    println!("Hello, world!");

    let mut state = utils::init_state("potato");
    println!("The state is {:?}", state);

    state = utils::update_state(&state, "t");
    println!("The new state is {:?}", state);

    println!(" The random word is :	{:?}", word_source::get_random_word());
}
