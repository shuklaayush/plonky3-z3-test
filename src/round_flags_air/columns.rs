use core::borrow::{Borrow, BorrowMut};
use core::mem::size_of;

use super::NUM_ROUNDS;

#[repr(C)]
pub(crate) struct KeccakCols<T> {
    pub step_flags: [T; NUM_ROUNDS],
}

pub(crate) const NUM_KECCAK_COLS: usize = size_of::<KeccakCols<u8>>();

impl<T> Borrow<KeccakCols<T>> for [T] {
    fn borrow(&self) -> &KeccakCols<T> {
        debug_assert_eq!(self.len(), NUM_KECCAK_COLS);
        let (prefix, shorts, suffix) = unsafe { self.align_to::<KeccakCols<T>>() };
        debug_assert!(prefix.is_empty(), "Alignment should match");
        debug_assert!(suffix.is_empty(), "Alignment should match");
        debug_assert_eq!(shorts.len(), 1);
        &shorts[0]
    }
}

impl<T> BorrowMut<KeccakCols<T>> for [T] {
    fn borrow_mut(&mut self) -> &mut KeccakCols<T> {
        debug_assert_eq!(self.len(), NUM_KECCAK_COLS);
        let (prefix, shorts, suffix) = unsafe { self.align_to_mut::<KeccakCols<T>>() };
        debug_assert!(prefix.is_empty(), "Alignment should match");
        debug_assert!(suffix.is_empty(), "Alignment should match");
        debug_assert_eq!(shorts.len(), 1);
        &mut shorts[0]
    }
}
