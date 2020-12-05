//! Data structures used to represent a DMX file in memory
use serde::{Deserialize, Serialize};
use std::{
    convert::TryInto,
    fmt::{self, Debug, Display, Formatter},
    os::raw::{c_char, c_float, c_int},
};

use anyhow::{anyhow, bail, Context, Result};

use crate::read::{Readable, Reader, ReaderString};

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum AttributeType {
    Element,
    Int,
    Float,
    Bool,
    String,
    Binary,
    Time,
    Color,
    Vector2,
    Vector3,
    Vector4,
    Qangle,
    Quaternion,
    Vmatrix,
    Uint64,
    Uint8,

    ElementArray,
    IntArray,
    FloatArray,
    BoolArray,
    StringArray,
    BinaryArray,
    TimeArray,
    ColorArray,
    Vector2Array,
    Vector3Array,
    Vector4Array,
    QangleArray,
    QuaternionArray,
    VmatrixArray,
    Uint64Array,
}

impl AttributeType {
    pub(crate) fn name(&self) -> &'static str {
        match self {
            AttributeType::Element => "Element",
            AttributeType::Int => "Int",
            AttributeType::Float => "Float",
            AttributeType::Bool => "Bool",
            AttributeType::String => "String",
            AttributeType::Binary => "Binary",
            AttributeType::Time => "Time",
            AttributeType::Color => "Color",
            AttributeType::Vector2 => "Vector2",
            AttributeType::Vector3 => "Vector3",
            AttributeType::Vector4 => "Vector4",
            AttributeType::Qangle => "Qangle",
            AttributeType::Quaternion => "Quaternion",
            AttributeType::Vmatrix => "Vmatrix",
            AttributeType::Uint64 => "Uint64",
            AttributeType::Uint8 => "Uint8",

            AttributeType::ElementArray => "ElementArray",
            AttributeType::IntArray => "IntArray",
            AttributeType::FloatArray => "FloatArray",
            AttributeType::BoolArray => "BoolArray",
            AttributeType::StringArray => "StringArray",
            AttributeType::BinaryArray => "BinaryArray",
            AttributeType::TimeArray => "TimeArray",
            AttributeType::ColorArray => "ColorArray",
            AttributeType::Vector2Array => "Vector2Array",
            AttributeType::Vector3Array => "Vector3Array",
            AttributeType::Vector4Array => "Vector4Array",
            AttributeType::QangleArray => "QangleArray",
            AttributeType::QuaternionArray => "QuaternionArray",
            AttributeType::VmatrixArray => "VmatrixArray",
            AttributeType::Uint64Array => "Uint64Array",
        }
    }
}

impl<R: Reader> Readable<R> for AttributeType {
    fn read(reader: &mut R) -> Result<Self> {
        Ok(match u8::read(reader)? {
            1 => AttributeType::Element,
            2 => AttributeType::Int,
            3 => AttributeType::Float,
            4 => AttributeType::Bool,
            5 => AttributeType::String,
            6 => AttributeType::Binary,
            7 => AttributeType::Time,
            8 => AttributeType::Color,
            9 => AttributeType::Vector2,
            10 => AttributeType::Vector3,
            11 => AttributeType::Vector4,
            12 => AttributeType::Qangle,
            13 => AttributeType::Quaternion,
            14 => AttributeType::Vmatrix,
            15 => AttributeType::Uint64,
            16 => AttributeType::Uint8,

            33 => AttributeType::ElementArray,
            34 => AttributeType::IntArray,
            35 => AttributeType::FloatArray,
            36 => AttributeType::BoolArray,
            37 => AttributeType::StringArray,
            38 => AttributeType::BinaryArray,
            39 => AttributeType::TimeArray,
            40 => AttributeType::ColorArray,
            41 => AttributeType::Vector2Array,
            42 => AttributeType::Vector3Array,
            43 => AttributeType::Vector4Array,
            44 => AttributeType::QangleArray,
            45 => AttributeType::QuaternionArray,
            46 => AttributeType::VmatrixArray,
            47 => AttributeType::Uint64Array,

            value => bail!("unimplemented AttributeType {}", value),
        })
    }
}

#[derive(Debug)]
pub struct File<B, S> {
    pub header: FileHeader<S>,
    pub prefix: Vec<(S, AttributeValue<B, S>)>,
    pub strings: Vec<S>,
    pub headers: Vec<Header>,
    pub bodies: Vec<Body<B, S>>,
}

