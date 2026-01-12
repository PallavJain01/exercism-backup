use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
	let mut anagrams = HashSet::<&'a str>::new();

	let word_lower = word.to_lowercase();
	for anag in possible_anagrams {
		if anag.chars().count() != word.chars().count() { continue; } // check for equal length

		let anag_lower = anag.to_lowercase();

		if anag_lower == word_lower { continue;	} // check for not matching with same word

		// sorting characters, if the output of both is equal, then they are anagrams, as we already checked lengths
		let mut anag_lower_vec = anag_lower.chars().collect::<Vec<_>>();
		let mut word_lower_vec = word_lower.chars().collect::<Vec<_>>();

		anag_lower_vec.sort();
		word_lower_vec.sort();

		if anag_lower_vec == word_lower_vec {
			anagrams.insert(anag);
		}
	}

	anagrams
}
