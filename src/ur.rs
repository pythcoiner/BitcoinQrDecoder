extern crate ur;

use bitcoin::psbt::PartiallySignedTransaction as Psbt;
use liana::descriptors::LianaDescriptor as Descriptor ;
use bitcoin::bip32::{ExtendedPubKey as XPub, ExtendedPrivKey as XPriv};
use crate::{QRType, DataType, QR, MultiQR, Decode, Encode, Error, qr};
use crate::qr::QRData;

pub struct UrData {
    decoder: Option<ur::Decoder>,
    encoder: Option<ur::Encoder>,
    qr_type: QRType,
    data_type: DataType,
    max_len: Option<usize>,

}

impl QR for UrData {
    fn data_init(&mut self, sequences: usize) {
        todo!()
    }

    fn receive(&mut self, data: &String) -> bool {
        todo!()
    }
}

impl MultiQR for UrData {
    fn check_complete(&mut self) {
        todo!()
    }

    fn next(&mut self) -> Result<String, String> {
        todo!()
    }
}

impl Decode for UrData {
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

    fn result() -> Result<DataType, Error> {
        todo!()
    }
}

impl Encode for UrData {
    fn is_multi(data: &str) -> bool {
        todo!()
    }

    fn load_string(&mut self, data: &str, max_len: usize) -> Result<qr::QRData, Error> {
        todo!()
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