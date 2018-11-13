#[derive(Debug)]
pub struct GameState {
	pub attempts: i8,
	pub word: Vec<char>,
	pub visibility: Vec<bool>
}


pub fn init_state(word: &'static str) -> GameState {
	GameState {
		attempts: 9,
		word: word.chars().collect(),
		visibility: vec![false; word.len()]
	}
}


pub fn update_state(state: &GameState, guess: &str) -> GameState {

	if guess.len() > 1 {
		if state.word.iter().collect::<String>() == guess {
			return GameState{
				attempts: state.attempts -1,
				word: state.word.to_vec(),
				visibility: vec![true; state.word.len()]
			}
		} else {
			return GameState{
				attempts: state.attempts -1,
				word: state.word.to_vec(),
				visibility: state.visibility.to_vec()
			}
		}

	} else {
		let mut new_visibility = vec![false; state.word.len()];
		let guess_vec: Vec<char> = guess.chars().collect();

		for i in 0..state.word.len() {
			new_visibility[i] = state.visibility[i] || state.word[i] == guess_vec[0]
		}

		return GameState{
			attempts: state.attempts -1,
			word: state.word.to_vec(),
			visibility: new_visibility
		}
	}
}


pub fn derive_visible_word(state: &GameState) -> String {
	let mut res = Vec::new();

	for i in 0..state.word.len() {
		if state.visibility[i] {
			res.push(state.word[i]);
		} else {
			res.push('_');
		}
	}

	res.iter().collect::<String>()
}


pub fn has_guessed_the_word(state: &GameState) -> bool {
	state.visibility.iter().fold(true, |res, val| res & val)
}


pub fn display_state(state: &GameState) {
	println!(
		"The word {}, and you have {} attempts.", 
		derive_visible_word(&state),
		state.attempts
	)
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn validate_has_guessed_the_word() {
    	let before = init_state("potato");
    	assert_eq!(false, has_guessed_the_word(&before));

    	let after = update_state(&before, "o");
    	assert_eq!(false, has_guessed_the_word(&after));

    	let last = update_state(&after, "potato");
    	assert_eq!(true, has_guessed_the_word(&last));
    }

    #[test]
    fn proper_initial_state() {
    	let init_state = init_state("potato");
	    
	    assert_eq!(9, init_state.attempts);
	    assert_eq!("potato".chars().collect::<Vec<char>>(), init_state.word);
	    assert_eq!(init_state.word.len(), init_state.visibility.len());
	}

	#[test]
	fn update_state_guess_char() {
		let dummy_word = "potato"; 

		let before = GameState {
			attempts: 3,
			word: dummy_word.chars().collect(),
			visibility: vec![false; dummy_word.len()]
		};

		let after = update_state(&before, "t");

		assert_eq!(after.attempts, before.attempts - 1);
		assert_eq!(after.word, before.word);
		assert_eq!(after.visibility, [false, false, true, false, true, false]);
	}

	#[test]
	fn update_state_wrong_char() {
		let dummy_word = "potato"; 

		let before = GameState {
			attempts: 2,
			word: dummy_word.chars().collect(),
			visibility: vec![false; dummy_word.len()]
		};

		let after = update_state(&before, "b");

		assert_eq!(after.attempts, before.attempts - 1);
		assert_eq!(after.word, before.word);
		assert_eq!(after.visibility, before.visibility);
	}

	#[test]
	fn update_state_guess_word() {
		let dummy_word = "potato"; 

		let before = GameState {
			attempts: 2,
			word: dummy_word.chars().collect(),
			visibility: vec![false; dummy_word.len()]
		};

		let after = update_state(&before, "potato");

		assert_eq!(after.attempts, before.attempts - 1);
		assert_eq!(after.word, before.word);
		assert_eq!(after.visibility, vec![true; dummy_word.len()]);
	}

	#[test]
	fn update_state_wrong_word() {
		let dummy_word = "potato";

		let before = GameState {
			attempts: 2,
			word: dummy_word.chars().collect(),
			visibility: vec![false; dummy_word.len()]
		};

		let after = update_state(&before, "botato");

		assert_eq!(after.attempts, before.attempts - 1);
		assert_eq!(after.word, before.word);
		assert_eq!(after.visibility, before.visibility);
	}

	#[test]
	fn validate_visible_word() {
		let dummy_word = "potato";

		let test_state = GameState {
			attempts: 2,
			word: dummy_word.chars().collect(),
			visibility: vec![false, true, false, true, false, true]
		};

		let res = derive_visible_word(&test_state);

		assert_eq!(res, "_o_a_o");
	}

}
