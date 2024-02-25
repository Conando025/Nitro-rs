pub trait Flags {
    fn check(self, other: Self) -> bool;
}

impl Flags for u8 {
    fn check(self, other: Self) -> bool {
        self & other != 0
    }
}

impl Flags for u16 {
    fn check(self, other: Self) -> bool {
        self & other != 0
    }
}
