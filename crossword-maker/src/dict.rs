use std::io::BufRead;
use std::collections::HashMap;

type TrieMap = HashMap<usize, trie_rs::Trie<u8>>;

pub struct Dict{
    word_list: Vec<String>,
    tries: TrieMap,
}

impl Dict{
    pub fn new(filepath: String, side_len: usize) -> Self {
        let word_list: Vec<String> = Self::load_into_list(filepath);
        let tries: TrieMap = Self::load_into_tries(&word_list, side_len);
        Dict {word_list, tries}
    }

    pub fn get_trie(&self, key: usize) -> &trie_rs::Trie<u8> {
        assert !(self.tries.contains_key(&key), "Trie for key {} does not exist", key);
        return &self.tries[&key]
    }

    #[allow(dead_code)]
    pub fn print_trie(&self, key: usize) {
        assert!(self.tries.contains_key(&key), "Trie for key {} does not exist", key);
        let trie = &self.tries[&key];
        println!("Trie for key {}: {:?}", key, trie.predictive_search("").collect::<Vec<String>>());
    }

    pub fn get_word_list(&self) -> &Vec<String> {
        return &self.word_list;
    }

    pub fn get_word(&self, index: &usize) -> &String {
        assert!(*index < self.word_list.len(), "Index {} out of bounds for word list", index);
        return &self.word_list[*index];
    }

    pub fn load_into_list(filepath: String) -> Vec<String> {
        let file = std::fs::File::open(filepath).expect("Failed to open file");
        let reader = std::io::BufReader::new(file);
        
        reader.lines()
            .map(|line| line.expect("Failed to read line"))
            .collect()
    }

    // Create many tries such that like:
    // For the word "apple", we have a trie for "apple", "ple", and "e"
    pub fn load_into_tries(word_list: &Vec<String>, side_len: usize) -> TrieMap {    
        let num_tries: usize = side_len / 2 + side_len % 2;
        
        // First build up the vocabulary for the tries
        let mut trie_builders = vec![trie_rs::TrieBuilder::<u8>::new(); num_tries];
        let word_len = word_list[0].len();
        for word in word_list {
            for i in 0..side_len/2 {
                trie_builders[i].push(word[i*2..].to_string())
            }
            if side_len % 2 == 1 {
                trie_builders[num_tries - 1].push(word[(word_len - 1)..].to_string());
            }
        }

        // Then make them easily accessible
        // tries[i] = trie for words starting at row i
        let mut tries: TrieMap = TrieMap::new();
        for row_i in 0..(side_len/2){
            tries.insert(row_i * 2, trie_builders[row_i].clone().build());
        }
        if side_len % 2 == 1 {
            tries.insert(side_len - 1, trie_builders[num_tries - 1].clone().build());
        }
        return tries;
    }
}
