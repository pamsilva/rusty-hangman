
mod state;
mod word_source;

fn main() {
    println!("Hello, world!");

    let mut state = state::init_state("potato");
    println!("The state is {:?}", state);

    state = state::update_state(&state, "t");
    println!("The new state is {:?}", state);

    println!(" The random word is :	{:?}", word_source::get_random_word());

    println!(" The visibility is :	{:?}", state::derive_visible_word(&state));
}