impl<R> Readable<R> for File<R::Buffer, R::String>
where
    R: Reader,
    R::String: Debug,
{
    fn read(reader: &mut R) -> Result<Self> {
        let header = FileHeader::read(reader)?;

        assert_eq!(&*header.encoding_name, "binary");
        assert_eq!(header.encoding_version, 9);

        let _padding = c_int::read(reader)?;

        let n_prefix = c_int::read(reader)?;
        let prefix = (0..n_prefix)
            .map(|_| {
                let name = R::String::read(reader)?;

                let value = AttributeValue::read(reader)
                    .with_context(|| format!("Failed to read attribute {:?}", name))?;

                Ok((name, value))
            })
            .collect::<Result<_>>()?;

        let n_strings = c_int::read(reader)?;
        let strings: Vec<_> = (0..n_strings)
            .map(|_| R::String::read(reader))
            .collect::<Result<_>>()?;

        let n_elements = c_int::read(reader)?;
        let headers: Vec<_> = (0..n_elements)
            .map(|_| Header::read(reader))
            .collect::<Result<_>>()?;

        let bodies: Vec<_> = (0..n_elements)
            .map(|_| Body::read(reader))
            .collect::<Result<_>>()
            .map_err(|mut err| {
                if let Some(err) = err.downcast_mut::<AttributeError>() {
                    err.1 = Some(format!("{:?}", strings[(err.0).0 as usize]));
                }
                err
            })?;

        Ok(File {
            header,
            prefix,
            strings,
            headers,
            bodies,
        })
    }
}

#[derive(Debug)]
pub struct FileHeader<S> {
    pub encoding_name: S,
    pub encoding_version: c_int,
    pub format_name: S,
    pub format_version: c_int,
}

impl<R: Reader> Readable<R> for FileHeader<R::String>
where
    R::String: Debug,
{
    fn read(reader: &mut R) -> Result<Self> {
        static OPEN_TOKEN: &'static str = "<!-- dmx encoding ";
        static FORMAT_TOKEN: &'static str = " format ";
        static CLOSE_TOKEN: &'static str = " -->\n";

        fn trim_start<'a, S: ReaderString>(value: &mut S, head: &str) {
            assert!(value.starts_with(head));
            value.split(head.len());
        }

        fn split_at<'a, S: ReaderString + Debug>(value: &mut S, sep: char) -> Result<S> {
            let index = value
                .find(sep)
                .ok_or_else(|| anyhow!("could not find separator {:?} in {:?}", sep, value))?;
            Ok(value.split(index))
        }

        let mut value = R::String::read(reader)?;

        trim_start(&mut value, OPEN_TOKEN);

        let encoding_name = split_at(&mut value, ' ')?;
        trim_start(&mut value, " ");

        let encoding_version = split_at(&mut value, ' ')?;
        let encoding_version = encoding_version.parse()?;

        trim_start(&mut value, FORMAT_TOKEN);

        let format_name = split_at(&mut value, ' ')?;
        trim_start(&mut value, " ");

        let format_version = split_at(&mut value, ' ')?;
        let format_version = format_version.parse()?;

        trim_start(&mut value, CLOSE_TOKEN);

        Ok(FileHeader {
            encoding_name,
            encoding_version,
            format_name,
            format_version,
        })
    }
}

#[derive(Debug)]
pub struct Header {
    pub type_: StringRef,
    pub name: StringRef,
    pub guid: [u8; 16],
}

impl<R: Reader> Readable<R> for Header {
    fn read(reader: &mut R) -> Result<Self> {
        let type_ = StringRef::read(reader)?;
        let name = StringRef::read(reader)?;

        let mut guid = [0; 16];
        reader.read_into(&mut guid)?;

        Ok(Header { type_, name, guid })
    }
}

#[derive(Debug)]
pub struct Body<B, S> {
    pub attributes: Vec<Attribute<B, S>>,
}

impl<R: Reader> Readable<R> for Body<R::Buffer, R::String> {
    fn read(reader: &mut R) -> Result<Self> {
        let n_attributes = c_int::read(reader)?;

        Ok(Body {
            attributes: (0..n_attributes)
                .map(|_| Attribute::read(reader))
                .collect::<Result<_>>()?,
        })
    }
}

