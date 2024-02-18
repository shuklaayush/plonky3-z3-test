use core::borrow::{Borrow, BorrowMut};
use core::mem::size_of;

use super::NUM_ROUNDS;

#[repr(C)]
pub(crate) struct RoundFlagsCols<T> {
    pub step_flags: [T; NUM_ROUNDS],
}

pub(crate) const NUM_ROUND_FLAGS_COLS: usize = size_of::<RoundFlagsCols<u8>>();

impl<T> Borrow<RoundFlagsCols<T>> for [T] {
    fn borrow(&self) -> &RoundFlagsCols<T> {
        debug_assert_eq!(self.len(), NUM_ROUND_FLAGS_COLS);
        let (prefix, shorts, suffix) = unsafe { self.align_to::<RoundFlagsCols<T>>() };
        debug_assert!(prefix.is_empty(), "Alignment should match");
        debug_assert!(suffix.is_empty(), "Alignment should match");
        debug_assert_eq!(shorts.len(), 1);
        &shorts[0]
    }
}

impl<T> BorrowMut<RoundFlagsCols<T>> for [T] {
    fn borrow_mut(&mut self) -> &mut RoundFlagsCols<T> {
        debug_assert_eq!(self.len(), NUM_ROUND_FLAGS_COLS);
        let (prefix, shorts, suffix) = unsafe { self.align_to_mut::<RoundFlagsCols<T>>() };
        debug_assert!(prefix.is_empty(), "Alignment should match");
        debug_assert!(suffix.is_empty(), "Alignment should match");
        debug_assert_eq!(shorts.len(), 1);
        &mut shorts[0]
    }
}
