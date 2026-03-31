pub enum SysError {
    NotFound,
    PermissionDenied,
    NameTooLong,
    Unknown,
}

impl SysError {
    pub fn from_isize(code: isize) -> Self {
        match -code {
            2 => SysError::NotFound,
            13 => SysError::PermissionDenied,
            36 => SysError::NameTooLong,
            _ => SysError::Unknown,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            SysError::NotFound => "No such file or directory",
            SysError::PermissionDenied => "Permission denied",
            SysError::NameTooLong => "Name too long",
            SysError::Unknown => "Unknown system error",
        }
    }
}
