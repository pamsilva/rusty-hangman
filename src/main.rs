
mod state;
mod word_source;
mod input;

fn play(mut _state: state::GameState) {
	while _state.attempts > 0 && !state::has_guessed_the_word(&_state){
		state::display_state(&_state);
		let guess = input::take_user_guess();

		_state = state::update_state(&_state, &guess);

		println!("guess {:?}", guess);
	}

	if state::has_guessed_the_word(&_state) {
		println!(
			"Congratulations, you guessed {} in {} attempts",
			_state.word.iter().collect::<String>(),
			9 - _state.attempts 
		);
	} else {
		println!(
			"Too bad you failed to guess the word, but got to {}...",
			state::derive_visible_word(&_state) 
		);
	}
}


fn main() {
    println!("Welcome to rusty hangman. Just hangman implemented in rust.");

    let state = state::init_state(word_source::get_random_word());
    play(state);
}
