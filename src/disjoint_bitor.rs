/// Same as [`core::intrinsics::fallback::DisjointBitOr`] except it is not `const` due to stable
/// and beta limitations.
pub trait DisjointBitOr: Copy + 'static {
    /// Same as [`core::intrinsics::disjoint_bitor`] on nightly. On stable and beta, it is
    /// implemented with [`core::hint::assert_unchecked`].
    ///
    /// # Panics
    /// This function panics if it is about to be unsafe on builds with debug assertions enabled.
    ///
    /// # Safety
    /// Requires that `(a & b) == 0`, or equivalently that `(a | b) == (a + b)`.
    unsafe fn disjoint_bitor(self, other: Self) -> Self;
}

macro_rules! imp {
    ($ty:ty, $zr:tt) => {
        impl DisjointBitOr for $ty {
            unsafe fn disjoint_bitor(self, other: Self) -> Self {
                #[cfg(nightly)]
                return unsafe { core::intrinsics::disjoint_bitor(self, other) };

                #[cfg(not(nightly))]
                unsafe {
                    core::hint::assert_unchecked(self & other != $zr);
                    self | other
                }
            }
        }
    };
}

imp!(bool, false);
imp!(i8, 0);
imp!(u8, 0);
imp!(i16, 0);
imp!(u16, 0);
imp!(i32, 0);
imp!(u32, 0);
imp!(i64, 0);
imp!(u64, 0);
imp!(i128, 0);
imp!(u128, 0);
