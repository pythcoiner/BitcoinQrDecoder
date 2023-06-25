// use std::fmt::format;
#[cfg(test)]

use regex::Regex;

// TODO: handle exceptions instead println!

#[derive(Clone)]
struct MultiQRElement {
    data: String,
    index: usize,
    total: usize,
}

trait QRDataSTD {
    fn pattern() -> &'static str;
    fn is_multi(data: &str) -> bool;
    fn is_complete(& self) -> bool;
    fn fetch_multi(data: &str) -> Option<MultiQRElement>;
    fn from_string(data: &str, max_len: usize) -> QRData;
    fn append(&mut self,data: &str) -> Result<bool, String>;
    fn process(&mut self);
}

struct SpecterQR{
    data: QRData,
}

impl SpecterQR{
    fn new() -> SpecterQR{
        let data = QRData::new();
        SpecterQR{data}
    }
}

impl QRDataSTD for SpecterQR {
    fn pattern() -> &'static str {
        r"^p\d+of\d+\s"
    }
    fn is_multi(data: &str) -> bool {
        let re: Regex = Regex::new(SpecterQR::pattern()).unwrap();
        re.is_match(data)
    }
    fn is_complete(& self) -> bool {
        self.data.is_completed
    }
    fn fetch_multi(data: &str) -> Option<MultiQRElement> {
        if SpecterQR::is_multi(data) {
            let re: Regex = Regex::new(SpecterQR::pattern()).unwrap();
            let header = re.find(data);
            let elements = match header {
                Some(result) => {
                    let data: String = result.as_str().trim().to_string();
                    let elmts: Vec<&str> = data.split("of").collect();
                    let index: usize = match elmts[0].trim_start_matches('p').parse::<usize>() {
                        Ok(i) => i,
                        Err(_) => {
                            println!("Cannot convert index element to usize!");
                            0
                        }
                    };
                    let total = match elmts[1].parse::<usize>() {
                        Ok(t) => t,
                        Err(_) => {
                            println!("Cannot convert total elements to usize!");
                            0
                        }
                    };
                    (index, total)
                }
                None => {
                    println!("Failed to fetch elements from SpecterMulti data in Specter::fetch_specter_multi");
                    (0, 0)
                }
            };
            let out_data: String = re.replace(data, "").to_string();
            if elements.0 <= elements.1 && elements.0 > 0 && elements.1 > 0 {
                Some(MultiQRElement {
                    data: out_data,
                    index: elements.0,
                    total: elements.1,
                })
            } else {
                None
            }
        } else {
            None
        }
    }
    fn from_string(data: &str, max_len: usize) -> QRData {
        let mut out = QRData::new();

        out.max_len = max_len;
        out.data = data.to_string();

        // if multi
        if data.len() > max_len {
            let mut end: bool = false;
            let mut buff = data.to_string();

            //data.len() > max_len : split and load
            while !end {
                //if len(data) > max_len : return (chunk, data) else return (data, None)
                let (sequence, data) = if buff.len() > 0 {
                    if buff.len() > max_len {
                        let (sequence, data) = buff.split_at(max_len);
                        // (chunk, data)
                        (sequence.to_owned().to_string(), data.to_owned().to_string())
                    } else {
                        // (data, None)
                        (buff, "".to_string())
                    }
                } else {
                    let sequence = data.clone().to_string();
                    // (data, None)
                    (sequence, "".to_string())
                };

                // load remaining data in buff
                buff = data.clone().to_string();

                // QR::append(&mut out, &sequence.to_string());
                let option = Some(sequence.to_string());
                out.data_stack.push(option);

                // stop when buffer empty
                if buff.len() == 0 { end = true;}
            }
        } else {
            QR::data_init(&mut out, 1);
        }
        out.total_sequences = out.data_stack.len();
        out.is_loaded = true;
        out
    }
    fn append(&mut self, raw_data: &str) -> Result<bool, String> {
        if SpecterQR::is_multi(raw_data){
            // header pattern
            let regex = Regex::new(SpecterQR::pattern()).unwrap();

            // fetch data
            let data = regex.replace_all(raw_data, "".to_string()).to_string();

            //fetch header
            let mut header: String = "".to_string();
            if let Some(found) = regex.find(raw_data){
                let found = found.as_str().to_string();
                header = found;
            };
            let parts: Vec<&str> = header.split("of").collect();

            // index
            let index: usize;
            let a = parts[0].replace("p", "")
                            .parse::<usize>();
            match a {
                Ok(value) => {
                    if value > 0{
                        index = value;
                    } else {return Err("Index cannot be 0!".to_string());}
                }
                Err(e) => {return Err(e.to_string());}
            }

            // total
            let total: usize;
            let b = parts[1].trim()
                            .parse::<usize>();
            match b {
                Ok(value) => {
                    if value > 1 {
                        total = value;
                    } else {return Err("Total might be > 1!".to_string());}
                }
                Err(e) => { return Err(e.to_string());}
            }

            // if first append
            if self.data.chunks.len() == 0 {
                self.data.chunks = vec![None; total];
            }

            // load element to the right position
            let d2 = data.clone();
            let element = MultiQRElement{data: d2, index, total };

            let idx = index -1;

            // check if this chunk already loaded
            match &self.data.chunks[idx] {
                None => {
                    // load the chunk
                    self.data.chunks[idx] = Some(element);
                }
                Some(value) => {
                    // check if loaded & actual value match
                    if value.data != data {
                        return Err("Value and data are different!".to_string());
                    }
                }
            }

            self.process();

            Ok(true)
        }
        else { Err("data is not MultiQR type!".to_string()) }
    }
    fn process(&mut self) {
        let mut buffer = String::new();
        // for each element of data chunks
        for i in &self.data.chunks{
            match i {
                Some(result) => {
                    buffer += &result.data;
                }
                None => {
                    self.data.is_completed = false;
                    return;
                }

            }
        }
        self.data.data = buffer;
        self.data.is_completed = true;

    }
}

