extern crate ur;

use bitcoin::psbt::PartiallySignedTransaction as Psbt;
use liana::descriptors::LianaDescriptor as Descriptor ;
use bitcoin::bip32::{ExtendedPubKey as XPub, ExtendedPrivKey as XPriv};
use crate::{OutputType, DataType, Decode, Encode, Error, qr, Encoding};
use crate::qr::QRData;

pub struct UrData {
    decoder: Option<ur::Decoder>,
    encoder: Option<ur::Encoder>,
    qr_type: OutputType,
    data_type: DataType,
    max_len: Option<usize>,

}

impl Decode for UrData {
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

impl Encode for UrData {
    fn max_len(&mut self) -> Option<usize> {
        todo!()
    }

    fn is_multi(data: &str) -> bool {
        todo!()
    }

    fn load_string(&mut self, data: &str) -> Result<&mut UrData, Error> {
        todo!()
    }

    fn set_output_type(&mut self, data_type: DataType, encoding: Encoding, max_len: Option<usize>) -> &mut Self {
        todo!()
    }

    fn from_psbt(psbt: &Psbt) -> Result<&mut UrData, Error> {
        todo!()
    }

    fn from_xpub(xpub: &XPub) -> Result<&mut UrData, Error> {
        todo!()
    }

    fn from_xpriv(xpriv: &XPriv) -> Result<&mut UrData, Error> {
        todo!()
    }

    fn from_descriptor(descriptor: &Descriptor) -> Result<&mut UrData, Error> {
        todo!()
    }

    fn next(&mut self) -> Option<String> {
        todo!()
    }
}