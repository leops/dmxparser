//! Implementation of a serde [Deserializer] from a [File]
use crate::dmx::{
    AttributeValue, Body, Color, File, Header, Qangle, Quaternion, StringRef, Time, Vector2,
    Vector3, Vector4, Vmatrix,
};
use serde::{
    de::{
        value::{
            BorrowedBytesDeserializer, BorrowedStrDeserializer, Error, MapDeserializer,
            SeqDeserializer, StringDeserializer as OwnedStrDeserializer,
        },
        DeserializeSeed, Deserializer, EnumAccess, Error as _, IntoDeserializer, MapAccess,
        VariantAccess, Visitor,
    },
    forward_to_deserialize_any, Deserialize,
};
use std::{
    convert::TryInto,
    fmt::Debug,
    iter::Cloned,
    os::raw::{c_char, c_float, c_int},
    slice::Iter,
    vec::IntoIter,
};

pub fn from_file<'de, B, S: Debug, T>(file: &'de File<B, S>) -> Result<T, Error>
where
    BufferWrapper<'de, B>: IntoDeserializer<'de>,
    StringWrapper<'de, S>: IntoDeserializer<'de>,
    T: Deserialize<'de>,
{
    let deserializer = ElementDeserializer {
        strings: &file.strings,
        headers: &file.headers,
        bodies: &file.bodies,
        index: 0,
    };

    T::deserialize(deserializer)
}

/// Deserialize a single element (header + body) from a file
struct ElementDeserializer<'de, B, S> {
    strings: &'de [S],
    headers: &'de [Header],
    bodies: &'de [Body<B, S>],
    index: i32,
}

impl<'de, B, S: Debug> Deserializer<'de> for ElementDeserializer<'de, B, S>
where
    BufferWrapper<'de, B>: IntoDeserializer<'de>,
    StringWrapper<'de, S>: IntoDeserializer<'de>,
{
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        if let Ok(index) = self.index.try_into() {
            visitor.visit_map(AttributesDeserializer {
                strings: self.strings,
                headers: self.headers,
                bodies: self.bodies,
                index,
                attr: 0,
            })
        } else {
            visitor.visit_none()
        }
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        if TryInto::<usize>::try_into(self.index).is_ok() {
            visitor.visit_enum(self)
        } else {
            visitor.visit_none()
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        if TryInto::<usize>::try_into(self.index).is_ok() {
            visitor.visit_some(self)
        } else {
            visitor.visit_none()
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf unit unit_struct newtype_struct seq tuple
        tuple_struct map struct identifier ignored_any
    }
}

// Enum values are not directly supported by the DMX format, but tagged enums are
// used for fields that can deserialize to multiple type (with the variant name being
// the AttributeType name)
impl<'de, B, S: Debug> EnumAccess<'de> for ElementDeserializer<'de, B, S>
where
    BufferWrapper<'de, B>: IntoDeserializer<'de>,
    StringWrapper<'de, S>: IntoDeserializer<'de>,
{
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let index: usize = self.index.try_into().unwrap();
        let head: &Header = &self.headers[index];

        let value = seed.deserialize(StringDeserializer {
            strings: self.strings,
            index: head.type_,
        })?;
        Ok((value, self))
    }
}

impl<'de, B, S: Debug> VariantAccess<'de> for ElementDeserializer<'de, B, S>
where
    BufferWrapper<'de, B>: IntoDeserializer<'de>,
    StringWrapper<'de, S>: IntoDeserializer<'de>,
{
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        unreachable!()
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        seed.deserialize(self)
    }

    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unreachable!()
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unreachable!()
    }
}

/// Deserialize a string from an index in the string table
struct StringDeserializer<'de, S> {
    strings: &'de [S],
    index: StringRef,
}

