use std::{
    io::BufRead,
    mem::swap,
    ops::Deref,
    os::raw::{c_char, c_float, c_int},
    str::from_utf8,
};

use anyhow::{anyhow, Context, Error, Result};

pub trait Reader {
    type Buffer: Deref<Target = [u8]>;
    type String: Readable<Self> + ReaderString;
    fn read_into(&mut self, buf: &mut [u8]) -> Result<()>;
    fn read_bytes(&mut self, size: usize) -> Result<Self::Buffer>;
    fn read_until(&mut self, predicate: u8) -> Result<Self::Buffer>;
}

pub struct Slice<'a>(pub &'a [u8]);

impl<'a> Reader for Slice<'a> {
    type Buffer = &'a [u8];
    type String = &'a str;

    fn read_into(&mut self, buf: &mut [u8]) -> Result<()> {
        let (head, tail) = self.0.split_at(buf.len());
        buf.copy_from_slice(head);
        self.0 = tail;
        Ok(())
    }

    fn read_bytes(&mut self, size: usize) -> Result<&'a [u8]> {
        let (head, tail) = self.0.split_at(size);
        self.0 = tail;
        Ok(head)
    }

    fn read_until(&mut self, predicate: u8) -> Result<&'a [u8]> {
        let index = self
            .0
            .iter()
            .position(|byte| *byte == predicate)
            .ok_or_else(|| anyhow!("Delimiter {:?} not found in {:?}", predicate, self.0))?;

        let (head, tail) = self.0.split_at(index + 1);
        self.0 = tail;
        Ok(head)
    }
}

impl<R: BufRead> Reader for R {
    type Buffer = Vec<u8>;
    type String = String;

    fn read_into(&mut self, buf: &mut [u8]) -> Result<()> {
        self.read_exact(buf)?;
        Ok(())
    }

    fn read_bytes(&mut self, size: usize) -> Result<Vec<u8>> {
        let mut buffer = vec![0; size];
        self.read_exact(&mut buffer)?;
        Ok(buffer)
    }

    fn read_until(&mut self, predicate: u8) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        self.read_until(predicate, &mut buffer)?;
        Ok(buffer)
    }
}

pub trait ReaderString: Deref<Target = str> {
    fn split(&mut self, index: usize) -> Self;
}

impl<'a> ReaderString for &'a str {
    fn split(&mut self, index: usize) -> Self {
        let (head, tail) = self.split_at(index);
        *self = tail;
        head
    }
}

impl ReaderString for String {
    fn split(&mut self, index: usize) -> Self {
        let mut tail = self.split_off(index);
        swap(&mut tail, self);
        tail
    }
}

pub trait Readable<R: ?Sized>
where
    Self: Sized,
{
    fn read(reader: &mut R) -> Result<Self>;
}

macro_rules! impl_from_bytes {
    ( $ty:ty ) => {
        impl<R: Reader> Readable<R> for $ty {
            fn read(reader: &mut R) -> anyhow::Result<Self> {
                let mut bytes = [0; std::mem::size_of::<$ty>()];
                reader.read_into(&mut bytes)?;
                Ok(<$ty>::from_le_bytes(bytes))
            }
        }
    };
}

impl_from_bytes!(u8);
impl_from_bytes!(c_char);
impl_from_bytes!(c_int);
impl_from_bytes!(c_float);
impl_from_bytes!(u64);

impl<'a, R: Reader<Buffer = &'a [u8]>> Readable<R> for &'a str {
    fn read(reader: &mut R) -> Result<Self> {
        let bytes = reader.read_until(0)?;
        let bytes = &bytes[..bytes.len() - 1];
        from_utf8(bytes)
            .with_context(|| format!("Could not read utf-8 string from bytes {:?}", bytes))
    }
}

impl<'a, R: Reader<Buffer = Vec<u8>>> Readable<R> for String {
    fn read(reader: &mut R) -> Result<Self> {
        let mut vec = reader.read_until(0)?;
        vec.pop().unwrap();

        String::from_utf8(vec).map_err(|err| {
            let utf8_error = err.utf8_error();
            let message = format!(
                "Could not read utf-8 string from bytes {:?}",
                err.into_bytes()
            );

            Error::new(utf8_error).context(message)
        })
    }
}
