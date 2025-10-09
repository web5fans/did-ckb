extern crate alloc;
use core::convert::TryInto;
use molecule::lazy_reader::{Cursor, Error, NUMBER_SIZE};
#[derive(Clone)]
pub struct Bytes {
    pub cursor: Cursor,
}
impl From<Cursor> for Bytes {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
impl Bytes {
    pub fn len(&self) -> Result<usize, Error> {
        self.cursor.fixvec_length()
    }
}
impl Bytes {
    pub fn get(&self, index: usize) -> Result<u8, Error> {
        let cur = self.cursor.fixvec_slice_by_index(1usize, index)?;
        cur.try_into()
    }
}
pub struct BytesIterator {
    cur: Bytes,
    index: usize,
    len: usize,
}
impl core::iter::Iterator for BytesIterator {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.len {
            None
        } else {
            let res = self.cur.get(self.index).unwrap();
            self.index += 1;
            Some(res)
        }
    }
}
impl core::iter::IntoIterator for Bytes {
    type Item = u8;
    type IntoIter = BytesIterator;
    fn into_iter(self) -> Self::IntoIter {
        let len = self.len().unwrap();
        Self::IntoIter {
            cur: self,
            index: 0,
            len,
        }
    }
}
pub struct BytesIteratorRef<'a> {
    cur: &'a Bytes,
    index: usize,
    len: usize,
}
impl<'a> core::iter::Iterator for BytesIteratorRef<'a> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.len {
            None
        } else {
            let res = self.cur.get(self.index).unwrap();
            self.index += 1;
            Some(res)
        }
    }
}
impl Bytes {
    pub fn iter(&self) -> BytesIteratorRef {
        let len = self.len().unwrap();
        BytesIteratorRef {
            cur: &self,
            index: 0,
            len,
        }
    }
}
impl Bytes {
    pub fn verify(&self, _compatible: bool) -> Result<(), Error> {
        self.cursor.verify_fixvec(1usize)?;
        Ok(())
    }
}
#[derive(Clone)]
pub struct String {
    pub cursor: Cursor,
}
impl From<Cursor> for String {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
impl String {
    pub fn len(&self) -> Result<usize, Error> {
        self.cursor.fixvec_length()
    }
}
impl String {
    pub fn get(&self, index: usize) -> Result<u8, Error> {
        let cur = self.cursor.fixvec_slice_by_index(1usize, index)?;
        cur.try_into()
    }
}
pub struct StringIterator {
    cur: String,
    index: usize,
    len: usize,
}
impl core::iter::Iterator for StringIterator {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.len {
            None
        } else {
            let res = self.cur.get(self.index).unwrap();
            self.index += 1;
            Some(res)
        }
    }
}
impl core::iter::IntoIterator for String {
    type Item = u8;
    type IntoIter = StringIterator;
    fn into_iter(self) -> Self::IntoIter {
        let len = self.len().unwrap();
        Self::IntoIter {
            cur: self,
            index: 0,
            len,
        }
    }
}
pub struct StringIteratorRef<'a> {
    cur: &'a String,
    index: usize,
    len: usize,
}
impl<'a> core::iter::Iterator for StringIteratorRef<'a> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.len {
            None
        } else {
            let res = self.cur.get(self.index).unwrap();
            self.index += 1;
            Some(res)
        }
    }
}
impl String {
    pub fn iter(&self) -> StringIteratorRef {
        let len = self.len().unwrap();
        StringIteratorRef {
            cur: &self,
            index: 0,
            len,
        }
    }
}
impl String {
    pub fn verify(&self, _compatible: bool) -> Result<(), Error> {
        self.cursor.verify_fixvec(1usize)?;
        Ok(())
    }
}
pub struct StringOpt {
    pub cursor: Cursor,
}
impl From<Cursor> for StringOpt {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
#[derive(Clone)]
pub struct DidCkbDataV1 {
    pub cursor: Cursor,
}
impl From<Cursor> for DidCkbDataV1 {
    fn from(cursor: Cursor) -> Self {
        DidCkbDataV1 { cursor }
    }
}
impl DidCkbDataV1 {
    pub fn document(&self) -> Result<Cursor, Error> {
        let cur = self.cursor.table_slice_by_index(0usize)?;
        cur.convert_to_rawbytes()
    }
}
impl DidCkbDataV1 {
    pub fn local_id(&self) -> Result<Option<Cursor>, Error> {
        let cur = self.cursor.table_slice_by_index(1usize)?;
        if cur.option_is_none() {
            Ok(None)
        } else {
            let cur = cur.convert_to_rawbytes()?;
            Ok(Some(cur.into()))
        }
    }
}
impl DidCkbDataV1 {
    pub fn verify(&self, compatible: bool) -> Result<(), Error> {
        self.cursor.verify_table(2usize, compatible)?;
        Ok(())
    }
}
pub enum DidCkbData {
    DidCkbDataV1(DidCkbDataV1),
}
impl TryFrom<Cursor> for DidCkbData {
    type Error = Error;
    fn try_from(cur: Cursor) -> Result<Self, Self::Error> {
        let item = cur.union_unpack()?;
        let mut cur = cur;
        cur.add_offset(NUMBER_SIZE)?;
        cur.sub_size(NUMBER_SIZE)?;
        match item.item_id {
            0usize => Ok(Self::DidCkbDataV1(cur.into())),
            _ => Err(Error::UnknownItem),
        }
    }
}
impl DidCkbData {
    pub fn verify(&self, compatible: bool) -> Result<(), Error> {
        match self {
            Self::DidCkbDataV1(v) => {
                v.verify(compatible)?;
                Ok(())
            }
        }
    }
}
