use std::io;
use std::io::{Error, ErrorKind};

/// Checks if the given unary flag is set within the given binary encoding of a
/// list of flags.
pub fn flag_is_set(flag_to_check: u16, flags: u16) -> bool {
    let check = flags & flag_to_check;

    check > 0
}

pub fn promote_result_to_io<A>(result: Result<A, String>) -> io::Result<A> {
    match result {
        Ok(v) => Ok(v),
        Err(s) => Err(Error::new(ErrorKind::Other, s)),
    }
}

pub fn io_err<S: Into<String>>(message: S) -> Error {
    Error::new(ErrorKind::Other, message.into())
}

/// A trait that is used to add a method to Result types to allow a context
/// description message to be prepended to an error.
pub trait Contextable {
    fn context<S: Into<String>>(self, error_description: S) -> Self;
}

impl<A> Contextable for Result<A, io::Error> {
    fn context<S: Into<String>>(self, error_description: S) -> io::Result<A> {
        self.map_err(|e| io_err(format!("{} {}", error_description.into(), e)))
    }
}

#[cfg(test)]
mod tests {
    use util::flag_is_set;

    #[test]
    fn flag_is_set_finds_a_set_flag() {
        let public_flag = 0b0000_0000_0000_0001;
        let access_flags = 0b0000_0000_0000_1001;

        assert_eq!(true, flag_is_set(public_flag, access_flags))
    }

    #[test]
    fn flag_is_set_does_not_find_a_missing_flag() {
        let volatile_flag = 0b0000_0000_0100_0000;
        let access_flags = 0b0000_0000_0000_1001;

        assert_eq!(false, flag_is_set(volatile_flag, access_flags))
    }
}
