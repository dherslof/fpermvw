use std::error::Error;
use std::fmt;
use std::fs;

//------------------------------------------------------------------------------
// Utility Error
//------------------------------------------------------------------------------

#[derive(Debug)]
pub enum UtilsError {
    InvalidInput(String),
}

impl fmt::Display for UtilsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Utility error occurred!")
    }
}

impl Error for UtilsError {}
//fn source(&self) -> Option<&(dyn Error + 'static)> {
//     Some(&self.side)
//}
//}

//------------------------------------------------------------------------------
// Utility functions
//------------------------------------------------------------------------------

pub fn file_exists(name: &String) -> Result<bool, UtilsError> {
    if name.is_empty() {
        //Not sure this can happen due to required cli argument
        return Err(UtilsError::InvalidInput("File name is empty".to_string()));
    }

    let f = fs::metadata(name.as_str());

    if f.is_err() {
        let err_msg: String = "File with name: ".to_string() + name + " does not exists";
        return Err(UtilsError::InvalidInput(err_msg));
    }

    return Ok(true);
}

//------------------------------------------------------------------------------

pub fn directory(name: &String) -> Result<bool, failure::Error> {
    let exists = file_exists(name)?;

    if !exists {
        return Ok(false);
    }

    return Ok(fs::metadata(name.as_str()).unwrap().is_dir());
}

//------------------------------------------------------------------------------

pub fn symlink(name: &String) -> Result<bool, failure::Error> {
    let exists = file_exists(name)?;

    if !exists {
        return Ok(false);
    }

    match fs::read_link(name) {
        Ok(_) => return Ok(true),
        Err(_) => return Ok(false),
    }
}

//------------------------------------------------------------------------------
