use core::{fmt, panic};

pub trait ResultExt<T, E: fmt::Debug> {
    fn expect_me(self, msg: &str) -> T;
}

pub trait OptionExt<T> {
    fn expect_me(self, msg: &str) -> T;
}

impl<T> OptionExt<T> for Option<T> {
    #[inline]
    #[track_caller]
    fn expect_me(self, msg: &str) -> T {
        match self {
            Some(t) => t,
            None => {
                let _ = msgbox::create("[CHRONA]'panic", msg, msgbox::IconType::Error);
                panic!("{}", msg);
            }
        }
    }
}

impl<T, E: fmt::Debug> ResultExt<T, E> for Result<T, E> {
    #[inline]
    #[track_caller]
    fn expect_me(self, msg: &str) -> T {
        match self {
            Ok(t) => t,
            Err(e) => {
                let msgcont = format!("{}: {:?}", msg, &e);
                let _ = msgbox::create("[CHRONA]'panic", &msgcont, msgbox::IconType::Error);
                panic!("{}", msgcont);
            }
        }
    }
}
