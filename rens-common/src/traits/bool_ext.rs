mod sealed {
    pub trait Sealed {}
    impl Sealed for bool {}
}

pub trait BoolExt: sealed::Sealed + Sized + Copy {
    fn tap_if_true(self, action: impl Fn()) -> bool;
    fn tap_if_false(self, action: impl Fn()) -> bool;
}

impl BoolExt for bool {
    #[inline]
    fn tap_if_true(self, action: impl Fn()) -> bool {
        if self {
            action();
        }
        self
    }

    #[inline]
    fn tap_if_false(self, action: impl Fn()) -> bool {
        if !self {
            action();
        }
        self
    }
}
