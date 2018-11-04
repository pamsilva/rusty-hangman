#[derive(Debug)]
pub struct GameState {
	attempts: i8,
	word: &'static str,
	visibility: Vec<bool>
}

pub fn init_state(word: &'static str) -> GameState {
	GameState {
		attempts: 9,
		word: word,
		visibility: vec![false; word.len()]
	}
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn proper_initial_state() {
    	let init_state = init_state("potato"); 
	    assert_eq!(9, init_state.attempts);
	    assert_eq!("potato", init_state.word);
	    assert_eq!(6, init_state.visibility.len());
	}

}
