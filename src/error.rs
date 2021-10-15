use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
/// Error type
pub enum SocarelErrorType {
    /// Iterator errors
    Iter,
    /// Tree errors
    Tree,
    /// Node errors
    Node,
    /// Forest errors
    Forest,
    /// Other type of error, not specified
    Other
}

impl fmt::Display for SocarelErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Forest => write!(f, "Forest"),
            Self::Tree => write!(f, "Tree"),
            Self::Node => write!(f, "Node"),
            Self::Iter => write!(f, "Iter"),
            Self::Other => write!(f, "Other")
        }
    }
}

#[derive(Debug)]
/// Socarel error.
pub struct SocarelError {
    err_msg: String,
    err_code: i32,
    err_type: SocarelErrorType
}

impl SocarelError {
    /// Create error.
    /// 
    /// # Arguments
    /// 
    /// * `err_msg` - Error message.
    /// * `err_code` - Error code.
    /// * `err_type` - Error type.
    /// 
    /// # Return
    /// 
    /// * An error model.
    ///
    pub fn new(err_msg: &str, err_code: i32, err_type: SocarelErrorType) -> Self {
        SocarelError {
            err_msg: String::from(err_msg),
            err_code,
            err_type
        }
    }

    /// Get error message.
    /// 
    /// # Return
    /// 
    /// * message.
    ///
    pub fn get_message(&self) -> &str {
        &self.err_msg
    }

    /// Get error code.
    /// 
    /// # Return
    /// 
    /// * code.
    ///
    pub fn get_code(&self) -> i32 {
        self.err_code
    }

    /// Get error type.
    /// 
    /// # Return
    /// 
    /// * type.
    ///
    pub fn get_type(&self) -> SocarelErrorType {
        self.err_type.clone()
    }
}

impl fmt::Display for SocarelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:`{}`({})", self.err_type, self.err_msg, self.err_code)
    }
}

impl Error for SocarelError {}