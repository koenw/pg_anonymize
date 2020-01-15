use rand::prelude::*;
use lipsum::lipsum_words;

pub(crate) fn element<T>(source: &[T]) -> T where T: Clone {
    let mut rng = rand::thread_rng();
    source[rng.gen_range(0, source.len() - 1)].clone()
}

pub(crate) fn char() -> char {
    let alphabet = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    element(&alphabet)
}

pub(crate) fn date() -> String {
    let mut rng = rand::thread_rng();
    format!("{year}-{month:02}-{day:02} 00:00:00",
        year = rng.gen_range(1940, 2019), month = rng.gen_range(1, 12), day = rng.gen_range(1, 30))
}

pub(crate) fn given_name() -> &'static str {
    const NAMES: &'static [&str] = include!(concat!(env!("OUT_DIR"), "/given_names.rs"));
    const NAMES_LEN: usize = NAMES.len();

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0, NAMES_LEN - 1);
    NAMES[index]
}

pub(crate) fn surname() -> &'static str {
    const NAMES: &'static [&str] = include!(concat!(env!("OUT_DIR"), "/surnames.rs"));
    const NAMES_LEN: usize = NAMES.len();

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0, NAMES_LEN - 1);
    NAMES[index]
}

pub(crate) fn prose() -> String {
    let mut rng = rand::thread_rng();
    let no_words = rng.gen_range(5, 50);
    lipsum_words(no_words)
}

pub(crate) fn phone_nr() -> String {
    let mut rng = rand::thread_rng();
    let nr: u64 = rng.gen_range(1000000000,9999999999);
    format!("{}", nr)
}