#[derive(Debug)]
pub struct Attribute<B, S> {
    pub name: StringRef,
    pub value: AttributeValue<B, S, StringRef>,
}

impl<R: Reader> Readable<R> for Attribute<R::Buffer, R::String> {
    fn read(reader: &mut R) -> Result<Self> {
        let name = StringRef::read(reader)?;
        let value = AttributeValue::read(reader).with_context(|| AttributeError(name, None))?;
        Ok(Attribute { name, value })
    }
}

#[derive(Debug)]
struct AttributeError(StringRef, Option<String>);

impl Display for AttributeError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        if let Some(name) = &self.1 {
            write!(fmt, "Failed to read attribute {}", name)
        } else {
            write!(fmt, "Failed to read attribute {:?}", self.0)
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct StringRef(pub c_int);

impl StringRef {
    pub fn index(&self) -> Option<usize> {
        self.0.try_into().ok()
    }
}

impl<R: Reader> Readable<R> for StringRef {
    fn read(reader: &mut R) -> Result<Self> {
        Ok(StringRef(c_int::read(reader)?))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Time {
    pub millis: c_int,
}

impl<R: Reader> Readable<R> for Time {
    fn read(reader: &mut R) -> Result<Self> {
        Ok(Time {
            millis: c_int::read(reader)?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Color {
    pub r: c_char,
    pub g: c_char,
    pub b: c_char,
    pub a: c_char,
}

impl<R: Reader> Readable<R> for Color {
    fn read(reader: &mut R) -> Result<Self> {
        let r = c_char::read(reader)?;
        let g = c_char::read(reader)?;
        let b = c_char::read(reader)?;
        let a = c_char::read(reader)?;
        Ok(Color { r, g, b, a })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vector2 {
    pub x: c_float,
    pub y: c_float,
}

impl<R: Reader> Readable<R> for Vector2 {
    fn read(reader: &mut R) -> Result<Self> {
        let x = c_float::read(reader)?;
        let y = c_float::read(reader)?;
        Ok(Vector2 { x, y })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vector3 {
    pub x: c_float,
    pub y: c_float,
    pub z: c_float,
}

impl<R: Reader> Readable<R> for Vector3 {
    fn read(reader: &mut R) -> Result<Self> {
        let x = c_float::read(reader)?;
        let y = c_float::read(reader)?;
        let z = c_float::read(reader)?;
        Ok(Vector3 { x, y, z })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vector4 {
    pub x: c_float,
    pub y: c_float,
    pub z: c_float,
    pub w: c_float,
}

impl<R: Reader> Readable<R> for Vector4 {
    fn read(reader: &mut R) -> Result<Self> {
        let x = c_float::read(reader)?;
        let y = c_float::read(reader)?;
        let z = c_float::read(reader)?;
        let w = c_float::read(reader)?;
        Ok(Vector4 { x, y, z, w })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Qangle {
    pub pitch: c_float,
    pub yaw: c_float,
    pub roll: c_float,
}

impl<R: Reader> Readable<R> for Qangle {
    fn read(reader: &mut R) -> Result<Self> {
        let pitch = c_float::read(reader)?;
        let yaw = c_float::read(reader)?;
        let roll = c_float::read(reader)?;
        Ok(Qangle { pitch, yaw, roll })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Quaternion {
    pub x: c_float,
    pub y: c_float,
    pub z: c_float,
    pub w: c_float,
}

impl<R: Reader> Readable<R> for Quaternion {
    fn read(reader: &mut R) -> Result<Self> {
        let x = c_float::read(reader)?;
        let y = c_float::read(reader)?;
        let z = c_float::read(reader)?;
        let w = c_float::read(reader)?;
        Ok(Quaternion { x, y, z, w })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vmatrix(pub [c_float; 16]);

impl<R: Reader> Readable<R> for Vmatrix {
    fn read(reader: &mut R) -> Result<Self> {
        let mut matrix = [0.0; 16];
        for item in matrix.iter_mut() {
            *item = c_float::read(reader)?;
        }

        Ok(Vmatrix(matrix))
    }
}

#[derive(Debug)]
pub enum AttributeValue<B, S, R = S> {
    Element(c_int),
    Int(c_int),
    Float(c_float),
    Bool(bool),
    String(R),
    Binary(B),
    Time(Time),
    Color(Color),
    Vector2(Vector2),
    Vector3(Vector3),
    Vector4(Vector4),
    Qangle(Qangle),
    Quaternion(Quaternion),
    Vmatrix(Vmatrix),
    Uint64(u64),
    Uint8(u8),

    ElementArray(Vec<c_int>),
    IntArray(Vec<c_int>),
    FloatArray(Vec<c_float>),
    BoolArray(Vec<bool>),
    StringArray(Vec<S>),
    BinaryArray(Vec<B>),
    TimeArray(Vec<Time>),
    ColorArray(Vec<Color>),
    Vector2Array(Vec<Vector2>),
    Vector3Array(Vec<Vector3>),
    Vector4Array(Vec<Vector4>),
    QangleArray(Vec<Qangle>),
    QuaternionArray(Vec<Quaternion>),
    VmatrixArray(Vec<Vmatrix>),
    Uint64Array(Vec<u64>),
}

impl<B, S, R> AttributeValue<B, S, R> {
    pub fn kind(&self) -> AttributeType {
        match self {
            AttributeValue::Element(_) => AttributeType::Element,
            AttributeValue::Int(_) => AttributeType::Int,
            AttributeValue::Float(_) => AttributeType::Float,
            AttributeValue::Bool(_) => AttributeType::Bool,
            AttributeValue::String(_) => AttributeType::String,
            AttributeValue::Binary(_) => AttributeType::Binary,
            AttributeValue::Time(_) => AttributeType::Time,
            AttributeValue::Color(_) => AttributeType::Color,
            AttributeValue::Vector2(_) => AttributeType::Vector2,
            AttributeValue::Vector3(_) => AttributeType::Vector3,
            AttributeValue::Vector4(_) => AttributeType::Vector4,
            AttributeValue::Qangle(_) => AttributeType::Qangle,
            AttributeValue::Quaternion(_) => AttributeType::Quaternion,
            AttributeValue::Vmatrix(_) => AttributeType::Vmatrix,
            AttributeValue::Uint64(_) => AttributeType::Uint64,
            AttributeValue::Uint8(_) => AttributeType::Uint8,

            AttributeValue::ElementArray(_) => AttributeType::ElementArray,
            AttributeValue::IntArray(_) => AttributeType::IntArray,
            AttributeValue::FloatArray(_) => AttributeType::FloatArray,
            AttributeValue::BoolArray(_) => AttributeType::BoolArray,
            AttributeValue::StringArray(_) => AttributeType::StringArray,
            AttributeValue::BinaryArray(_) => AttributeType::BinaryArray,
            AttributeValue::TimeArray(_) => AttributeType::TimeArray,
            AttributeValue::ColorArray(_) => AttributeType::ColorArray,
            AttributeValue::Vector2Array(_) => AttributeType::Vector2Array,
            AttributeValue::Vector3Array(_) => AttributeType::Vector3Array,
            AttributeValue::Vector4Array(_) => AttributeType::Vector4Array,
            AttributeValue::QangleArray(_) => AttributeType::QangleArray,
            AttributeValue::QuaternionArray(_) => AttributeType::QuaternionArray,
            AttributeValue::VmatrixArray(_) => AttributeType::VmatrixArray,
            AttributeValue::Uint64Array(_) => AttributeType::Uint64Array,
        }
    }
}

impl<R: Reader, S: Readable<R>> Readable<R> for AttributeValue<R::Buffer, R::String, S> {
    fn read(reader: &mut R) -> Result<Self> {
        Ok(match AttributeType::read(reader)? {
            AttributeType::Element => AttributeValue::Element(c_int::read(reader)?),
            AttributeType::Int => AttributeValue::Int(c_int::read(reader)?),
            AttributeType::Float => AttributeValue::Float(c_float::read(reader)?),
            AttributeType::Bool => AttributeValue::Bool(u8::read(reader)? != 0),
            AttributeType::String => AttributeValue::String(S::read(reader)?),
            AttributeType::Binary => {
                let size = c_int::read(reader)?;
                AttributeValue::Binary(reader.read_bytes(size as usize)?)
            }
            AttributeType::Time => AttributeValue::Time(Time::read(reader)?),
            AttributeType::Color => AttributeValue::Color(Color::read(reader)?),
            AttributeType::Vector2 => AttributeValue::Vector2(Vector2::read(reader)?),
            AttributeType::Vector3 => AttributeValue::Vector3(Vector3::read(reader)?),
            AttributeType::Vector4 => AttributeValue::Vector4(Vector4::read(reader)?),
            AttributeType::Qangle => AttributeValue::Qangle(Qangle::read(reader)?),
            AttributeType::Quaternion => AttributeValue::Quaternion(Quaternion::read(reader)?),
            AttributeType::Vmatrix => AttributeValue::Vmatrix(Vmatrix::read(reader)?),
            AttributeType::Uint64 => AttributeValue::Uint64(u64::read(reader)?),
            AttributeType::Uint8 => AttributeValue::Uint8(u8::read(reader)?),

            AttributeType::ElementArray => AttributeValue::ElementArray({
                let size = c_int::read(reader)?;
                (0..size)
                    .map(|_| c_int::read(reader))
                    .collect::<Result<_>>()?
            }),
            AttributeType::IntArray => AttributeValue::IntArray({
                let size = c_int::read(reader)?;
                (0..size)
                    .map(|_| c_int::read(reader))
                    .collect::<Result<_>>()?
            }),
            AttributeType::FloatArray => AttributeValue::FloatArray({
                let size = c_int::read(reader)?;
                (0..size)
                    .map(|_| c_float::read(reader))
                    .collect::<Result<_>>()?
            }),
            AttributeType::BoolArray => AttributeValue::BoolArray({
                let size = c_int::read(reader)?;
                (0..size)
                    .map(|_| Ok(u8::read(reader)? != 0))
                    .collect::<Result<_>>()?
            }),
            AttributeType::StringArray => AttributeValue::StringArray({
                let size = c_int::read(reader)?;
                (0..size)
                    .map(|_| R::String::read(reader))
                    .collect::<Result<_>>()?
            }),
            AttributeType::BinaryArray => AttributeValue::BinaryArray({
                let size = c_int::read(reader)?;
                (0..size)
                    .map(|_| {
                        let size = c_int::read(reader)?;
                        reader.read_bytes(size as usize)
                    })
                    .collect::<Result<_>>()?
            }),
            AttributeType::TimeArray => AttributeValue::TimeArray({
                let size = c_int::read(reader)?;
                (0..size)
                    .map(|_| Time::read(reader))
                    .collect::<Result<_>>()?
            }),
            AttributeType::ColorArray => AttributeValue::ColorArray({
                let size = c_int::read(reader)?;
                (0..size)
                    .map(|_| Color::read(reader))
                    .collect::<Result<_>>()?
            }),
            AttributeType::Vector2Array => AttributeValue::Vector2Array({
                let size = c_int::read(reader)?;
                (0..size)
                    .map(|_| Vector2::read(reader))
                    .collect::<Result<_>>()?
            }),
            AttributeType::Vector3Array => AttributeValue::Vector3Array({
                let size = c_int::read(reader)?;
                (0..size)
                    .map(|_| Vector3::read(reader))
                    .collect::<Result<_>>()?
            }),
            AttributeType::Vector4Array => AttributeValue::Vector4Array({
                let size = c_int::read(reader)?;
                (0..size)
                    .map(|_| Vector4::read(reader))
                    .collect::<Result<_>>()?
            }),
            AttributeType::QangleArray => AttributeValue::QangleArray({
                let size = c_int::read(reader)?;
                (0..size)
                    .map(|_| Qangle::read(reader))
                    .collect::<Result<_>>()?
            }),
            AttributeType::QuaternionArray => AttributeValue::QuaternionArray({
                let size = c_int::read(reader)?;
                (0..size)
                    .map(|_| Quaternion::read(reader))
                    .collect::<Result<_>>()?
            }),
            AttributeType::VmatrixArray => AttributeValue::VmatrixArray({
                let size = c_int::read(reader)?;
                (0..size)
                    .map(|_| Vmatrix::read(reader))
                    .collect::<Result<_>>()?
            }),
            AttributeType::Uint64Array => AttributeValue::Uint64Array({
                let size = c_int::read(reader)?;
                (0..size)
                    .map(|_| u64::read(reader))
                    .collect::<Result<_>>()?
            }),
        })
    }
}
