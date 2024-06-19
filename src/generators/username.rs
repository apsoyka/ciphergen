use rand::{random, thread_rng, Rng};
use rand::rngs::ThreadRng;
use rand::distributions::{Distribution, Standard};
use rand::seq::SliceRandom;

const VOWELS: [char; 6] = [
    'a', 'e', 'i',
    'o', 'u', 'y'
];

const CONSONANTS: [char; 20] = [
    'b', 'c', 'd', 'f', 'g',
    'h', 'j', 'k', 'l', 'm',
    'n', 'p', 'q', 'r', 's',
    't', 'v', 'w', 'x', 'z'
];

fn choose_random_vowel(rng: &mut ThreadRng) -> &char {
    VOWELS.choose(rng).unwrap()
}

fn choose_random_consonant(rng: &mut ThreadRng) -> &char {
    CONSONANTS.choose(rng).unwrap()
}

fn add_vowel(input: &mut Vec<char>, rng: &mut ThreadRng) {
    let value = choose_random_vowel(rng);

    input.push(*value);
}

fn add_consonant(input: &mut Vec<char>, rng: &mut ThreadRng) {
    let value = choose_random_consonant(rng);

    input.push(*value);
}

fn create_closed_syllable(rng: &mut ThreadRng) -> Vec<char> {
    vec![
        *choose_random_consonant(rng),
        *choose_random_vowel(rng),
        *choose_random_consonant(rng)
    ]
}

fn create_open_syllable(rng: &mut ThreadRng) -> Vec<char> {
    vec![
        *choose_random_consonant(rng),
        *choose_random_vowel(rng)
    ]
}

enum SyllableType {
    CLOSED,
    OPEN
}

impl Distribution<SyllableType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SyllableType {
        match rng.gen_range(0..=1) {
            0 => SyllableType::CLOSED,
            _ => SyllableType::OPEN
        }
    }
}

/// Generate a simple pronounceable username by alternating random vowels and consonants.
///
/// Usernames created in this fashion are guaranteed to be pronouncable,
/// but are likely to be flagged as suspicious by automated tools and may not be aesthetically pleasing.
pub fn generate_simple_username(capitalize: bool, length: usize) -> Vec<u8> {
    if length == 0 { return Vec::<u8>::new(); }

    let mut output: Vec<char> = Vec::new();
    let rng = &mut thread_rng();
    let start = rng.gen_bool(1.0 / 2.0);

    // Flip a coin to choose between starting with a vowel or a consonant.
    match start {
        true => { add_vowel(&mut output, rng); }
        false => { add_consonant(&mut output, rng); }
    }

    // If only one character is needed, then we are done.
    if length == 1 { return output.iter().collect::<String>().into_bytes(); }

    // Alternate between adding consonants and vowels
    for index in 0..(length - 1) {
        // If we started with a vowel, then we must add a consonant next, and vice-versa.
        match start {
            true => if index % 2 != 0 { add_vowel(&mut output, rng); } else { add_consonant(&mut output, rng); }
            false => if index % 2 != 0 { add_consonant(&mut output, rng); } else { add_vowel(&mut output, rng); }
        }
    }

    if capitalize { output[0].make_ascii_uppercase(); }

    output.iter()
        .collect::<String>()
        .into_bytes()
}

/// Generate a pronounceable username from random syllables.
///
/// Syllabic usernames are less likely to be flagged as suspicious by automated tools,
/// and may be more aesthetically pleasing.
pub fn generate_complex_username(capitalize: bool, length: usize) -> Vec<u8> {
    if length == 0 { return Vec::<u8>::new(); }

    let rng = &mut thread_rng();
    let mut output = Vec::<char>::new();

    for _ in 0..length {
        // Generate a random syllable of a random type.
        let syllable = match random::<SyllableType>() {
            SyllableType::CLOSED => create_closed_syllable(rng),
            SyllableType::OPEN => create_open_syllable(rng)
        };

        output.extend(syllable);
    }

    if capitalize { output[0].make_ascii_uppercase(); }

    output.iter()
        .collect::<String>()
        .into_bytes()
}

#[cfg(test)]
mod tests {
    use std::str::from_utf8;

    use super::{generate_simple_username, generate_complex_username};

    #[test]
    fn generates_ten_thousand_character_simple_username() {
        let bytes = generate_simple_username(false, 10000);
        let string = from_utf8(&bytes).unwrap();

        assert_eq!(string.chars().count(), 10000)
    }

    #[test]
    fn generates_empty_simple_username() {
        let bytes = generate_simple_username(false, 0);

        assert_eq!(bytes.len(), 0)
    }

    #[test]
    fn generates_ten_thousand_syllable_complex_username() {
        let bytes = generate_complex_username(false, 10000);
        let string = from_utf8(&bytes).unwrap();
        let length = string.chars().count();
        let range = 20000..=30000;

        assert!(range.contains(&length), "expected a number in {:?}, but got {} instead", range, length);
    }

    #[test]
    fn generates_empty_complex_username() {
        let bytes = generate_complex_username(false, 0);

        assert_eq!(bytes.len(), 0)
    }
}
