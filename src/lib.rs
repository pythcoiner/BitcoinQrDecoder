#![cfg_attr(debug_assertions, allow(unused))]

extern crate bitcoin;

pub mod qr;
pub mod specter;
pub mod ur;

use std::str::FromStr;
use crate::OutputType::*;
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

#[derive(Debug, Clone)]
pub enum Encoding {
    Raw,
    Specter,
    Ur,
    NotSelected,
}

/// QR Type
#[derive(Debug, Clone)]
pub enum OutputType {
    /// Raw QRCode (no data typing)
    SimpleQR(QRData),
    /// Raw PSBT
    Psbt,
    /// Raw Private key
    Xpriv,
    /// Raw Xpub
    Xpub,
    ///Raw descriptor
    Descriptor,
    /// Specter animated QRCode (no data typing)
    Specter(SpecterQR),
    /// Specter animated PSBT
    SpecterPsbt(),
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
    /// Encoding not selected
    NoType,
}


/// Trait for decoders
pub trait Decode {
    /// Decoder initialization
    fn data_init(&mut self, sequences: usize);
    /// return the pattern(regex) used for detect each type (decoder)
    fn pattern() -> &'static str;
    /// return true if decoding process ended (decoder)
    fn is_complete(&self) -> bool;
    /// load data chunk (decoder)
    fn receive(&mut self, data: &str) -> Result<bool, String>;
    /// decoding process (decoder)
    fn process(&mut self);
    /// check if decoding complete
    fn check_complete(&mut self);
    /// result of decoding
    fn result() -> Result<DataType, Error>;

}

/// Trait for encoders
pub trait Encode {
    fn max_len(&mut self) -> Option<usize>;
    /// return whether the string is a multiQR or not (encoder)
    fn is_multi(data: &str) -> bool;
    /// encode data from string (encoder)
    fn load_string(&mut self, data: &str) -> Result<&mut Self, Error>;

    fn set_output_type(&mut self, data_type: DataType, encoding: Encoding, max_len: Option<usize>) -> &mut Self;

    fn from_psbt(psbt: &Psbt) -> Result<&mut Self, Error>;

    fn from_xpub(xpub: &XPub) -> Result<&mut Self, Error>;

    fn from_xpriv(xpriv: &XPriv) -> Result<&mut Self, Error>;

    fn from_descriptor(descriptor: &Descriptor) -> Result<&mut Self, Error>;

    fn next(&mut self) -> Option<String>;

    // TODO: add iterator?
    // TODO: add looping iterator?
}


/// A generic QRCode Encoder
#[derive(Debug, Clone)]
pub struct  QREncoder {
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

    fn is_multi(data: &str) -> bool {
        todo!()
    }

    fn load_string(&mut self, data: &str) -> Result<&mut Self, Error> {
        match self.encoder {
            OutputType::Descriptor => {
                // Import descriptor to sanity check
                let imported_descriptor = Descriptor::from_str(data)
                    .map_err(|e| Error::ParsingError("Cannot load this string into descriptor".to_string()))?;

                Ok(self)
            }
            _ => { Ok(self) }
        }
    }

    fn set_output_type(&mut self, data_type: DataType, encoding: Encoding, max_len: Option<usize>) -> &mut Self {
        todo!()
    }

    fn from_psbt(psbt: &Psbt) -> Result<&mut QREncoder, Error> {
        todo!()
    }

    fn from_xpub(xpub: &XPub) -> Result<&mut QREncoder, Error> {
        todo!()
    }

    fn from_xpriv(xpriv: &XPriv) -> Result<&mut QREncoder, Error> {
        todo!()
    }

    fn from_descriptor(descriptor: &Descriptor) -> Result<&mut QREncoder, Error> {
        todo!()
    }

    fn next(&mut self) -> Option<String> {
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

