mod utils;


fn main() {
    println!("Hello, world!");

    let mut state = utils::init_state("potato");
    println!("The state is {:?}", state);

    state = utils::update_state(&state, "t");
    println!("The new state is {:?}", state);


}