enum QRType {
    QRType,
    MultiQRType,
}

/// A generic Trait for QRData Containers (single/multi QR)
trait QR {
    // fn load(&mut self, data: &String);
    fn data_init(&mut self, sequences: usize);
    fn append(&mut self, data: &String) -> bool;
}

trait MultiQR {
    fn check_complete(&mut self);
    fn next(&mut self) -> Result<String, String>;
}

/// A Generic container for QRCode data
struct QRData {
    qr_type: QRType,
    data: String,
    total_sequences: usize,
    sequences_count: usize,
    is_completed: bool,
    is_loaded: bool,
    is_init: bool,
    current: usize,
    data_stack: Vec<Option<String>>,
    chunks: Vec<Option<MultiQRElement>>,
    max_len: usize ,
}

impl QRData {
    fn new() -> QRData {
        QRData {
            qr_type: QRType::QRType,
            data: String::new(),
            total_sequences: 0,
            sequences_count: 0,
            is_completed: false,
            is_loaded: false,
            is_init: false,
            current: 0,
            data_stack: vec![],
            chunks: vec![],
            max_len: 0,
        }
    }

}

impl QR for QRData {
    ///  Initialize QRData Container
    ///
    fn data_init(&mut self, sequences: usize) {
        self.total_sequences = sequences;
        self.sequences_count = 0;
        self.current = sequences;
        self.is_loaded = true;
    }

    ///  Append data from a single QRCode received without formatting
    ///
    fn append(&mut self, data: &String) -> bool {
        self.data = data.clone();
        self.sequences_count = 1;
        self.total_sequences = 1;
        self.current = 1;
        self.is_loaded = true;
        true
    }
    
}

impl MultiQR for QRData {
    fn check_complete(&mut self) {
        let mut fill_sequences: usize = 0;
        for sequence in &self.data_stack {
            if let Some(_result) = sequence {
                fill_sequences += 1;
            }
        }
        self.sequences_count = fill_sequences;
        if fill_sequences == self.total_sequences{
            self.is_completed = true;
        }
    }

    fn next(&mut self) -> Result<String, String> {
        if self.is_loaded {
            if self.current >= self.total_sequences {
                self.current = 0;
            }
            if let Some(result) = &self.data_stack[self.current] {
                let a  = (self.current + 1).to_string();
                let b = self.total_sequences.to_string();
                let c = "p".to_string() + &a + &"of".to_string() +  &b + &" ".to_string();
                let d = result;
                let out = c + d;
                self.current += 1;
                Ok(out)
            } else {Err("data_stack element is None".to_string())}

        } else {
            Err("QRData not yet loaded!".to_string())
        }
    }

}

fn main() {


}

#[test]
fn test_specter_iterate_qr() {
    let qr = SpecterQR::from_string("012345678901234567890123456789012345678", 13);

    let s = qr.data_stack.len().to_string();
    println!("data_stack.len()={s}");
    for i in qr.data_stack{
        match i {
            Some(result) => {
                println!("{result}")
            },
            None => {
                println!("None")
            },
        }
    }

}

#[test]
fn test_specter_datastack_qr() {
    let qr = SpecterQR::from_string("012345678901234567890123456789012345678", 13);

    let l = qr.data_stack.len();
    assert_eq!(l, 3);

    let data = qr.data_stack;

    assert_eq!(data[0], Some("0123456789012".to_string()));
    assert_eq!(data[1], Some("3456789012345".to_string()));
    assert_eq!(data[2], Some("6789012345678".to_string()));

}

#[test]
fn test_specter_next_qr(){
    let mut qr = SpecterQR::from_string("012345678901234567890123456789012345678", 13);
    let a = qr.next().unwrap();
    let b = qr.next().unwrap();
    let c = qr.next().unwrap();


    assert_eq!(a, "p1of3 0123456789012".to_string());
    assert_eq!(b, "p2of3 3456789012345".to_string());
    assert_eq!(c, "p3of3 6789012345678".to_string());
}

#[test]
fn test_specter_is_multi(){
    let data = "p1of3 jsdgkjdrghlkjkmj".to_string();
    let out = SpecterQR::is_multi(&data);
    assert!(out)
}

#[test]
fn test_specter_is_multi_false(){
    let data = "jsdgkjdrghlkjkmj".to_string();
    let out = SpecterQR::is_multi(&data);
    assert!(!out)
}

#[test]
fn test_append_specter_multi(){
    let mut multi = SpecterQR::new();

    QRDataSTD::append(&mut multi, "p0of3 a");
    QRDataSTD::append(&mut multi, "p1of0 a");
    QRDataSTD::append(&mut multi, "p0of0 a");
    QRDataSTD::append(&mut multi, "p1of3 a").unwrap();
    QRDataSTD::append(&mut multi, "p2of3 b").unwrap();
    QRDataSTD::append(&mut multi, "p3of3 c").unwrap();

    assert!(multi.is_complete());
    assert_eq!(multi.data.data, "abc".to_string());

}





