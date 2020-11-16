// This library was partially code-generated using an experimental CDDL to rust tool:
// https://github.com/Emurgo/cddl-codegen

use std::io::{BufRead, Write};
use linked_hash_map::LinkedHashMap;

#[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
use noop_proc_macro::wasm_bindgen;

#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
use wasm_bindgen::prelude::*;

use cbor_event::{self, de::Deserializer, se::{Serialize, Serializer}, Value};
use cbor_event::Type as CBORType;
use cbor_event::Special as CBORSpecial;

mod error;
mod serialization;
#[macro_use]
pub mod utils;

use error::*;
use utils::*;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct EmptyOrSerializedMap(Vec<u8>);

to_from_bytes!(EmptyOrSerializedMap);

#[wasm_bindgen]
impl EmptyOrSerializedMap {
    pub fn new_empty() -> Self {
        Self(Vec::new())
    }

    pub fn new(header_map: &HeaderMap) -> Self {
        Self(header_map.to_bytes())
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum LabelKind {
    Int,
    Text,
}

#[derive(Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
enum LabelEnum {
    Int(Int),
    Text(String),
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Label(LabelEnum);

to_from_bytes!(Label);

#[wasm_bindgen]
impl Label {
    pub fn new_int(int: &Int) -> Self {
        Self(LabelEnum::Int(int.clone()))
    }

    pub fn new_text(text: String) -> Self {
        Self(LabelEnum::Text(text))
    }

    pub fn kind(&self) -> LabelKind {
        match &self.0 {
            LabelEnum::Int(_) => LabelKind::Int,
            LabelEnum::Text(_) => LabelKind::Text,
        }
    }

    pub fn as_int(&self) -> Option<Int> {
        match &self.0 {
            LabelEnum::Int(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_text(&self) -> Option<String> {
        match &self.0 {
            LabelEnum::Text(x) => Some(x.clone()),
            _ => None,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Labels(Vec<Label>);

to_from_bytes!(Labels);

#[wasm_bindgen]
impl Labels {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Label {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: &Label) {
        self.0.push(elem.clone());
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct COSESignatures(Vec<COSESignature>);

to_from_bytes!(COSESignatures);

#[wasm_bindgen]
impl COSESignatures {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> COSESignature {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: &COSESignature) {
        self.0.push(elem.clone());
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub enum COSESignatureOrArrCOSESignatureKind {
    COSESignature,
    ArrCOSESignature,
}

#[derive(Clone, Debug)]
enum COSESignatureOrArrCOSESignatureEnum {
    COSESignature(COSESignature),
    ArrCOSESignature(COSESignatures),
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct COSESignatureOrArrCOSESignature(COSESignatureOrArrCOSESignatureEnum);

to_from_bytes!(COSESignatureOrArrCOSESignature);

#[wasm_bindgen]
impl COSESignatureOrArrCOSESignature {
    pub fn new_cose_signature(cose_signature: &COSESignature) -> Self {
        Self(COSESignatureOrArrCOSESignatureEnum::COSESignature(cose_signature.clone()))
    }

    pub fn new_cose_signatures(cose_signatures: &COSESignatures) -> Self {
        Self(COSESignatureOrArrCOSESignatureEnum::ArrCOSESignature(cose_signatures.clone()))
    }

    pub fn kind(&self) -> COSESignatureOrArrCOSESignatureKind {
        match &self.0 {
            COSESignatureOrArrCOSESignatureEnum::COSESignature(_) => COSESignatureOrArrCOSESignatureKind::COSESignature,
            COSESignatureOrArrCOSESignatureEnum::ArrCOSESignature(_) => COSESignatureOrArrCOSESignatureKind::ArrCOSESignature,
        }
    }

    pub fn as_cose_signature(&self) -> Option<COSESignature> {
        match &self.0 {
            COSESignatureOrArrCOSESignatureEnum::COSESignature(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_cose_signatures(&self) -> Option<COSESignatures> {
        match &self.0 {
            COSESignatureOrArrCOSESignatureEnum::ArrCOSESignature(x) => Some(x.clone()),
            _ => None,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct HeaderMap {
    algorithm_id: Option<Label>,
    criticality: Option<Labels>,
    content_type: Option<Label>,
    key_id: Option<Vec<u8>>,
    init_vector: Option<Vec<u8>>,
    partial_init_vector: Option<Vec<u8>>,
    counter_signature: Option<Box<COSESignatureOrArrCOSESignature>>,
    pub other_headers: LinkedHashMap<Label, Value>,
}

to_from_bytes!(HeaderMap);

#[wasm_bindgen]
impl HeaderMap {
    pub fn set_algorithm_id(&mut self, algorithm_id: &Label) {
        self.algorithm_id = Some(algorithm_id.clone())
    }

    pub fn algorithm_id(&self) -> Option<Label> {
        self.algorithm_id.clone()
    }

    pub fn set_criticality(&mut self, criticality: &Labels) {
        self.criticality = Some(criticality.clone())
    }

    pub fn criticality(&self) -> Option<Labels> {
        self.criticality.clone()
    }

    pub fn set_content_type(&mut self, content_type: &Label) {
        self.content_type = Some(content_type.clone())
    }

    pub fn content_type(&self) -> Option<Label> {
        self.content_type.clone()
    }

    pub fn set_key_id(&mut self, key_id: Vec<u8>) {
        self.key_id = Some(key_id)
    }

    pub fn key_id(&self) -> Option<Vec<u8>> {
        self.key_id.clone()
    }

    pub fn set_init_vector(&mut self, init_vector: Vec<u8>) {
        self.init_vector = Some(init_vector)
    }

    pub fn init_vector(&self) -> Option<Vec<u8>> {
        self.init_vector.clone()
    }

    pub fn set_partial_init_vector(&mut self, partial_init_vector: Vec<u8>) {
        self.partial_init_vector = Some(partial_init_vector)
    }

    pub fn partial_init_vector(&self) -> Option<Vec<u8>> {
        self.partial_init_vector.clone()
    }

    pub fn set_counter_signature(&mut self, counter_signature: &COSESignatureOrArrCOSESignature) {
        self.counter_signature = Some(Box::new(counter_signature.clone()))
    }

    pub fn counter_signature(&self) -> Option<COSESignatureOrArrCOSESignature> {
        use std::ops::Deref;
        self.counter_signature.as_ref().map(|sig| sig.deref().clone())
    }

    pub fn new() -> Self {
        Self {
            algorithm_id: None,
            criticality: None,
            content_type: None,
            key_id: None,
            init_vector: None,
            partial_init_vector: None,
            counter_signature: None,
            other_headers: LinkedHashMap::new(),
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Headers {
    protected: EmptyOrSerializedMap,
    unprotected: HeaderMap,
}

to_from_bytes!(Headers);

#[wasm_bindgen]
impl Headers {
    pub fn protected(&self) -> EmptyOrSerializedMap {
        self.protected.clone()
    }

    pub fn unprotected(&self) -> HeaderMap {
        self.unprotected.clone()
    }

    pub fn new(protected: &EmptyOrSerializedMap, unprotected: &HeaderMap) -> Self {
        Self {
            protected: protected.clone(),
            unprotected: unprotected.clone(),
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct COSESignature {
    headers: Headers,
    signature: Vec<u8>,
}

to_from_bytes!(COSESignature);

#[wasm_bindgen]
impl COSESignature {
    pub fn headers(&self) -> Headers {
        self.headers.clone()
    }

    pub fn signature(&self) -> Vec<u8> {
        self.signature.clone()
    }

    pub fn new(headers: &Headers, signature: Vec<u8>) -> Self {
        Self {
            headers: headers.clone(),
            signature: signature,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct COSESign1 {
    headers: Headers,
    payload: Option<Vec<u8>>,
    signature: Vec<u8>,
}

to_from_bytes!(COSESign1);

#[wasm_bindgen]
impl COSESign1 {
    pub fn headers(&self) -> Headers {
        self.headers.clone()
    }

    pub fn payload(&self) -> Option<Vec<u8>> {
        self.payload.clone()
    }

    pub fn signature(&self) -> Vec<u8> {
        self.signature.clone()
    }

    pub fn new(headers: &Headers, payload: Option<Vec<u8>>, signature: Vec<u8>) -> Self {
        Self {
            headers: headers.clone(),
            payload: payload,
            signature: signature,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct COSESign {
    headers: Headers,
    payload: Option<Vec<u8>>,
    signatures: COSESignatures,
}

to_from_bytes!(COSESign);

#[wasm_bindgen]
impl COSESign {
    pub fn headers(&self) -> Headers {
        self.headers.clone()
    }

    pub fn payload(&self) -> Option<Vec<u8>> {
        self.payload.clone()
    }

    pub fn signatures(&self) -> COSESignatures {
        self.signatures.clone()
    }

    pub fn new(headers: &Headers, payload: Option<Vec<u8>>, signatures: &COSESignatures) -> Self {
        Self {
            headers: headers.clone(),
            payload: payload,
            signatures: signatures.clone(),
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum SignedMessageKind {
    COSESIGN,
    COSESIGN1,
}

#[derive(Clone, Debug)]
enum SignedMessageEnum {
    COSESIGN(COSESign),
    COSESIGN1(COSESign1),
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct SignedMessage(SignedMessageEnum);

to_from_bytes!(SignedMessage);

#[wasm_bindgen]
impl SignedMessage {
    pub fn new_cose_sign(cose_sign: &COSESign) -> Self {
        Self(SignedMessageEnum::COSESIGN(cose_sign.clone()))
    }

    pub fn new_cose_sign1(cose_sign1: &COSESign1) -> Self {
        Self(SignedMessageEnum::COSESIGN1(cose_sign1.clone()))
    }

    pub fn kind(&self) -> SignedMessageKind {
        match &self.0 {
            SignedMessageEnum::COSESIGN(_) => SignedMessageKind::COSESIGN,
            SignedMessageEnum::COSESIGN1(_) => SignedMessageKind::COSESIGN1,
        }
    }

    pub fn as_cose_sign(&self) -> Option<COSESign> {
        match &self.0 {
            SignedMessageEnum::COSESIGN(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_cose_sign1(&self) -> Option<COSESign1> {
        match &self.0 {
            SignedMessageEnum::COSESIGN1(x) => Some(x.clone()),
            _ => None,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum SigContext {
    Signature,
    Signature1,
    CounterSignature
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct SigStructure {
    context: SigContext,
    body_protected: EmptyOrSerializedMap,
    sign_protected: Option<EmptyOrSerializedMap>,
    external_aad: Vec<u8>,
    payload: Vec<u8>,
}

to_from_bytes!(SigStructure);

#[wasm_bindgen]
impl SigStructure {
    pub fn context(&self) -> SigContext {
        self.context.clone()
    }

    pub fn body_protected(&self) -> EmptyOrSerializedMap {
        self.body_protected.clone()
    }

    pub fn sign_protected(&self) -> Option<EmptyOrSerializedMap> {
        self.sign_protected.clone()
    }

    pub fn external_aad(&self) -> Vec<u8> {
        self.external_aad.clone()
    }

    pub fn payload(&self) -> Vec<u8> {
        self.payload.clone()
    }

    pub fn set_sign_protected(&mut self, sign_protected: &EmptyOrSerializedMap) {
        self.sign_protected = Some(sign_protected.clone());
    }

    pub fn new(context: SigContext, body_protected: &EmptyOrSerializedMap, external_aad: Vec<u8>, payload: Vec<u8>) -> Self {
        Self {
            context,
            body_protected: body_protected.clone(),
            sign_protected: None,
            external_aad,
            payload,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct COSEEncrypt0 {
    headers: Headers,
    ciphertext: Option<Vec<u8>>,
}

to_from_bytes!(COSEEncrypt0);

#[wasm_bindgen]
impl COSEEncrypt0 {
    pub fn headers(&self) -> Headers {
        self.headers.clone()
    }

    pub fn ciphertext(&self) -> Option<Vec<u8>> {
        self.ciphertext.clone()
    }

    pub fn new(headers: &Headers, ciphertext: Option<Vec<u8>>) -> Self {
        Self {
            headers: headers.clone(),
            ciphertext: ciphertext,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct PasswordEncryption(COSEEncrypt0);

to_from_bytes!(PasswordEncryption);

#[wasm_bindgen]
impl PasswordEncryption {
    pub fn new(data: &COSEEncrypt0) -> Self {
        Self(data.clone())
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct COSERecipients(Vec<COSERecipient>);

to_from_bytes!(COSERecipients);

#[wasm_bindgen]
impl COSERecipients {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> COSERecipient {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: &COSERecipient) {
        self.0.push(elem.clone());
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct COSEEncrypt {
    headers: Headers,
    ciphertext: Option<Vec<u8>>,
    recipients: COSERecipients,
}

to_from_bytes!(COSEEncrypt);

#[wasm_bindgen]
impl COSEEncrypt {
    pub fn headers(&self) -> Headers {
        self.headers.clone()
    }

    pub fn ciphertext(&self) -> Option<Vec<u8>> {
        self.ciphertext.clone()
    }

    pub fn recipients(&self) -> COSERecipients {
        self.recipients.clone()
    }

    pub fn new(headers: &Headers, ciphertext: Option<Vec<u8>>, recipients: &COSERecipients) -> Self {
        Self {
            headers: headers.clone(),
            ciphertext: ciphertext,
            recipients: recipients.clone(),
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct COSERecipient {
    headers: Headers,
    ciphertext: Option<Vec<u8>>,
}

to_from_bytes!(COSERecipient);

#[wasm_bindgen]
impl COSERecipient {
    pub fn headers(&self) -> Headers {
        self.headers.clone()
    }

    pub fn ciphertext(&self) -> Option<Vec<u8>> {
        self.ciphertext.clone()
    }

    pub fn new(headers: &Headers, ciphertext: Option<Vec<u8>>) -> Self {
        Self {
            headers: headers.clone(),
            ciphertext: ciphertext,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct PubKeyEncryption(COSEEncrypt);

to_from_bytes!(PubKeyEncryption);

#[wasm_bindgen]
impl PubKeyEncryption {
    pub fn new(data: &COSEEncrypt) -> Self {
        Self(data.clone())
    }
}