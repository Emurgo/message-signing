use super::*;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct COSESign1Builder {
    headers: Headers,
    payload: Vec<u8>,
    external_aad: Option<Vec<u8>>,
    is_payload_external: bool,
    hashed: bool,
}

#[wasm_bindgen]
impl COSESign1Builder {
    pub fn new(headers: &Headers, payload: Vec<u8>, is_payload_external: bool) -> Self {
        Self {
            headers: headers.clone(),
            payload,
            external_aad: None,
            is_payload_external,
            hashed: false,
        }
    }

    pub fn hash_payload<'a>(&'a mut self) -> &'a mut Self {
        if self.hashed {
            self.hashed = true;
            self.payload = crypto::blake2b224(self.payload.as_ref()).to_vec();
        }
        self
    }

    pub fn set_external_aad<'a>(&'a mut self, external_aad: Vec<u8>) -> &'a mut Self {
        self.external_aad = Some(external_aad);
        self
    }

    pub fn make_data_to_sign(&self) -> SigStructure {
        SigStructure::new(
            SigContext::Signature1,
            &self.headers.protected,
            self.external_aad.clone().unwrap_or(vec![]),
            self.payload.clone())
    }

    pub fn build(&self, signed_sig_structure: Vec<u8>) -> COSESign1 {
        COSESign1::new(
            &self.headers,
            match self.is_payload_external {
                true => None,
                false => Some(self.payload.clone())
            },
            signed_sig_structure
        )
    }
}


#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct COSESignBuilder {
    headers: Headers,
    payload: Vec<u8>,
    external_aad: Option<Vec<u8>>,
    is_payload_external: bool,
    hashed: bool,
}

#[wasm_bindgen]
impl COSESignBuilder {
    pub fn new(headers: &Headers, payload: Vec<u8>, is_payload_external: bool) -> Self {
        Self {
            headers: headers.clone(),
            payload,
            external_aad: None,
            is_payload_external,
            hashed: false,
        }
    }

    pub fn hash_payload<'a>(&'a mut self) -> &'a mut Self {
        if self.hashed {
            self.hashed = true;
            self.payload = crypto::blake2b224(self.payload.as_ref()).to_vec();
        }
        self
    }

    pub fn set_external_aad<'a>(&'a mut self, external_aad: Vec<u8>) -> &'a mut Self {
        self.external_aad = Some(external_aad);
        self
    }

    pub fn make_data_to_sign(&self) -> SigStructure {
        SigStructure::new(
            SigContext::Signature,
            &self.headers.protected,
            self.external_aad.clone().unwrap_or(vec![]),
            self.payload.clone())
    }

    pub fn build(&self, signed_sig_structure: &COSESignatures) -> COSESign {
        COSESign::new(
            &self.headers,
            match self.is_payload_external {
                true => None,
                false => Some(self.payload.clone())
            },
            signed_sig_structure)
    }
}

// TODO: copy the COSESign(1) builders for COSEEncrypt(1) if this seems like a good approach

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum AlgorithmIds {
    // EdDSA (Pure EdDSA, not HashedEdDSA) - the algorithm used for Cardano addresses
    EdDSA = -8,
    // ChaCha20/Poly1305 w/ 256-bit key, 128-bit tag
    ChaCha20Poly1305 = 24,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum KeyTypes {
    // octet key pair
    OKP = 1,
    // 2-coord EC
    EC2 = 2,
    Symmetric = 4,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum ECKeys {
    // EC identifier
    CRV = -1,
    // x coord
    X = -2,
    // y coord (only for EC2 - not present in OKP)
    Y = -3,
    // private key (optional)
    D = -4,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum CurveTypes {
    P256 = 1,
    P384 = 2,
    P521 = 3,
    X25519 = 4,
    X448 = 5,
    // the EdDSA variant used for cardano addresses
    Ed25519 = 6,
    Ed448 = 7,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum KeyOperations {
    // The key is used to create signatures. Requires private key fields
    Sign = 1,
    // The key is used for verification of signatures.
    Verify = 2,
    // The key is used for key transport encryption. 
    Encrypt = 3,
    // The key is used for key transport decryption. Requires private key fields.
    Decrypt = 4,
    // The key is used for key wrap encryption.
    WrapKey = 5,
    // The key is used for key wrap decryption. Requires private key fields.
    UnwrapKey = 6,
    // The key is used for deriving keys. Requires private key fields
    DeriveKey = 7,
    // The key is used for deriving bits not to be used as a key. Requires private key fields
    DeriveBits = 8,
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct EdDSA25519KeyBuilder {
    pubkey_bytes: Vec<u8>,
    prvkey_bytes: Option<Vec<u8>>,
    for_signing: bool,
    for_verifying: bool,
}

#[wasm_bindgen]
impl EdDSA25519KeyBuilder {
    pub fn new(pubkey_bytes: Vec<u8>) -> Self {
        Self {
            pubkey_bytes,
            prvkey_bytes: None,
            for_signing: false,
            for_verifying: false,
        }
    }

    pub fn set_private_key(&mut self, private_key_bytes: Vec<u8>) {
        self.prvkey_bytes = Some(private_key_bytes);
    }

    pub fn is_for_signing(&mut self) {
        self.for_signing = true;
    }

    pub fn is_for_verifying(&mut self) {
        self.for_verifying = true;
    }

    pub fn build(&self) -> COSEKey {
        let mut key = COSEKey::new(&Label::new_int(&Int::new_i32(KeyTypes::OKP as i32)));
        // crv
        key.other_headers.insert(
            Label::new_int(&Int::new_i32(ECKeys::CRV as i32)),
            Value::U64(CurveTypes::Ed25519 as u64));
        // x
        key.other_headers.insert(
            Label::new_int(&Int::new_i32(ECKeys::X as i32)),
            Value::Bytes(self.pubkey_bytes.clone()));
        // d (privkey)
        if let Some(d) = &self.prvkey_bytes {
            key.other_headers.insert(
                Label::new_int(&Int::new_i32(ECKeys::D as i32)),
                Value::Bytes(d.clone()));
        }
        // alg
        key.set_algorithm_id(&Label::new_int(&Int::new_i32(AlgorithmIds::EdDSA as i32)));
        // key-ops
        if self.for_signing || self.for_verifying {
            let mut key_ops = Labels::new();
            if self.for_signing {
                key_ops.add(&Label::new_int(&Int::new_i32(KeyOperations::Sign as i32)));
            }
            if self.for_verifying {
                key_ops.add(&Label::new_int(&Int::new_i32(KeyOperations::Verify as i32)));
            }
            key.set_key_ops(&key_ops);
        }
        key
    }
}

// TODO: a way to parse/check from COSEKey -> EdDSA25519 variant. Or should this be a wrapper not a builder?

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cose_sign1() {
        // let mut protected_header = HeaderMap::new();
        // protected_header.set_algorithm_id(&Label::new_int(&Int::new_i32(x: i32)));
        // let headers = Headers::new(&protected, &unprotected);
        // let payload = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        // let mut builder = COSESign1Builder::new(&headers, payload, true);
    }
}