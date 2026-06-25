pub trait OptionExt<T> {
    fn ok(self) -> Result<T, NoneError>;
}

impl<T> OptionExt<T> for Option<T> {
    #[track_caller]
    fn ok(self) -> Result<T, NoneError> {
        self.ok_or_else(NoneError::default)
    }
}

#[derive(Debug)]
pub struct NoneError(&'static std::panic::Location<'static>);
impl Default for NoneError {
    #[track_caller]
    fn default() -> Self {
        NoneError(std::panic::Location::caller())
    }
}
impl std::error::Error for NoneError {}
impl std::fmt::Display for NoneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "None is not `.ok`: {}", self.0)
    }
}
impl From<NoneError> for std::io::Error {
    fn from(none_error: NoneError) -> std::io::Error {
        std::io::Error::other(none_error)
    }
}
