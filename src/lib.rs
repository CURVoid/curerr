use std::io::ErrorKind;

/// struct created for error handling
/// 
/// # Examples
/// ```
/// use curerr::*;
/// 
/// fn devide(a: i32, b: i32) -> Result<i32, CursedErrorHandle> {
///     if b == 0 {
///         return Err(CursedErrorHandle::new(
///             CursedError::Argument(CursedErrorType::Invalid),
///             "0 division!!!".to_string()
///         ))
///     }
/// 
///     Ok(a/b)
/// }
/// 
/// let result = devide(6, 3).expect("division error");
/// 
/// assert_eq!(result, 2)
/// ```
pub struct CursedErrorHandle {
    error: CursedError,
    reason: String,
}

impl CursedErrorHandle {
    pub fn new(error: CursedError, reason: String) -> Self {
        Self { error, reason }
    }
    pub fn get_error(&self) -> &CursedError {
        &self.error
    }
    pub fn get_reason(&self) -> &String {
        &self.reason
    }
}

impl std::fmt::Display for CursedErrorHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} error: \"{}\"", self.error.to_string(), self.reason)
    }
}
impl std::fmt::Debug for CursedErrorHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(&self.error.to_string())
            .field(&self.reason)
            .finish()
        }
}
impl std::error::Error for CursedErrorHandle {}

/// enum with kinds of errors
/// # Examples
/// ```
/// use curerr::*;
/// 
/// let error = CursedErrorHandle::new(
///     CursedError::Path(CursedErrorType::Invalid),
///     "path is invalid".to_string()
/// );
/// 
/// assert_eq!(format!("{}", error), "path invalid error: \"path is invalid\"".to_string());
/// ```
pub enum CursedError {
    Connection(CursedErrorType),
    Address(CursedErrorType),
    Memory(CursedErrorType),
    Buffer(CursedErrorType),
    Envvar(CursedErrorType),
    Other(CursedErrorType),
    Input(CursedErrorType),
    File(CursedErrorType),
    Path(CursedErrorType),
    Data(CursedErrorType),
    Call(CursedErrorType),
    NoError,
    Unknown
}

impl ToString for CursedError {
    fn to_string(&self) -> String {
        match self {
            CursedError::Connection(err) => format!("connection {}", err.to_str()),
            CursedError::Address(err) => format!("address {}", err.to_str()),
            CursedError::Buffer(err) => format!("buffer {}", err.to_str()),
            CursedError::Envvar(err) => format!("envvar {}", err.to_str()),
            CursedError::Memory(err) => format!("memory {}", err.to_str()),
            CursedError::Input(err) => format!("input {}", err.to_str()),
            CursedError::File(err) => format!("file {}", err.to_str()),
            CursedError::Path(err) => format!("path {}", err.to_str()),
            CursedError::Call(err) => format!("call {}", err.to_str()), 
            CursedError::Data(err) => format!("data {}", err.to_str()),
            CursedError::Other(err) => err.to_str().to_string(),
            CursedError::NoError => "no error".to_string(),
            CursedError::Unknown => "unknown".to_string(),
        }        
    }
}

/// enum with error types
pub enum CursedErrorType {
    NotImplemented,
    AlreadyExists,
    AccessDenied,
    NotSupported,
    Interrupted,
    NotEnough,
    Timedout,
    Overflow,
    NotFound,
    Refused,
    Invalid,
    Aborted,
    Reset,
    Parse,
}

impl CursedErrorType {
    pub fn to_str(&self) -> &'static str {
        match self {
            CursedErrorType::NotImplemented => "not implemented",
            CursedErrorType::AlreadyExists => "already exists",
            CursedErrorType::AccessDenied => "access denied",
            CursedErrorType::NotSupported => "not supported", 
            CursedErrorType::Interrupted => "interrupted",
            CursedErrorType::NotEnough => "not enough",
            CursedErrorType::Timedout => "timed out",
            CursedErrorType::NotFound => "not found",
            CursedErrorType::Overflow => "overflow",
            CursedErrorType::Refused => "refused",
            CursedErrorType::Invalid => "invalid",
            CursedErrorType::Aborted => "aborted",
            CursedErrorType::Reset => "reset",
            CursedErrorType::Parse => "parse",
        }
    }
}

impl From<ErrorKind> for CursedError {
    fn from(error: ErrorKind) -> Self {
        match error {
            ErrorKind::NotFound => Self::Other(CursedErrorType::NotFound),
            ErrorKind::PermissionDenied => Self::Other(CursedErrorType::AccessDenied),
            ErrorKind::ConnectionRefused => Self::Connection(CursedErrorType::Refused),
            ErrorKind::ConnectionReset => Self::Connection(CursedErrorType::Reset),
            ErrorKind::ConnectionAborted => Self::Connection(CursedErrorType::Aborted),
            ErrorKind::NotConnected => Self::Connection(CursedErrorType::NotImplemented),
            ErrorKind::AddrInUse => Self::Address(CursedErrorType::AlreadyExists),
            ErrorKind::AddrNotAvailable => Self::Address(CursedErrorType::NotSupported),
            ErrorKind::AlreadyExists => Self::Other(CursedErrorType::AlreadyExists),
            ErrorKind::InvalidInput => Self::Input(CursedErrorType::Invalid),
            ErrorKind::InvalidData => Self::Data(CursedErrorType::Invalid),
            ErrorKind::TimedOut => Self::Call(CursedErrorType::Timedout),
            ErrorKind::Interrupted => Self::Other(CursedErrorType::Interrupted),
            ErrorKind::Unsupported => Self::Other(CursedErrorType::NotSupported),
            ErrorKind::OutOfMemory => Self::Memory(CursedErrorType::NotEnough),
            _ => Self::Unknown,
        }
    }
}