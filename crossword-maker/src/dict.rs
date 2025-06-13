use std::io::BufRead;

pub fn load_into_dict(filepath: String) -> trie_rs::Trie<u8> {
    let mut builder = trie_rs::TrieBuilder::<u8>::new();
    let file = std::fs::File::open(filepath).expect("Failed to open file");
    let reader = std::io::BufReader::new(file);
    
    for line in reader.lines() {
        let word = line.expect("Failed to read line");
        builder.push(word)
    }
    
    builder.build()
}