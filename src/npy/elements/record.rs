use crate::{ReadDataError, ReadableElement};
use py_literal::Value as PyValue;

pub trait RecordFromSlice: Sized {
    const SIZE: usize;

    // TODO(perf): validate type_descr once?

    fn from_raw_slice<R: std::io::Read>(
        type_descr: &PyValue,
        reader: &mut R,
    ) -> Result<Self, ReadDataError>;
}

impl<T: RecordFromSlice> ReadableElement for T {
    fn read_to_end_exact_vec<R: std::io::Read>(
        mut reader: R,
        type_desc: &PyValue,
        len: usize,
    ) -> Result<Vec<Self>, ReadDataError> {
        let mut out = Vec::new(); // NOTE(perf): in theory could be MaybeUinitVec?

        for _ in 0..len {
            out.push(RecordFromSlice::from_raw_slice(type_desc, &mut reader)?);
        }

        Ok(out)
    }
}
