struct Codec;

impl Codec {
    pub fn encode(&self, strs: &[String]) -> String {
            let mut result = String::new();
            for s in strs {
            result.push_str(&s.len().to_string());
            result.push('#');
            result.push_str(&s);
        }
        result
    }

    pub fn decode(&self, s: &String) -> Vec<String> {
        let mut result = Vec::new();
        let mut remaining = s.as_str();

        while !remaining.is_empty() {
            let hash_index = remaining.find('#').unwrap();
            let length: usize = remaining[..hash_index].parse().unwrap();
            remaining = &remaining[hash_index + 1..];
            let word = &remaining[..length];
            result.push(word.to_string());
            remaining = &remaining[length..];
        }

        result
    }
}