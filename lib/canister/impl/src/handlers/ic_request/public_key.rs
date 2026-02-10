use common_canister_types::{Asn1BlockPublicKey, UncompressedPublicKey};
use der::asn1::BitString;
use der::{
    asn1::ObjectIdentifier, DecodeValue, Encode, EncodeValue, Header, Length, Reader, Sequence,
    Writer,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MetaData {
    pub ec_public_key_id: ObjectIdentifier,
    pub secp256k1_id: ObjectIdentifier,
}

impl MetaData {
    fn new() -> Self {
        Self {
            ec_public_key_id: "1.2.840.10045.2.1".parse::<ObjectIdentifier>().unwrap(),
            secp256k1_id: "1.3.132.0.10".parse::<ObjectIdentifier>().unwrap(),
        }
    }
}

impl<'a> DecodeValue<'a> for MetaData {
    fn decode_value<R: Reader<'a>>(reader: &mut R, _header: Header) -> der::Result<Self> {
        let ec_public_key_id = reader.decode()?;
        let secp256k1_id = reader.decode()?;

        Ok(Self {
            ec_public_key_id,
            secp256k1_id,
        })
    }
}

impl EncodeValue for MetaData {
    fn value_len(&self) -> der::Result<Length> {
        self.ec_public_key_id.encoded_len()? + self.secp256k1_id.encoded_len()?
    }

    fn encode_value(&self, writer: &mut impl Writer) -> der::Result<()> {
        self.ec_public_key_id.encode(writer)?;
        self.secp256k1_id.encode(writer)?;
        Ok(())
    }
}

impl Sequence<'_> for MetaData {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Asn1PublicKey {
    pub meta_data: MetaData,
    pub data: BitString,
}

impl Asn1PublicKey {
    fn new(pk: &[u8]) -> Self {
        Self {
            meta_data: MetaData::new(),
            data: BitString::from_bytes(pk).unwrap(),
        }
    }
}

impl<'a> DecodeValue<'a> for Asn1PublicKey {
    fn decode_value<R: Reader<'a>>(reader: &mut R, _header: Header) -> der::Result<Self> {
        let meta_data = reader.decode()?;
        let data = reader.decode()?;

        Ok(Self { meta_data, data })
    }
}

impl Sequence<'_> for Asn1PublicKey {}

impl EncodeValue for Asn1PublicKey {
    fn value_len(&self) -> der::Result<Length> {
        self.meta_data.encoded_len()? + self.data.encoded_len()?
    }

    fn encode_value(&self, writer: &mut impl Writer) -> der::Result<()> {
        self.meta_data.encode(writer)?;
        self.data.encode(writer)?;
        Ok(())
    }
}

pub fn uncompressed_public_key_to_asn1_block(
    public_key: UncompressedPublicKey,
) -> Asn1BlockPublicKey {
    Asn1PublicKey::new(public_key.as_slice()).to_der().unwrap()
}
