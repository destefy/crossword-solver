use std::io::BufRead;

pub struct Dict{
    trie: trie_rs::Trie<u8>,
    word_list: Vec<String>,
}

impl Dict{
    pub fn new(filepath: String) -> Self {
        let word_list = Self::load_into_list(filepath);
        let trie = Self::load_into_dict(&word_list);
        Dict {trie, word_list}
    }

    pub fn get_trie(&self) -> &trie_rs::Trie<u8> {
        &self.trie
    }

    pub fn get_word_list(&self) -> &Vec<String> {
        &self.word_list
    }

    pub fn load_into_list(filepath: String) -> Vec<String> {
        let file = std::fs::File::open(filepath).expect("Failed to open file");
        let reader = std::io::BufReader::new(file);
        
        reader.lines()
            .map(|line| line.expect("Failed to read line"))
            .collect()
    }

    pub fn load_into_dict(word_list: &Vec<String>) -> trie_rs::Trie<u8> {
        let mut builder = trie_rs::TrieBuilder::<u8>::new();
        for word in word_list {
            builder.push(word)
        }
        builder.build()
    }
}
