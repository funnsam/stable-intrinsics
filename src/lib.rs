#![cfg_attr(nightly, feature(core_intrinsics))]
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(internal_features)]

#![doc = include_str!("../README.md")]

mod disjoint_bitor;
mod test;

pub use disjoint_bitor::*;

/// Same as [`std::intrinsics::abort`] on nightly. On stable and beta, the exact behavior depends
/// on whether or not that `std` is avalible.
///
/// If `std` feature is enabled, use [`std::process::abort`].
/// If `std` feature is not enabled, use `panic!()`.
#[inline(always)]
pub fn abort() -> ! {
    #[cfg(nightly)]
    core::intrinsics::abort();

    #[cfg(not(nightly))] {
        // use system abort as fallback on std
        #[cfg(feature = "std")]
        std::process::abort();

        // panic on non-std
        #[cfg(not(feature = "std"))]
        panic!();
    }
}

/// Same as [`std::intrinsics::breakpoint`] on nightly. On stable and beta, the implementation
/// depends on the target architecture.
///
/// # Notes
/// On stable and beta, the exact behavior is not stable across crate versions and may change.
///
/// Currently it is implemented as followed:
///     - On x86 and x86-64, an `int3` instruction is emitted.
///     - On ARM, an `bkpt` instruction is emitted.
///     - On AArch64, an `brk #0xf000` instruction is emitted, like the nightly counterpart.
///     - On RISCV, an `ebreak` instruction is emitted.
///     - On MIPS, an `break` instruction is emitted.
///     - Otherwise, it is a no-op.
#[inline(always)]
pub fn breakpoint() {
    #[cfg(nightly)]
    core::intrinsics::breakpoint();

    #[cfg(not(nightly))] {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        unsafe { core::arch::asm!("int3", options(nomem, nostack)) };

        #[cfg(target_arch = "arm")]
        unsafe { core::arch::asm!("bkpt", options(nomem, nostack)) };

        #[cfg(target_arch = "aarch64")]
        unsafe { core::arch::asm!("brk #0xf000", options(nomem, nostack)) };

        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        unsafe { core::arch::asm!("ebreak", options(nomem, nostack)) };

        #[cfg(any(target_arch = "mips", target_arch = "mips64"))]
        unsafe { core::arch::asm!("break", options(nomem, nostack)) };
    }
}

/// Same as [`core::intrinsics::cold_path`] on nightly. On stable and beta, it is a no-op.
#[inline(always)]
pub const fn cold_path() {
    #[cfg(nightly)]
    core::intrinsics::cold_path();
}

/// Same as [`core::intrinsics::likely`] on nightly. On stable and beta, it is an identity
/// function.
#[inline(always)]
pub const fn likely(b: bool) -> bool {
    #[cfg(nightly)]
    return core::intrinsics::likely(b);

    #[cfg(not(nightly))]
    return b;
}

/// Same as [`core::intrinsics::unlikely`] on nightly. On stable and beta, it is an identity
/// function.
#[inline(always)]
pub const fn unlikely(b: bool) -> bool {
    #[cfg(nightly)]
    return core::intrinsics::unlikely(b);

    #[cfg(not(nightly))]
    return b;
}

/// Same as [`core::intrinsics::prefetch_read_data`] on nightly. On stable and beta, it is a no-op.
///
/// # Safety
/// Marked as unsafe like the standard library.
#[inline(always)]
pub unsafe fn prefetch_read_data<T>(_data: *const T, _locality: i32) {
    #[cfg(nightly)]
    unsafe {
        core::intrinsics::prefetch_read_data(_data, _locality);
    }
}

/// Same as [`core::intrinsics::prefetch_read_instruction`] on nightly. On stable and beta, it is a no-op.
///
/// # Safety
/// Marked as unsafe like the standard library.
#[inline(always)]
pub unsafe fn prefetch_read_instruction<T>(_data: *const T, _locality: i32) {
    #[cfg(nightly)]
    unsafe {
        core::intrinsics::prefetch_read_instruction(_data, _locality);
    }
}

