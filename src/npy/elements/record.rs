use crate::{ReadDataError, ReadableElement};
use py_literal::Value as PyValue;

pub trait RecordFromSlice: Sized {
    fn compatible_schema(type_descr: &PyValue) -> bool;

    fn from_raw_slice<R: std::io::Read>(reader: &mut R) -> Result<Self, ReadDataError>;
}

impl<T: RecordFromSlice> ReadableElement for T {
    fn read_to_end_exact_vec<R: std::io::Read>(
        mut reader: R,
        type_desc: &PyValue,
        len: usize,
    ) -> Result<Vec<Self>, ReadDataError> {
        if !T::compatible_schema(type_desc) {
            return Err(ReadDataError::WrongDescriptor(type_desc.clone()));
        }

        let mut out = Vec::with_capacity(len);
        for _ in 0..len {
            out.push(RecordFromSlice::from_raw_slice(&mut reader)?);
        }

        Ok(out)
    }
}
