#![cfg_attr(debug_assertions, allow(unused))]

extern crate bitcoin;

use std::str::FromStr;
use crate::QRType::*;
use crate::qr::*;
use crate::specter::SpecterQR;
use bitcoin::psbt::PartiallySignedTransaction as Psbt;
use bitcoin::bip32::{ExtendedPubKey as XPub, ExtendedPrivKey as XPriv};
use liana::descriptors::LianaDescriptor as Descriptor ;
use crate::Error::ParsingError;

#[derive(Debug)]
pub enum Error {
    EncodingError(String),
    DecodingError(String),
    ParsingError(String),
    NotImplementedError(String),
}


pub mod qr;
pub mod specter;
pub mod ur;

/// Struct holding a chunk of MultiQR
#[derive(Debug, Clone)]
pub struct MultiQRElement {
    data: String,
    index: usize,
    total: usize,
}

/// Data Type
#[derive(Debug, Clone)]
pub enum DataType {
    Psbt(Option<Psbt>),
    Xpub(Option<XPub>),
    Xpriv(Option<XPriv>),
    Descriptor(Option<Descriptor>),
}

/// QR Type
#[derive(Debug, Clone)]
pub enum QRType {
    SimpleQR(QRData),
    Psbt,
    Xpriv,
    Xpub,
    Descriptor,
    Specter(SpecterQR),
    SpecterPsbt(),
    SpecterDescriptor,
    Ur,
    UrBytes,
    UrPsbt,
    UrXpub,
    UrXpriv,
    UrDescriptor,
    NoType,
}

/// A generic Trait for QRData Containers (single/multi QR)
pub trait QR {
    fn data_init(&mut self, sequences: usize);
    fn receive(&mut self, data: &String) -> bool;
}

///A generic trait for QRData Containers (multi)
pub trait MultiQR {
    fn check_complete(&mut self);
    fn next(&mut self) -> Result<String, String>;
}

/// Trait for decoders
pub trait Decode {
    /// return the pattern(regex) used for detect each type (decoder)
    fn pattern() -> &'static str;
    /// return true if decoding process ended (decoder)
    fn is_complete(&self) -> bool;
    /// load data chunk (decoder)
    fn receive(&mut self, data: &str) -> Result<bool, String>;
    /// decoding process (decoder)
    fn process(&mut self);

    fn result() -> Result<DataType, Error>;
}

/// Trait for encoders
pub trait Encode {
    /// return whether the string is a multiQR or not (encoder)
    fn is_multi(data: &str) -> bool;
    /// encode data from string (encoder)
    fn load_string(&mut self, data: &str, max_len: usize) -> Result<QRData, Error>;

    fn from_psbt(psbt: &Psbt) -> QRType;

    fn from_xpub(xpub: &XPub) -> QRType;

    fn from_xpriv(xpriv: &XPriv) -> QRType;

    fn from_descriptor(descriptor: &Descriptor) -> QRType;
}


/// A generic QRCode Encoder
#[derive(Debug, Clone)]
pub struct  QREncoder {
    encoder: QRType,
}

impl QREncoder {
    pub fn new() -> QREncoder {
        let encoder = QRType::NoType;
        QREncoder{encoder}
    }
    /// load (String) data into a given QRType with a given max(max_len) length of chunks
    pub fn load_str(&mut self, data: &str, qr_type: QRType, max_len: usize) -> Result<bool, Error> {
        match qr_type {
            Specter(_) => {
                // if multi => return a SpecterQR
                if data.len() > max_len {
                let mut specter_qr = SpecterQR::new();
                let encoded_data = <SpecterQR as Encode>::load_string(&mut specter_qr,data, max_len);
                specter_qr.data = encoded_data.map_err(|_| Error::EncodingError("Cannot encode specter_qr".to_string()))?;
                self.encoder = Specter(specter_qr);
                Ok(true)
                // else return a QRDat
                } else {
                    let mut simple_qr = QRData::new();
                    QR::receive(&mut simple_qr, &data.to_string());
                    self.encoder = SimpleQR(simple_qr);
                    Ok(true)
                }
            }
            _ => {Err(Error::NotImplementedError("Type not yet implemented!".to_string()))}
        }
    }

}

impl Encode for QREncoder {
    fn is_multi(data: &str) -> bool {
        todo!()
    }

    fn load_string(&mut self, data: &str, max_len: usize) -> Result<QRData, Error> {
        match self.encoder {
            QRType::Descriptor => {
                let imported_descriptor = Descriptor::from_str(data)
                    .map_err(|e| Error::ParsingError("Cannot load this string into descriptor".to_string()))?;

                Ok(QRData::new())
            }
            _ => { Err(Error::NotImplementedError("".to_string())) }
        }
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


#[cfg(test)]

#[test]
fn test_specter_iterate_qr() {
    let mut qr = SpecterQR::new();
    let mut data = qr.load_string("012345678901234567890123456789012345678", 13).unwrap();

    let _s = data.data_stack.len().to_string();
    for i in data.data_stack {
        match i {
            Some(result) => {
                println!("{result}")
            }
            None => {
                println!("None")
            }
        }
    }
}

#[test]
fn test_specter_datastack_qr() {
    let mut qr = SpecterQR::new();
    let mut data = SpecterQR::load_string(&mut qr,"012345678901234567890123456789012345678", 13).unwrap();

    let l = data.data_stack.len();
    assert_eq!(l, 3);

    let data = data.data_stack;

    assert_eq!(data[0], Some("0123456789012".to_string()));
    assert_eq!(data[1], Some("3456789012345".to_string()));
    assert_eq!(data[2], Some("6789012345678".to_string()));
}

#[test]
fn test_specter_next_qr() {
    let mut qr = SpecterQR::new();
    let mut data = SpecterQR::load_string(&mut qr,"012345678901234567890123456789012345678", 13).unwrap();
    let a = data.next()
        .unwrap();
    let b = data.next().unwrap();
    let c = data.next().unwrap();

    assert_eq!(a, "p1of3 0123456789012".to_string());
    assert_eq!(b, "p2of3 3456789012345".to_string());
    assert_eq!(c, "p3of3 6789012345678".to_string());
}

#[test]
fn test_specter_is_multi() {
    let data = "p1of3 jsdgkjdrghlkjkmj".to_string();
    let out = SpecterQR::is_multi(&data);
    assert!(out)
}

#[test]
fn test_specter_is_multi_false() {
    let data = "jsdgkjdrghlkjkmj".to_string();
    let out = SpecterQR::is_multi(&data);
    assert!(!out)
}

#[test]
fn test_append_specter_multi() {
    let mut multi = SpecterQR::new();

    Decode::receive(&mut multi, "p0of3 a");
    Decode::receive(&mut multi, "p1of0 a");
    Decode::receive(&mut multi, "p0of0 a");
    Decode::receive(&mut multi, "p1of3 a").unwrap();
    Decode::receive(&mut multi, "p2of3 b").unwrap();
    Decode::receive(&mut multi, "p3of3 c").unwrap();

    assert!(multi.is_complete());
    assert_eq!(multi.data.data, "abc".to_string());
}

#[test]
fn qr_encoder_load() {
    let mut qr_encoder = QREncoder::new();
    qr_encoder.load_str("213216546842lkljbjkhbvhgv5654", Specter(SpecterQR::new()), 13).unwrap();
}

