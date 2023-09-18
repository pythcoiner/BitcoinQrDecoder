use crate::MultiQRElement;

/// A Generic container for QRCode data
#[derive(Debug, Clone)]
pub struct QRData {
    pub data: String,
    pub total_sequences: usize,
    sequences_count: usize,
    pub is_completed: bool,
    pub is_loaded: bool,
    is_init: bool,
    current: usize,
    pub data_stack: Vec<Option<String>>,
    pub chunks: Vec<Option<MultiQRElement>>,
    pub max_len: Option<usize>,
}

impl QRData {
    pub fn new() -> QRData {
        QRData {
            data: String::new(),
            total_sequences: 0,
            sequences_count: 0,
            is_completed: false,
            is_loaded: false,
            is_init: false,
            current: 0,
            data_stack: vec![],
            chunks: vec![],
            max_len: None,
        }
    }
}

impl  QRData {
    ///  Initialize QRData Container
    ///
    pub fn data_init(&mut self, sequences: usize) {
        self.total_sequences = sequences;
        self.sequences_count = 0;
        self.current = sequences;
        self.is_loaded = true;
    }

    ///  Append data from a single QRCode received without formatting
    ///
    pub fn receive(&mut self, data: &String) -> bool {
        self.data = data.clone();
        self.sequences_count = 1;
        self.total_sequences = 1;
        self.current = 1;
        self.is_loaded = true;
        true
    }

    pub fn check_complete(&mut self) {
        let mut fill_sequences: usize = 0;
        for sequence in &self.data_stack {
            if let Some(_result) = sequence {
                fill_sequences += 1;
            }
        }
        self.sequences_count = fill_sequences;
        if fill_sequences == self.total_sequences {
            self.is_completed = true;
        }
    }

    pub fn next(&mut self) -> Result<String, String> {
        if self.is_loaded {
            if self.current >= self.total_sequences {
                self.current = 0;
            }
            if let Some(result) = &self.data_stack[self.current] {
                let a = (self.current + 1).to_string();
                let b = self.total_sequences.to_string();
                let c = "p".to_string() + &a + &"of".to_string() + &b + &" ".to_string();
                let d = result;
                let out = c + d;
                self.current += 1;
                Ok(out)
            } else {
                Err("data_stack element is None".to_string())
            }
        } else {
            Err("QRData not yet loaded!".to_string())
        }
    }
}