impl<'de, S> Deserializer<'de> for StringDeserializer<'de, S>
where
    StringWrapper<'de, S>: IntoDeserializer<'de>,
{
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        if let Some(index) = self.index.index() {
            StringWrapper(&self.strings[index])
                .into_deserializer()
                .deserialize_any(visitor)
        } else {
            visitor.visit_none()
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        if self.index.index().is_some() {
            visitor.visit_some(self)
        } else {
            visitor.visit_none()
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

/// Deserialize an Attribute
struct AttributesDeserializer<'de, B, S> {
    strings: &'de [S],
    headers: &'de [Header],
    bodies: &'de [Body<B, S>],
    index: usize,
    attr: usize,
}

impl<'de, B, S: Debug> MapAccess<'de> for AttributesDeserializer<'de, B, S>
where
    BufferWrapper<'de, B>: IntoDeserializer<'de>,
    StringWrapper<'de, S>: IntoDeserializer<'de>,
{
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Error>
    where
        K: DeserializeSeed<'de>,
    {
        let body = &self.bodies[self.index];

        if self.attr >= body.attributes.len() {
            return Ok(None);
        }

        let attr = &body.attributes[self.attr];

        if let Some(index) = attr.name.index() {
            let value = &self.strings[index];
            seed.deserialize(StringWrapper(value).into_deserializer())
                .map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Error>
    where
        V: DeserializeSeed<'de>,
    {
        let body = &self.bodies[self.index];
        let attr = &body.attributes[self.attr];
        self.attr += 1;

        seed.deserialize(ValueDeserializer {
            strings: self.strings,
            headers: self.headers,
            bodies: self.bodies,
            value: &attr.value,
        })
        .map_err(|err| {
            Error::custom(format!(
                "Could not deserialize attribute {:?}\n\ncaused by:\n{}",
                self.strings[attr.name.index().unwrap()],
                err
            ))
        })
    }
}

/// Deserialize an AttributeValue
struct ValueDeserializer<'de, B, S> {
    strings: &'de [S],
    headers: &'de [Header],
    bodies: &'de [Body<B, S>],
    value: &'de AttributeValue<B, S, StringRef>,
}

impl<'de, 'a, B, S: Debug> Deserializer<'de> for ValueDeserializer<'de, B, S>
where
    BufferWrapper<'de, B>: IntoDeserializer<'de>,
    StringWrapper<'de, S>: IntoDeserializer<'de>,
{
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            AttributeValue::Element(index) => {
                let deserializer = ElementDeserializer {
                    strings: self.strings,
                    headers: self.headers,
                    bodies: self.bodies,
                    index: *index,
                };

                deserializer.deserialize_any(visitor)
            }

            AttributeValue::Bool(value) => visitor.visit_bool(*value),
            AttributeValue::Uint8(value) => visitor.visit_u8(*value),
            AttributeValue::Int(value) => visitor.visit_i32(*value),
            AttributeValue::Uint64(value) => visitor.visit_u64(*value),
            AttributeValue::Float(value) => visitor.visit_f32(*value),

            AttributeValue::String(index) => {
                if let Some(index) = index.index() {
                    let value = &self.strings[index];
                    StringWrapper(value)
                        .into_deserializer()
                        .deserialize_any(visitor)
                } else {
                    visitor.visit_none()
                }
            }

            AttributeValue::Binary(value) => BufferWrapper(value)
                .into_deserializer()
                .deserialize_any(visitor),

            AttributeValue::Time(value) => value.into_deserializer().deserialize_any(visitor),
            AttributeValue::Color(value) => value.into_deserializer().deserialize_any(visitor),
            AttributeValue::Vector2(value) => value.into_deserializer().deserialize_any(visitor),
            AttributeValue::Vector3(value) => value.into_deserializer().deserialize_any(visitor),
            AttributeValue::Vector4(value) => value.into_deserializer().deserialize_any(visitor),
            AttributeValue::Qangle(value) => value.into_deserializer().deserialize_any(visitor),
            AttributeValue::Quaternion(value) => value.into_deserializer().deserialize_any(visitor),
            AttributeValue::Vmatrix(value) => value.into_deserializer().deserialize_any(visitor),

            AttributeValue::ElementArray(value) => {
                visitor.visit_seq(SeqDeserializer::new(value.iter().map(|index| {
                    ElementWrapper {
                        strings: self.strings,
                        headers: self.headers,
                        bodies: self.bodies,
                        index: *index,
                    }
                })))
            }
            AttributeValue::IntArray(value) => {
                visitor.visit_seq(SeqDeserializer::new(value.iter().cloned()))
            }
            AttributeValue::FloatArray(value) => {
                visitor.visit_seq(SeqDeserializer::new(value.iter().cloned()))
            }
            AttributeValue::BoolArray(value) => {
                visitor.visit_seq(SeqDeserializer::new(value.iter().cloned()))
            }
            AttributeValue::StringArray(value) => {
                visitor.visit_seq(SeqDeserializer::new(value.iter().map(StringWrapper)))
            }
            AttributeValue::BinaryArray(value) => {
                visitor.visit_seq(SeqDeserializer::new(value.iter().map(BufferWrapper)))
            }
            AttributeValue::TimeArray(value) => {
                visitor.visit_seq(SeqDeserializer::new(value.iter()))
            }
            AttributeValue::ColorArray(value) => {
                visitor.visit_seq(SeqDeserializer::new(value.iter()))
            }
            AttributeValue::Vector2Array(value) => {
                visitor.visit_seq(SeqDeserializer::new(value.iter()))
            }
            AttributeValue::Vector3Array(value) => {
                visitor.visit_seq(SeqDeserializer::new(value.iter()))
            }
            AttributeValue::Vector4Array(value) => {
                visitor.visit_seq(SeqDeserializer::new(value.iter()))
            }
            AttributeValue::QangleArray(value) => {
                visitor.visit_seq(SeqDeserializer::new(value.iter()))
            }
            AttributeValue::QuaternionArray(value) => {
                visitor.visit_seq(SeqDeserializer::new(value.iter()))
            }
            AttributeValue::VmatrixArray(value) => {
                visitor.visit_seq(SeqDeserializer::new(value.iter()))
            }
            AttributeValue::Uint64Array(value) => {
                visitor.visit_seq(SeqDeserializer::new(value.iter().cloned()))
            }
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            AttributeValue::String(index) => {
                let deserializer = StringDeserializer {
                    strings: self.strings,
                    index: *index,
                };

                deserializer.deserialize_option(visitor)
            }
            AttributeValue::Element(index) => {
                let deserializer = ElementDeserializer {
                    strings: self.strings,
                    headers: self.headers,
                    bodies: self.bodies,
                    index: *index,
                };

                deserializer.deserialize_option(visitor)
            }
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        if let AttributeValue::Element(index) = self.value {
            let deserializer = ElementDeserializer {
                strings: self.strings,
                headers: self.headers,
                bodies: self.bodies,
                index: *index,
            };

            deserializer.deserialize_enum(name, variants, visitor)
        } else {
            visitor.visit_enum(self)
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char
        str string bytes byte_buf
        unit unit_struct newtype_struct seq tuple
        tuple_struct map struct identifier ignored_any
    }
}

impl<'de, 'a, B, S: Debug> EnumAccess<'de> for ValueDeserializer<'de, B, S>
where
    BufferWrapper<'de, B>: IntoDeserializer<'de>,
    StringWrapper<'de, S>: IntoDeserializer<'de>,
{
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        Ok((
            seed.deserialize(BorrowedStrDeserializer::new(self.value.kind().name()))?,
            self,
        ))
    }
}

impl<'de, 'a, B, S: Debug> VariantAccess<'de> for ValueDeserializer<'de, B, S>
where
    BufferWrapper<'de, B>: IntoDeserializer<'de>,
    StringWrapper<'de, S>: IntoDeserializer<'de>,
{
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        unreachable!()
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        seed.deserialize(self)
    }

    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unreachable!()
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unreachable!()
    }
}