/// Same as [`core::intrinsics::prefetch_write_data`] on nightly. On stable and beta, it is a no-op.
///
/// # Safety
/// Marked as unsafe like the standard library.
#[inline(always)]
pub unsafe fn prefetch_write_data<T>(_data: *const T, _locality: i32) {
    #[cfg(nightly)]
    unsafe {
        core::intrinsics::prefetch_write_data(_data, _locality);
    }
}

/// Same as [`core::intrinsics::prefetch_write_instruction`] on nightly. On stable and beta, it is a no-op.
///
/// # Safety
/// Marked as unsafe like the standard library.
#[inline(always)]
pub unsafe fn prefetch_write_instruction<T>(_data: *const T, _locality: i32) {
    #[cfg(nightly)]
    unsafe {
        core::intrinsics::prefetch_write_instruction(_data, _locality);
    }
}

/// Same as [`core::intrinsics::select_unpredictable`] on nightly. On stable and beta, use an
/// if-else to determine the output.
#[inline(always)]
pub fn select_unpredictable<T>(b: bool, true_val: T, false_val: T) -> T {
    #[cfg(nightly)]
    return core::intrinsics::select_unpredictable(b, true_val, false_val);

    #[cfg(not(nightly))]
    return if b { true_val } else { false_val };
}

/// Same as [`core::intrinsics::disjoint_bitor`] on nightly except the `const`-ness. On stable and beta, it is
/// implemented with [`core::hint::assert_unchecked`].
///
/// # Panics
/// This function panics if it is about to be unsafe on builds with debug assertions enabled.
///
/// # Safety
/// Requires that `(a & b) == 0`, or equivalently that `(a | b) == (a + b)`.
#[inline(always)]
pub unsafe fn disjoint_bitor<T: DisjointBitOr>(a: T, b: T) -> T {
    unsafe { a.disjoint_bitor(b) }
}

/// Same as [`core::intrinsics::transmute_unchecked`].
///
/// # Safety
/// Same as [`core::intrinsics::transmute_unchecked`].
#[inline(always)]
pub const unsafe fn transmute_unchecked<Src, Dst>(src: Src) -> Dst {
    #[cfg(nightly)]
    return unsafe { core::intrinsics::transmute_unchecked(src) };

    #[cfg(not(nightly))]
    unsafe {
        let dst = core::mem::transmute::<*const Src, *const Dst>(&src as *const Src);
        core::mem::forget(src);
        dst.read()
    }
}

/// Same as [`core::intrinsics::raw_eq`] on nightly. On stable and beta, `a` and `b` are converted
/// into `[u8]` before comparason.
///
/// # Safety
/// Same as [`core::intrinsics::raw_eq`].
#[inline(always)]
pub const unsafe fn raw_eq<T>(a: &T, b: &T) -> bool {
    #[cfg(nightly)]
    return unsafe { core::intrinsics::raw_eq(a, b) };

    #[cfg(not(nightly))]
    unsafe {
        let a = core::slice::from_raw_parts(a as *const T as *const u8, size_of::<T>());
        let b = core::slice::from_raw_parts(b as *const T as *const u8, size_of::<T>());

        // NOTE: using `while` instead of `for` or `==` because const-evaluation
        let mut i = 0;
        while i < size_of::<T>() {
            if a[i] != b[i] {
                return false;
            }

            i += 1;
        }

        true
    }
}

/// Same as [`core::intrinsics::nontemporal_store`] on nightly. On stable and beta, it is simply a
/// pointer store.
///
/// # Safety
/// [`core::ptr::write`] must be safe.
pub unsafe fn nontemporal_store<T>(ptr: *mut T, val: T) {
    unsafe {
        #[cfg(nightly)]
        core::intrinsics::nontemporal_store(ptr, val);

        #[cfg(not(nightly))]
        ptr.write(val);
    }
}
