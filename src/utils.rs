
pub fn init_state() -> i8 {
	4
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn proper_initial_state() {
	    assert_eq!(4, init_state());
	}

}