/// IntoDeserializer implementation for ElementDeserializer
struct ElementWrapper<'de, B, S> {
    strings: &'de [S],
    headers: &'de [Header],
    bodies: &'de [Body<B, S>],
    index: i32,
}

impl<'de, B, S: Debug> IntoDeserializer<'de> for ElementWrapper<'de, B, S>
where
    BufferWrapper<'de, B>: IntoDeserializer<'de>,
    StringWrapper<'de, S>: IntoDeserializer<'de>,
{
    type Deserializer = ElementDeserializer<'de, B, S>;

    fn into_deserializer(self) -> Self::Deserializer {
        ElementDeserializer {
            strings: self.strings,
            headers: self.headers,
            bodies: self.bodies,
            index: self.index,
        }
    }
}

/// Implementation detail: provides owned or borrowed byte array deserialization
/// depending on the type of the input file
#[doc(hidden)]
pub struct BufferWrapper<'de, T>(&'de T);

impl<'de> IntoDeserializer<'de> for BufferWrapper<'de, &'de [u8]> {
    type Deserializer = BorrowedBytesDeserializer<'de, Error>;

    fn into_deserializer(self) -> Self::Deserializer {
        BorrowedBytesDeserializer::new(self.0)
    }
}

impl<'de> IntoDeserializer<'de> for BufferWrapper<'de, Vec<u8>> {
    type Deserializer = SeqDeserializer<IntoIter<u8>, Error>;

    fn into_deserializer(self) -> Self::Deserializer {
        self.0.clone().into_deserializer()
    }
}

