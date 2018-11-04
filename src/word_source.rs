extern crate rand;

use self::rand::Rng;


const DICTIONARY:  &'static [&'static str] = &[
	"potato",
	"pomegranate",
	"sofa",
	"bed",
	"cabage"
];


pub fn get_random_word() -> &'static str {
	DICTIONARY[rand::thread_rng().gen_range(0, DICTIONARY.len())]
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_random_word() {
    	let word = get_random_word();

    	assert_eq!(true, DICTIONARY.iter().fold(false, |t, &candidate| {t || (word == candidate)}));
    }
}
