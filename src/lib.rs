#![cfg_attr(debug_assertions, allow(unused))]

extern crate bitcoin;

pub mod qr;
pub mod specter;
pub mod ur;

use crate::qr::*;
use crate::specter::SpecterQR;
use crate::Error::ParsingError;
use crate::OutputType::*;
use bitcoin::bip32::{ExtendedPrivKey as XPriv, ExtendedPubKey as XPub};
use bitcoin::psbt::PartiallySignedTransaction as Psbt;
use liana::descriptors::LianaDescriptor;
use std::str::FromStr;

#[derive(Debug)]
pub enum Error {
    EncodingError(String),
    DecodingError(String),
    ParsingError(String),
    NotImplementedError(String),
}

/// Struct holding a chunk of MultiQR
#[derive(Debug, Clone, PartialEq)]
pub struct MultiQRElement {
    data: String,
    index: usize,
    total: usize,
}

/// Data Type
#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Address(),
    Psbt(Option<Psbt>),
    Xpub(Option<XPub>),
    Xpriv(Option<XPriv>),
    LianaDescriptor(Option<LianaDescriptor>),
    Descriptor(),
    NoType(Option<String>),
}

#[derive(Debug, Clone)]
pub enum Encoding {
    Raw,
    Specter,
    Ur,
    NotSelected,
}

/// QR Type
#[derive(Debug, Clone, PartialEq)]
pub enum OutputType {
    /// Raw QRCode (no data typing)
    SimpleQR(QRData),
    /// Raw PSBT
    Psbt,
    /// Raw Private key
    Xpriv,
    /// Raw Xpub
    Xpub,
    ///Raw Liana descriptor
    LianaDescriptor,
    ///Raw Bitcoin descriptor
    Descriptor,
    /// Specter animated QRCode (no data typing)
    Specter(SpecterQR),
    /// Specter animated PSBT
    SpecterPsbt,
    /// Specter animated Descriptor
    SpecterDescriptor,
    /// UR encoded QRCode (no data typing)
    Ur,
    /// UR encoded as BYTES QRCode
    UrBytes,
    /// UR encoded as crypto-psbt QRCode
    UrPsbt,
    /// UR encoded as crypto-pubkeys QRCode
    UrXpub,
    /// UR encoded as crypto-prvkeys QRCode
    UrXpriv,
    /// UR encoded as crypto-output QRCode
    UrDescriptor,
    /// UR encoded as crypto-address QRCode
    UrAddress,
    /// Encoding not selected
    NoType,
}

/// Trait for decoders
pub trait Decode {
    /// return the pattern(regex) used for detect each type (decoder)
    fn pattern() -> &'static str;
    /// return true if decoding process ended (decoder)
    fn is_complete(&self) -> bool;
    /// load data chunk (decoder)
    fn receive(&mut self, data: &str) -> Result<bool, Error>;
    /// result of decoding
    fn result(&self) -> Result<DataType, Error>;
}

/// Trait for encoders
pub trait Encode {
    fn max_len(&mut self) -> Option<usize>;

    fn from_liana_descriptor(descriptor: &LianaDescriptor) -> Result<Box<Self>, Error>;

    /// encode data from string (encoder)
    fn load_string(&mut self, data: &str) -> Result<Box<Self>, Error>;

    fn set_output_type(
        &mut self,
        data_type: DataType,
        encoding: Encoding,
        max_len: Option<usize>,
    ) -> &mut Self;

    fn from_psbt(psbt: &Psbt) -> Result<Box<Self>, Error>;

    fn from_xpub(xpub: &XPub) -> Result<Box<Self>, Error>;

    fn from_xpriv(xpriv: &XPriv) -> Result<Box<Self>, Error>;

    fn next(&mut self) -> Option<String>;

    // TODO: add iterator?
    // TODO: add looping iterator?
}

/// A generic QRCode Encoder
#[derive(Debug, Clone)]
pub struct QREncoder {
    encoder: OutputType,
}

impl QREncoder {
    pub fn new() -> QREncoder {
        let encoder = OutputType::NoType;
        QREncoder { encoder }
    }
}

impl Encode for QREncoder {
    fn max_len(&mut self) -> Option<usize> {
        todo!()
    }

    fn from_liana_descriptor(descriptor: &LianaDescriptor) -> Result<Box<Self>, Error> {
        todo!()
    }