/// Implementation detail: provides owned or borrowed string deserialization
/// depending on the type of the input file
#[doc(hidden)]
pub struct StringWrapper<'de, T>(&'de T);

impl<'de> IntoDeserializer<'de> for StringWrapper<'de, &'de str> {
    type Deserializer = BorrowedStrDeserializer<'de, Error>;

    fn into_deserializer(self) -> Self::Deserializer {
        BorrowedStrDeserializer::new(self.0)
    }
}

impl<'de> IntoDeserializer<'de> for StringWrapper<'de, String> {
    type Deserializer = OwnedStrDeserializer<Error>;

    fn into_deserializer(self) -> Self::Deserializer {
        self.0.clone().into_deserializer()
    }
}

impl<'de> IntoDeserializer<'de> for &'de Time {
    type Deserializer = MapDeserializer<'de, IntoIter<(&'static str, c_int)>, Error>;

    fn into_deserializer(self) -> Self::Deserializer {
        MapDeserializer::new(vec![("millis", self.millis)].into_iter())
    }
}

impl<'de> IntoDeserializer<'de> for &'de Color {
    type Deserializer = MapDeserializer<'de, IntoIter<(&'static str, c_char)>, Error>;

    fn into_deserializer(self) -> Self::Deserializer {
        MapDeserializer::new(
            vec![("r", self.r), ("g", self.g), ("b", self.b), ("a", self.a)].into_iter(),
        )
    }
}

impl<'de> IntoDeserializer<'de> for &'de Vector2 {
    type Deserializer = MapDeserializer<'de, IntoIter<(&'static str, c_float)>, Error>;

    fn into_deserializer(self) -> Self::Deserializer {
        MapDeserializer::new(vec![("x", self.x), ("y", self.y)].into_iter())
    }
}

impl<'de> IntoDeserializer<'de> for &'de Vector3 {
    type Deserializer = MapDeserializer<'de, IntoIter<(&'static str, c_float)>, Error>;

    fn into_deserializer(self) -> Self::Deserializer {
        MapDeserializer::new(vec![("x", self.x), ("y", self.y), ("z", self.z)].into_iter())
    }
}

impl<'de> IntoDeserializer<'de> for &'de Vector4 {
    type Deserializer = MapDeserializer<'de, IntoIter<(&'static str, c_float)>, Error>;

    fn into_deserializer(self) -> Self::Deserializer {
        MapDeserializer::new(
            vec![("x", self.x), ("y", self.y), ("z", self.z), ("w", self.w)].into_iter(),
        )
    }
}

impl<'de> IntoDeserializer<'de> for &'de Qangle {
    type Deserializer = MapDeserializer<'de, IntoIter<(&'static str, c_float)>, Error>;

    fn into_deserializer(self) -> Self::Deserializer {
        MapDeserializer::new(
            vec![
                ("pitch", self.pitch),
                ("yaw", self.yaw),
                ("roll", self.roll),
            ]
            .into_iter(),
        )
    }
}

impl<'de> IntoDeserializer<'de> for &'de Quaternion {
    type Deserializer = MapDeserializer<'de, IntoIter<(&'static str, c_float)>, Error>;

    fn into_deserializer(self) -> Self::Deserializer {
        MapDeserializer::new(
            vec![("x", self.x), ("y", self.y), ("z", self.z), ("w", self.w)].into_iter(),
        )
    }
}

impl<'de> IntoDeserializer<'de> for &'de Vmatrix {
    type Deserializer = SeqDeserializer<Cloned<Iter<'de, f32>>, Error>;

    fn into_deserializer(self) -> Self::Deserializer {
        SeqDeserializer::new(self.0.iter().cloned())
    }
}
