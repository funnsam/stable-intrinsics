#[test]
fn cold_path() {
    crate::cold_path();
}

#[test]
fn select_unpredictable() {
    assert_eq!(crate::select_unpredictable(true, 1, 2), 1);
    assert_eq!(crate::select_unpredictable(false, 1, 2), 2);
}

#[test]
fn transmute_unchecked() {
    #[repr(u8)]
    enum A {
        _0 = 0,
        _1 = 1,
        _2 = 2,
    }

    #[repr(u8)]
    #[derive(Debug, PartialEq)]
    enum B {
        _0 = 0,
        _1 = 1,
        _2 = 2,
    }

    impl Drop for A {
        fn drop(&mut self) {
            panic!();
        }
    }

    unsafe {
        assert_eq!(core::mem::transmute::<A, B>(A::_0), B::_0);
        assert_eq!(core::mem::transmute::<A, B>(A::_1), B::_1);
        assert_eq!(core::mem::transmute::<A, B>(A::_2), B::_2);

        assert_eq!(crate::transmute_unchecked::<A, B>(A::_0), B::_0);
        assert_eq!(crate::transmute_unchecked::<A, B>(A::_1), B::_1);
        assert_eq!(crate::transmute_unchecked::<A, B>(A::_2), B::_2);
    }
}

#[test]
fn raw_eq() {
    struct A([u8; 2]);

    let a = A([1, 2]);
    let b = A([1, 2]);
    let c = A([1, 3]);

    core::hint::black_box(&a.0);
    core::hint::black_box(&b.0);
    core::hint::black_box(&c.0);

    unsafe {
        assert!(crate::raw_eq(&a, &b));
        assert!(!crate::raw_eq(&a, &c));
    }
}
