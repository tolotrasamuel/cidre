use crate::mac_types::FourCharCode;

pub type Err = i16;

/// https://www.osstatus.com
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Status(pub i32);

impl std::fmt::Debug for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let help = format!("https://www.osstatus.com?search={}", self.0);
        f.debug_struct("os::Status")
            .field("raw", &self.0)
            .field("help", &help)
            .finish()
    }
}

pub type Type = FourCharCode;

pub mod lock;

impl PartialEq<i32> for Status {
    fn eq(&self, other: &i32) -> bool {
        self.0.eq(other)
    }
}

impl Status {
    pub const NO_ERR: Self = Self(0);

    #[inline]
    pub fn is_ok(&self) -> bool {
        *self == Self::NO_ERR
    }

    #[inline]
    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }

    #[inline]
    pub unsafe fn to_result_unchecked<T>(self, option: Option<T>) -> Result<T, Self> {
        if self.is_ok() {
            Ok(option.unwrap_unchecked())
        } else {
            Err(self)
        }
    }

    #[inline]
    pub fn to_result_option<T>(self, option: Option<T>) -> Result<Option<T>, Self> {
        if self.is_ok() {
            Ok(option)
        } else {
            Err(self)
        }
    }

    #[inline]
    pub fn result(self) -> Result<(), Self> {
        if self.is_ok() {
            Ok(())
        } else {
            Err(self)
        }
    }
}

impl From<Status> for Result<(), Status> {
    #[inline]
    fn from(v: Status) -> Self {
        v.result()
    }
}
