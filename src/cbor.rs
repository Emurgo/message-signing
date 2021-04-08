use super::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TaggedCBOR {
    tag: BigNum,
    value: CBORValue,
}

#[wasm_bindgen]
impl TaggedCBOR {
    pub fn tag(&self) -> BigNum {
        self.tag
    }

    pub fn value(&self) -> CBORValue {
        self.value.clone()
    }

    pub fn new(tag: BigNum, value: &CBORValue) -> Self {
        Self {
            tag,
            value: value.clone(),
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CBORArray {
    definite: bool,
    pub (crate) values: Vec<CBORValue>,
}

#[wasm_bindgen]
impl CBORArray {
    pub fn new() -> Self {
        Self {
            definite: true,
            values: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn get(&self, index: usize) -> CBORValue {
        self.values[index].clone()
    }

    pub fn add(&mut self, elem: &CBORValue) {
        self.values.push(elem.clone());
    }

    // CBOR allows either definite or indefinite encoding - specify which to use here.
    // Default is definite.
    pub fn set_definite_encoding(&mut self, use_definite: bool) {
        self.definite = use_definite
    }

    // True -> Definite CBOR encoding is used (length explicitly written)
    // False -> Indefinite CBOR encoding used (length implicit - read/write until CBOR Break found)
    pub fn is_definite(&self) -> bool {
        self.definite
    }
}

impl From<Vec<CBORValue>> for CBORArray {
    fn from(vec: Vec<CBORValue>) -> Self {
        Self {
            definite: true,
            values: vec,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CBORObject {
    definite: bool,
    values: LinkedHashMap<CBORValue, CBORValue>,
}

#[wasm_bindgen]
impl CBORObject {
    pub fn new() -> Self {
        Self {
            definite: true,
            values: LinkedHashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn insert(&mut self, key: &CBORValue, value: &CBORValue) -> Option<CBORValue> {
        self.values.insert(key.clone(), value.clone())
    }

    pub fn get(&self, key: &CBORValue) -> Option<CBORValue> {
        self.values.get(key).map(|v| v.clone())
    }

    pub fn keys(&self) -> CBORArray {
        self.values.iter().map(|(k, _v)| k.clone()).collect::<Vec<CBORValue>>().into()
    }

    // CBOR allows either definite or indefinite encoding - specify which to use here.
    // Default is definite.
    pub fn set_definite_encoding(&mut self, use_definite: bool) {
        self.definite = use_definite
    }

    // True -> Definite CBOR encoding is used (length explicitly written)
    // False -> Indefinite CBOR encoding used (length implicit - read/write until CBOR Break found)
    pub fn is_definite(&self) -> bool {
        self.definite
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CBORSpecialType {
    Bool,
    Float,
    Unassigned,
    Break,
    Undefined,
    Null,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
enum CBORSpecialEnum {
    Bool(bool),
    Float(f64),
    Unassigned(u8),
    Break,
    Undefined,
    Null,
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct CBORSpecial(CBORSpecialEnum);

#[wasm_bindgen]
impl CBORSpecial {
    pub fn new_bool(b: bool) -> Self {
        Self(CBORSpecialEnum::Bool(b))
    }

    pub fn new_float(f: f64) -> Self {
        Self(CBORSpecialEnum::Float(f))
    }

    pub fn new_unassigned(u: u8) -> Self {
        Self(CBORSpecialEnum::Unassigned(u))
    }

    pub fn new_break() -> Self {
        Self(CBORSpecialEnum::Break)
    }

    pub fn new_null() -> Self {
        Self(CBORSpecialEnum::Null)
    }

    pub fn new_undefined() -> Self {
        Self(CBORSpecialEnum::Undefined)
    }

    pub fn kind(&self) -> CBORSpecialType {
        match &self.0 {
            CBORSpecialEnum::Bool(_) => CBORSpecialType::Bool,
            CBORSpecialEnum::Float(_) => CBORSpecialType::Float,
            CBORSpecialEnum::Unassigned(_) => CBORSpecialType::Unassigned,
            CBORSpecialEnum::Break => CBORSpecialType::Break,
            CBORSpecialEnum::Undefined => CBORSpecialType::Undefined,
            CBORSpecialEnum::Null => CBORSpecialType::Null,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match &self.0 {
            CBORSpecialEnum::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match &self.0 {
            CBORSpecialEnum::Float(f) => Some(*f),
            _ => None,
        }
    }

    pub fn as_unassigned(&self) -> Option<u8> {
        match &self.0 {
            CBORSpecialEnum::Unassigned(u) => Some(*u),
            _ => None,
        }
    }
}

// Rust does not provide Ord, Hash or Eq implementations for floats due to issues with
// NaN, etc. We provide them here and compare floats byte-wise just so we can use the
// CBORSpecial enum containing them (and others transitively) as keys to follow the CBOR spec.
fn f64_to_bytes(f: f64) -> [u8; std::mem::size_of::<f64>()] {
    use byteorder::{BigEndian, WriteBytesExt};

    let mut bytes = [0u8; std::mem::size_of::<f64>()];
    bytes.as_mut().write_f64::<BigEndian>(f).unwrap();
    bytes
}

impl PartialEq for CBORSpecial {
    fn eq(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (CBORSpecialEnum::Bool(b1), CBORSpecialEnum::Bool(b2)) => *b1 == *b2,
            (CBORSpecialEnum::Float(f1), CBORSpecialEnum::Float(f2)) => f64_to_bytes(*f1) == f64_to_bytes(*f2),
            (CBORSpecialEnum::Unassigned(u1), CBORSpecialEnum::Unassigned(u2)) => *u1 == *u2,
            (CBORSpecialEnum::Break, CBORSpecialEnum::Break) |
            (CBORSpecialEnum::Undefined, CBORSpecialEnum::Undefined) |
            (CBORSpecialEnum::Null, CBORSpecialEnum::Null) => true,
            _mixed_types => false,
        }
    }
}

impl Eq for CBORSpecial {
}

impl Ord for CBORSpecial {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (&self.0, &other.0) {
            (CBORSpecialEnum::Bool(b1), CBORSpecialEnum::Bool(b2)) => b1.cmp(b2),
            (CBORSpecialEnum::Float(f1), CBORSpecialEnum::Float(f2)) => f64_to_bytes(*f1).cmp(&f64_to_bytes(*f2)),
            (CBORSpecialEnum::Unassigned(u1), CBORSpecialEnum::Unassigned(u2)) => u1.cmp(u2),
            (CBORSpecialEnum::Break, CBORSpecialEnum::Break) |
            (CBORSpecialEnum::Undefined, CBORSpecialEnum::Undefined) |
            (CBORSpecialEnum::Null, CBORSpecialEnum::Null) => std::cmp::Ordering::Equal,
            _mixed_types => self.kind().cmp(&other.kind()),
        }
    }
}

impl std::hash::Hash for CBORSpecial {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.kind().hash(state);
        match &self.0 {
            CBORSpecialEnum::Bool(b) => b.hash(state),
            CBORSpecialEnum::Float(f) => f64_to_bytes(*f).hash(state),
            CBORSpecialEnum::Unassigned(u) => u.hash(state),
            _no_extra_data => (),
        }
    }
}

impl PartialOrd for CBORSpecial {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CBORValueKind {
    Int,
    Bytes,
    Text,
    Array,
    Object,
    TaggedCBOR,
    Special,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CBORValueEnum {
    Int(Int),
    Bytes(Vec<u8>),
    Text(String),
    Array(CBORArray),
    Object(CBORObject),
    TaggedCBOR(Box<TaggedCBOR>),
    Special(CBORSpecial),
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CBORValue(pub (crate) CBORValueEnum);

#[wasm_bindgen]
impl CBORValue {
    pub fn new_int(int: &Int) -> Self {
        Self(CBORValueEnum::Int(int.clone()))
    }

    pub fn new_bytes(bytes: Vec<u8>) -> Self {
        Self(CBORValueEnum::Bytes(bytes))
    }

    pub fn new_text(text: String) -> Self {
        Self(CBORValueEnum::Text(text))
    }

    pub fn new_array(arr: &CBORArray) -> Self {
        Self(CBORValueEnum::Array(arr.clone()))
    }

    pub fn new_object(obj: &CBORObject) -> Self {
        Self(CBORValueEnum::Object(obj.clone()))
    }

    pub fn new_tagged(tagged: &TaggedCBOR) -> Self {
        Self(CBORValueEnum::TaggedCBOR(Box::new(tagged.clone())))
    }

    pub fn new_special(special: &CBORSpecial) -> Self {
        Self(CBORValueEnum::Special(special.clone()))
    }

    pub fn kind(&self) -> CBORValueKind {
        match &self.0 {
            CBORValueEnum::Int(_) => CBORValueKind::Int,
            CBORValueEnum::Bytes(_) => CBORValueKind::Bytes,
            CBORValueEnum::Text(_) => CBORValueKind::Text,
            CBORValueEnum::Array(_) => CBORValueKind::Array,
            CBORValueEnum::Object(_) => CBORValueKind::Object,
            CBORValueEnum::TaggedCBOR(_) => CBORValueKind::TaggedCBOR,
            CBORValueEnum::Special(_) => CBORValueKind::Special,
        }
    }

    pub fn as_int(&self) -> Option<Int> {
        match &self.0 {
            CBORValueEnum::Int(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_bytes(&self) -> Option<Vec<u8>> {
        match &self.0 {
            CBORValueEnum::Bytes(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_text(&self) -> Option<String> {
        match &self.0 {
            CBORValueEnum::Text(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<CBORArray> {
        match &self.0 {
            CBORValueEnum::Array(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<CBORObject> {
        match &self.0 {
            CBORValueEnum::Object(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_tagged(&self) -> Option<TaggedCBOR> {
        use std::ops::Deref;
        match &self.0 {
            CBORValueEnum::TaggedCBOR(x) => Some((*x).deref().clone()),
            _ => None,
        }
    }

    pub fn as_special(&self) -> Option<CBORSpecial> {
        match &self.0 {
            CBORValueEnum::Special(x) => Some(x.clone()),
            _ => None,
        }
    }
}


// serialization

use std::io::{Seek, SeekFrom};

impl cbor_event::se::Serialize for TaggedCBOR {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.tag.serialize(serializer)?;
        self.value.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for TaggedCBOR {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let tag = (|| -> Result<_, DeserializeError> {
                Ok(BigNum::deserialize(raw)?)
            })().map_err(|e| e.annotate("tag"))?;
            let value = (|| -> Result<_, DeserializeError> {
                Ok(CBORValue::deserialize(raw)?)
            })().map_err(|e| e.annotate("value"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(TaggedCBOR {
                tag,
                value,
            })
        })().map_err(|e| e.annotate("TaggedCBOR"))
    }
}

impl cbor_event::se::Serialize for CBORArray {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        if self.definite {
            serializer.write_array(cbor_event::Len::Len(self.values.len() as u64))?;
        } else {
            serializer.write_array(cbor_event::Len::Indefinite)?;
        }
        for element in &self.values {
            element.serialize(serializer)?;
        }
        if !self.definite {
            serializer.write_special(cbor_event::Special::Break)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for CBORArray {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut arr = Vec::new();
        let definite = (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let definite = len != cbor_event::Len::Indefinite;
            while match len { cbor_event::Len::Len(n) => arr.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == cbor_event::Type::Special {
                    assert_eq!(raw.special()?, cbor_event::Special::Break);
                    break;
                }
                arr.push(CBORValue::deserialize(raw)?);
            }
            Ok(definite)
        })().map_err(|e| e.annotate("CBORArray"))?;
        Ok(Self {
            definite,
            values: arr,
        })
    }
}

impl cbor_event::se::Serialize for CBORObject {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        if self.definite {
            serializer.write_map(cbor_event::Len::Len(self.values.len() as u64))?;
        } else {
            serializer.write_map(cbor_event::Len::Indefinite)?;
        }
        for (key, value) in &self.values {
            key.serialize(serializer)?;
            value.serialize(serializer)?;
        }
        if !self.definite {
            serializer.write_special(cbor_event::Special::Break)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for CBORObject {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut table = LinkedHashMap::new();
        let definite = (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            let definite = len != cbor_event::Len::Indefinite;
            while match len { cbor_event::Len::Len(n) => table.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == cbor_event::Type::Special {
                    assert_eq!(raw.special()?, cbor_event::Special::Break);
                    break;
                }
                let key = CBORValue::deserialize(raw)?;
                let value = CBORValue::deserialize(raw)?;
                if table.insert(key.clone(), value).is_some() {
                    return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from("some complicated/unsupported type"))).into());
                }
            }
            Ok(definite)
        })().map_err(|e| e.annotate("CBORObject"))?;
        Ok(Self {
            definite,
            values: table,
        })
    }
}

impl cbor_event::se::Serialize for CBORSpecialEnum {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        let special = match self {
            CBORSpecialEnum::Bool(b) => cbor_event::Special::Bool(*b),
            CBORSpecialEnum::Float(f) => cbor_event::Special::Float(*f),
            CBORSpecialEnum::Unassigned(u) => cbor_event::Special::Unassigned(*u),
            CBORSpecialEnum::Break => cbor_event::Special::Break,
            CBORSpecialEnum::Undefined => cbor_event::Special::Undefined,
            CBORSpecialEnum::Null => cbor_event::Special::Null,
        };
        serializer.write_special(special)
    }
}

impl Deserialize for CBORSpecialEnum {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            Ok(match raw.special()? {
                cbor_event::Special::Bool(b) => CBORSpecialEnum::Bool(b),
                cbor_event::Special::Float(f) => CBORSpecialEnum::Float(f),
                cbor_event::Special::Unassigned(u) => CBORSpecialEnum::Unassigned(u),
                cbor_event::Special::Break => CBORSpecialEnum::Break,
                cbor_event::Special::Undefined => CBORSpecialEnum::Undefined,
                cbor_event::Special::Null => CBORSpecialEnum::Null,
            })
        })().map_err(|e| e.annotate("CBORSpecialEnum"))
    }
}

impl cbor_event::se::Serialize for CBORSpecial {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.0.serialize(serializer)
    }
}

impl Deserialize for CBORSpecial {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Ok(Self(CBORSpecialEnum::deserialize(raw)?))
    }
}

impl cbor_event::se::Serialize for CBORValueEnum {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            CBORValueEnum::Int(x) => {
                x.serialize(serializer)
            },
            CBORValueEnum::Bytes(x) => {
                serializer.write_bytes(&x)
            },
            CBORValueEnum::Text(x) => {
                serializer.write_text(&x)
            },
            CBORValueEnum::Array(x) => {
                x.serialize(serializer)
            },
            CBORValueEnum::Object(x) => {
                x.serialize(serializer)
            },
            CBORValueEnum::TaggedCBOR(x) => {
                x.serialize(serializer)
            },
            CBORValueEnum::Special(x) => {
                x.serialize(serializer)
            },
        }
    }
}

impl Deserialize for CBORValueEnum {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(Int::deserialize(raw)?)
            })(raw)
            {
                Ok(variant) => return Ok(CBORValueEnum::Int(variant)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(raw.bytes()?)
            })(raw)
            {
                Ok(variant) => return Ok(CBORValueEnum::Bytes(variant)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(String::deserialize(raw)?)
            })(raw)
            {
                Ok(variant) => return Ok(CBORValueEnum::Text(variant)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(CBORArray::deserialize(raw)?)
            })(raw)
            {
                Ok(variant) => return Ok(CBORValueEnum::Array(variant)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(CBORObject::deserialize(raw)?)
            })(raw)
            {
                Ok(variant) => return Ok(CBORValueEnum::Object(variant)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(TaggedCBOR::deserialize(raw)?)
            })(raw)
            {
                Ok(variant) => return Ok(CBORValueEnum::TaggedCBOR(Box::new(variant))),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(CBORSpecial::deserialize(raw)?)
            })(raw)
            {
                Ok(variant) => return Ok(CBORValueEnum::Special(variant)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            Err(DeserializeError::new("CBORValueEnum", DeserializeFailure::NoVariantMatched.into()))
        })().map_err(|e| e.annotate("CBORValueEnum"))
    }
}

impl cbor_event::se::Serialize for CBORValue {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.0.serialize(serializer)
    }
}

impl Deserialize for CBORValue {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Ok(Self(CBORValueEnum::deserialize(raw)?))
    }
}