#![cfg_attr(nightly, feature(core_intrinsics))]
#![allow(internal_features)]

/// Same as [`std::intrinsics::abort`] on nightly. On stable and beta, it finds a way to crash the
/// program. Currently the order is as followed:
///     1. Try to read a null pointer.
///     2. If `std` feature is enabled, use [`std::process::abort`].
///     3. If `std` feature is not enabled, use `panic!()`.
#[inline(always)]
pub fn abort() -> ! {
    #[cfg(nightly)]
    core::intrinsics::abort();

    #[cfg(not(nightly))] {
        // try access null pointer
        unsafe { core::ptr::read_volatile::<usize>(core::ptr::null()) };

        // use system abort as fallback on std
        #[cfg(feature = "std")]
        std::process::abort();

        // panic on non-std
        #[cfg(not(feature = "std"))]
        panic!();
    }
}

/// Same as [`std::intrinsics::breakpoint`] on nightly. On stable and beta, the implementation
/// depends on the target architecture. Note that the exact behavior is not stable across crate
/// versions and may change.
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
