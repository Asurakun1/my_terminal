#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cotoba {
    pub word: String,
    pub reading: Vec<String>,
    pub definition: String,
}

impl Cotoba {
    pub fn new(word: String, reading: Vec<String>, definition: String) -> Self {
        Self {
            word,
            reading,
            definition,
        }
    }
    pub fn get_word(&self) -> &str {
        &self.word
    }
}