    fn load_string(&mut self, data: &str) -> Result<Box<Self>, Error> {
        match self.encoder {
            OutputType::NoType => Err(Error::EncodingError(
                "An output type should be define prior to load!".to_string(),
            )),
            OutputType::LianaDescriptor => {
                // Import descriptor
                let imported_descriptor = LianaDescriptor::from_str(data).map_err(|e| {
                    Error::ParsingError("Cannot load this string into Liana descriptor".to_string())
                })?;
                let out = QREncoder::from_liana_descriptor(&imported_descriptor)?;
                Ok(out)
            }
            OutputType::Specter(_) => Err(Error::NotImplementedError(
                "type not yet implemented!".to_string(),
            )),
            OutputType::SpecterDescriptor => Err(Error::NotImplementedError(
                "type not yet implemented!".to_string(),
            )),
            OutputType::SpecterPsbt => Err(Error::NotImplementedError(
                "type not yet implemented!".to_string(),
            )),

            OutputType::SimpleQR(_) => Err(Error::NotImplementedError(
                "type not yet implemented!".to_string(),
            )),

            OutputType::Ur => Err(Error::NotImplementedError(
                "type not yet implemented!".to_string(),
            )),
            OutputType::UrBytes => Err(Error::NotImplementedError(
                "type not yet implemented!".to_string(),
            )),
            OutputType::UrDescriptor => Err(Error::NotImplementedError(
                "type not yet implemented!".to_string(),
            )),
            OutputType::UrPsbt => Err(Error::NotImplementedError(
                "type not yet implemented!".to_string(),
            )),
            OutputType::UrXpriv => Err(Error::NotImplementedError(
                "type not yet implemented!".to_string(),
            )),
            OutputType::UrXpub => Err(Error::NotImplementedError(
                "type not yet implemented!".to_string(),
            )),

            OutputType::Psbt => Err(Error::NotImplementedError(
                "type not yet implemented!".to_string(),
            )),
            OutputType::Xpriv => Err(Error::NotImplementedError(
                "type not yet implemented!".to_string(),
            )),
            OutputType::Xpub => Err(Error::NotImplementedError(
                "type not yet implemented!".to_string(),
            )),
            OutputType::Descriptor => Err(Error::NotImplementedError(
                "type not yet implemented!".to_string(),
            )),
            OutputType::LianaDescriptor => Err(Error::NotImplementedError(
                "type not yet implemented!".to_string(),
            )),

            _ => Err(Error::NotImplementedError(
                "type not yet implemented!".to_string(),
            )),
        }
    }

    fn set_output_type(
        &mut self,
        data_type: DataType,
        encoding: Encoding,
        max_len: Option<usize>,
    ) -> &mut Self {
        todo!()
    }

    fn from_psbt(psbt: &Psbt) -> Result<Box<Self>, Error> {
        todo!()
    }

    fn from_xpub(xpub: &XPub) -> Result<Box<Self>, Error> {
        todo!()
    }

    fn from_xpriv(xpriv: &XPriv) -> Result<Box<Self>, Error> {
        todo!()
    }

    fn next(&mut self) -> Option<String> {
        todo!()
    }
}

impl Decode for QREncoder {
    fn data_init(&mut self, sequences: usize) {
        todo!()
    }

    fn pattern() -> &'static str {
        todo!()
    }

    fn is_complete(&self) -> bool {
        todo!()
    }

    fn receive(&mut self, data: &str) -> Result<bool, String> {
        todo!()
    }

    fn process(&mut self) {
        todo!()
    }

    fn check_complete(&mut self) {
        todo!()
    }

    fn result() -> Result<DataType, Error> {
        todo!()
    }
}

// #[cfg(test)]
//
// #[test]
// fn test_specter_iterate_qr() {
//     let mut qr = SpecterQR::new();
//     let mut data = qr.load_string("012345678901234567890123456789012345678", 13).unwrap();
//
//     let _s = data.data_stack.len().to_string();
//     for i in data.data_stack {
//         match i {
//             Some(result) => {
//                 println!("{result}")
//             }
//             None => {
//                 println!("None")
//             }
//         }
//     }
// }
//
// #[test]
// fn test_specter_datastack_qr() {
//     let mut qr = SpecterQR::new();
//     let mut data = SpecterQR::load_string(&mut qr,"012345678901234567890123456789012345678", 13).unwrap();
//
//     let l = data.data_stack.len();
//     assert_eq!(l, 3);
//
//     let data = data.data_stack;
//
//     assert_eq!(data[0], Some("0123456789012".to_string()));
//     assert_eq!(data[1], Some("3456789012345".to_string()));
//     assert_eq!(data[2], Some("6789012345678".to_string()));
// }
//
// #[test]
// fn test_specter_next_qr() {
//     let mut qr = SpecterQR::new();
//     let mut data = SpecterQR::load_string(&mut qr,"012345678901234567890123456789012345678", 13).unwrap();
//     let a = data.next()
//         .unwrap();
//     let b = data.next().unwrap();
//     let c = data.next().unwrap();
//
//     assert_eq!(a, "p1of3 0123456789012".to_string());
//     assert_eq!(b, "p2of3 3456789012345".to_string());
//     assert_eq!(c, "p3of3 6789012345678".to_string());
// }
//
// #[test]
// fn test_specter_is_multi() {
//     let data = "p1of3 jsdgkjdrghlkjkmj".to_string();
//     let out = SpecterQR::is_multi(&data);
//     assert!(out)
// }
//
// #[test]
// fn test_specter_is_multi_false() {
//     let data = "jsdgkjdrghlkjkmj".to_string();
//     let out = SpecterQR::is_multi(&data);
//     assert!(!out)
// }
//
// #[test]
// fn test_append_specter_multi() {
//     let mut multi = SpecterQR::new();
//
//     Decode::receive(&mut multi, "p0of3 a");
//     Decode::receive(&mut multi, "p1of0 a");
//     Decode::receive(&mut multi, "p0of0 a");
//     Decode::receive(&mut multi, "p1of3 a").unwrap();
//     Decode::receive(&mut multi, "p2of3 b").unwrap();
//     Decode::receive(&mut multi, "p3of3 c").unwrap();
//
//     assert!(multi.is_complete());
//     assert_eq!(multi.data.data, "abc".to_string());
// }
//
// #[test]
// fn qr_encoder_load() {
//     let mut qr_encoder = QREncoder::new();
//     qr_encoder.load_str("213216546842lkljbjkhbvhgv5654", Specter(SpecterQR::new()), 13).unwrap();
// }
