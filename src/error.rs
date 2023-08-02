use std::fmt;

#[derive(Debug, Clone)]
pub struct BadExtensionError {
    pub message: String,
}

impl fmt::Display for BadExtensionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Extension must end with \".log\"")
    }
}

impl std::error::Error for BadExtensionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
    fn description(&self) -> &str {
        &self.message
    }
    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }
}


#[derive(Debug, Clone)]
pub struct FileNameError {
    pub message: String,
}

impl fmt::Display for FileNameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Extension must end with \".log\"")
    }
}

impl std::error::Error for FileNameError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
    fn description(&self) -> &str {
        &self.message
    }
    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }
}
