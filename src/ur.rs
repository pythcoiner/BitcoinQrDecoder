extern crate regex;
extern crate ur;

use crate::qr::QRData;
use crate::Error::DecodingError;
use crate::{qr, DataType, Decode, Encode, Encoding, Error, OutputType};
use bitcoin::bip32::{ExtendedPrivKey as XPriv, ExtendedPubKey as XPub};
use bitcoin::psbt::PartiallySignedTransaction as Psbt;
use liana::descriptors::LianaDescriptor as Descriptor;
use regex::Regex;

pub struct UrData {
    decoder: Option<ur::Decoder>,
    encoder: Option<ur::Encoder>,
    output_type: OutputType,
    data_type: DataType,
    max_len: Option<usize>,
    multi: Option<bool>,
}

impl UrData {
    pub fn new() -> Self {
        UrData {
            decoder: None,
            encoder: None,
            output_type: OutputType::NoType,
            data_type: DataType::NoType,
            max_len: None,
            multi: None,
        }
    }

    pub fn is_ur(&data: &String) -> bool {
        return if data[..2].to_lowercase() == "ur:" {
            true
        } else {
            false
        };
    }

    pub fn get_type(&data: &String) -> Result<OutputType, Error> {
        let t: Vec<&str> = data.clone().split('/').collect();
        let _type: &str = t[0];
        return match _type.to_lowercase().as_str() {
            "ur:bytes" => Ok(OutputType::NoType),
            "ur:crypto-psbt" => Ok(OutputType::UrPsbt),
            "ur:crypto-output" => Ok(OutputType::UrDescriptor),
            "ur:crypto-account" => Ok(OutputType::UrXpub),
            "ur:crypto-address" => Ok(OutputType::UrAddress),
            "ur:crypto-hdkey" => Ok(OutputType::UrXpriv),
            _ => Err(Error::DecodingError("Unknown UR type!".to_string())),
        };
    }

    pub fn is_multi(&data: &String) -> bool {
        let t: Vec<&str> = data.clone().split('/').collect();
        let multi: &str = t[1];
        let re = Regex::new(r"^\d+-\d+$").unwrap();
        return if re.is_match(multi) { true } else { false };
    }

    pub fn type_check(&self, &data: &String) -> bool {
        let data_type = UrData::get_type(&data);
        // if not UR type
        return if !UrData::is_ur(&data) {
            false
        }
        // if data type unknown
        else if data_type.is_err() {
            false
        }
        // decoder not init
        else if self.multi.is_none() & (self.output_type == OutputType::NoType) {
            true
        }
        // multi not match
        else if self.multi != Some(UrData::is_multi(&data)) {
            false
        }
        // output not match
        else if self.output_type != data_type.unwrap() {
            false
        }
        // any other case
        else {
            true
        };
    }

    pub fn is_encoder(&self) -> bool {
        self.encoder.is_some() & self.decoder.is_none()
    }

    pub fn is_decoder(&self) -> bool {
        self.encoder.is_none() & self.encoder.is_some()
    }
}

impl Decode for UrData {
    fn pattern() -> &'static str {
        todo!()
    }

    fn is_complete(&self) -> bool {
        if !self.is_decoder() {
            false
        } else {
            self.decoder.unwrap().complete()
        }
    }

    fn receive(&mut self, data: &str) -> Result<bool, Error> {
        // encoder/decoder mismatch
        if self.is_decoder() {
            return Err(Error::DecodingError(
                "UrData cannot have encode and decode feature at same time!".to_string(),
            ));
        };
        if self.type_check(&data.to_string()) {
            let _type = UrData::get_type(&data.to_string())?;
            // if no data_type define, init the decoder
            return if (self.data_type == DataType::NoType) & (_type != DataType::NoType) {
                self.data_type = _type;
                self.decoder = Some(ur::Decoder::default());
                self.decoder.as_mut().map(|decoder| decoder.receive(data));
                Ok(true)
            // if decoder already init
            } else if self.data_type == _type {
                self.decoder.as_mut().map(|decoder| decoder.receive(data));
                Ok(true)
            // types are different
            } else {
                Err(Error::DecodingError(" Mismatching types!".to_string()))
            };
        } else {
            return Err(Error::DecodingError(
                "data don't pass type check!".to_string(),
            ));
        }
    }

    fn result(&self) -> Result<DataType, Error> {
        if !self.is_decoder() {
            Err(Error::DecodingError(
                "UrData is not of decoder type!".to_string(),
            ))
        } else if self.is_complete() {
            match self.output_type {
                // UR BYTES
                OutputType::NoType => {
                    let m = self.decoder.unwrap().message();
                    if m.is_err() {
                        Err(Error::DecodingError("Cannot decode message !".to_string()))
                    } else {
                        if m.unwrap().is_none() {
                            Err(Error::DecodingError("Message is None".to_string()))
                        } else {
                            let message = String::from_utf8(m.unwrap().unwrap());
                            if message.is_err() {
                                Err(Error::DecodingError("FromUtf8Error".to_string()))
                            } else {
                                Ok(DataType::NoType(Some(message.unwrap())))
                            }
                        }
                    }
                }
                // TODO: implememt other cases
                _ => Err(Error::NotImplementedError("".to_string())),
            }
        } else {
            Err(Error::NotImplementedError("".to_string()))
        }
    }
}

impl Encode for UrData {
    fn max_len(&mut self) -> Option<usize> {
        todo!()
    }

    fn from_liana_descriptor(descriptor: &Descriptor) -> Result<Box<Self>, Error> {
        todo!()
    }

    fn load_string(&mut self, data: &str) -> Result<Box<Self>, Error> {
        todo!()
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
