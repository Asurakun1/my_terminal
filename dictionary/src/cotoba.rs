#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Cotoba {
    pub word: String,
    pub reading: Vec<String>,
    pub definition: Vec<String>,
}

impl Cotoba {
    pub fn new() -> Self {
        Self {
            word: String::from(""),
            reading: vec![],
            definition: vec![],
        }
    }

    pub fn get_word(&self) -> &str {
        &self.word
    }

    pub fn get_reading(&self) -> &Vec<String> {
        &self.reading
    }

    pub fn get_definition(&self) -> &[String] {
        &self.definition
    }

    pub fn set_word(&mut self, word: &str) {
        self.word = word.to_string();
    }

    pub fn set_reading(&mut self, reading: &str) {
        self.reading = reading
            .split("　")
            .map(|str| str.trim().to_string())
            .collect();
    }

    pub fn set_definition(&mut self, definition: &[String]) {
        self.definition = definition.to_vec();
    }
}
