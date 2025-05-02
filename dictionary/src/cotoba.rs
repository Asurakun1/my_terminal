#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Cotoba {
    pub word: String,
    pub reading: Vec<String>,
    pub definition: String,
}

impl Cotoba {
    pub fn new() -> Self {
        Self {
            word: String::from(""),
            reading: vec![],
            definition: String::from(""),
        }
    }

    pub fn get_word(&self) -> &str {
        &self.word
    }

    pub fn get_reading(&self) -> &Vec<String> {
        &self.reading
    }

    pub fn get_definition(&self) -> &str {
        &self.definition
    }

    pub fn set_word(&mut self, word: &str) {
        self.word = word.to_string();
    }

    pub fn set_reading(&mut self, reading: &str) {
        self.reading = reading
            .split(",")
            .map(|str| str.trim().to_string())
            .collect();
    }

    pub fn set_definition(&mut self, definition: &str) {
        self.definition = definition.to_string();
    }
}
