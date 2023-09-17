use regex::Regex;
use crate::{DataType, Decode, Encode, Error, MultiQRElement, QR, qr, QRType};
use crate::qr::QRData;
use bitcoin::bip32::{ExtendedPubKey as XPub, ExtendedPrivKey as XPriv};
use bitcoin::psbt::PartiallySignedTransaction as Psbt;
use liana::descriptors::LianaDescriptor as Descriptor ;

/// A decoder for Specter MultiQR
#[derive(Debug, Clone)]
pub struct SpecterQR {
    pub data: QRData,
}

impl SpecterQR {
    pub fn new() -> SpecterQR {
        let data = QRData::new();
        SpecterQR { data }
    }
}

impl Decode for SpecterQR {
    fn pattern() -> &'static str {
        r"^p\d+of\d+\s"
    }
    fn is_complete(&self) -> bool {
        self.data.is_completed
    }
    fn receive(&mut self, raw_data: &str) -> Result<bool, String> {
        if SpecterQR::is_multi(raw_data) {
            // header pattern
            let regex = Regex::new(SpecterQR::pattern())
                .unwrap();

            // fetch data
            let data = regex
                .replace_all(raw_data, "".to_string())
                .to_string();

            //fetch header
            let mut header: String = ""
                .to_string();
            if let Some(found) = regex
                .find(raw_data) {
                let found = found
                    .as_str()
                    .to_string();
                header = found;
            };
            let parts: Vec<&str> = header
                .split("of")
                .collect();

            // index
            let index: usize;
            let a = parts[0]
                .replace("p", "")
                .parse::<usize>();
            match a {
                Ok(value) => {
                    if value > 0 {
                        index = value;
                    } else {
                        return Err("Index cannot be 0!".to_string());
                    }
                }
                Err(e) => {
                    return Err(e.to_string());
                }
            }

            // total
            let total: usize;
            let b = parts[1]
                .trim()
                .parse::<usize>();
            match b {
                Ok(value) => {
                    if value > 1 {
                        total = value;
                    } else {
                        return Err("Total might be > 1!".to_string())
                    }
                }
                Err(e) => {
                    return Err(e.to_string());
                }
            }

            // if first append
            if self.data.chunks.len() == 0 {
                self.data.chunks = vec![None; total];
            }

            // load element to the right position
            let d2 = data.clone();
            let element = MultiQRElement {
                data: d2,
                index,
                total,
            };

            let idx = index - 1;

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
        } else {
            Err("data is not MultiQR type!".to_string())
        }
    }
    fn process(&mut self) {
        let mut buffer = String::new();
        // for each element of data chunks
        for i in &self.data.chunks {
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

    fn result() -> Result<DataType, Error> {
        todo!()
    }
}

impl Encode for SpecterQR {
    fn is_multi(data: &str) -> bool {
        let re: Regex = Regex::new(SpecterQR::pattern())
            .unwrap();
        re.is_match(data)
    }
    fn load_string(&mut self, data: &str, max_len: usize) -> Result<qr::QRData, Error> {
        let mut out = QRData::new();

        out.max_len = Some(max_len);
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
                if buff.len() == 0 {
                    end = true;
                }
            }
        } else {
            QR::data_init(&mut out, 1);
        }
        out.total_sequences = out.data_stack.len();
        out.is_loaded = true;
        Ok(out)
    }

    fn from_psbt(psbt: &Psbt) -> QRType {
        todo!()
    }

    fn from_xpub(xpub: &XPub) -> QRType {
        todo!()
    }

    fn from_xpriv(xpriv: &XPriv) -> QRType {
        todo!()
    }

    fn from_descriptor(descriptor: &Descriptor) -> QRType {
        todo!()
    }
}