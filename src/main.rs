use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::process::{ExitCode, Termination};

fn main() -> CliResult<()> {
    println!("Hello, world!");
    CliError::Retryable("Error processing".to_string())
}

pub enum CliResult<T> {
    Ok(T),
    Err(CliError),
}

impl <T> Termination for CliResult<T> {
    fn report(self) -> ExitCode {
        match self {
            CliResult::Ok(_) => ExitCode::SUCCESS,
            CliResult::Err(err) => {
                eprintln!("Error: {:?}", err);
                ExitCode::from(<u8 as From<CliError>>::from(err))
            },
        }
    }
}

impl<T> TryFrom<CliResult<T>> for Result<T, CliResult<T>> {
    type Ok = T;
    type Error = CliError;

    fn from_error(err: Self::Error) -> Self {
        Exit::Err(err)
    }

    fn try_from(value: CliResult<T>) -> Result<Self, Self::Error> {
        todo!()
    }
}


#[derive(Debug)]
pub enum CliError<T: Error> {
    Processing(String),
    Retryable(String),
    Anyhow(anyhow::Error),
    Error(T)
}

impl std::error::Error for CliError {}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CliError::Processing(err) => write!(f, "Processing error: {}", err),
            CliError::Retryable(err) => write!(f, "Retryable error: {}", err),
            CliError::Anyhow(err) => fmt::Display::fmt(&err, f),
        }
    }
}

impl From<CliError> for u8 {
    fn from(err: CliError) -> u8 {
        match err {
            CliError::Processing(_) => 2,
            CliError::Retryable(_) => 100,
            CliError::Anyhow(_) => 1,
        }
    }
}

impl From<anyhow::Error> for CliError {
    fn from(err: anyhow::Error) -> Self {
        CliError::Anyhow(err)
    }
}

// impl <T> From<CliError> for Result<T, CliResult<T>> {
//     fn from(err: CliError) -> Self {
//         Err(CliResult::Err(err))
//     }
// }

impl <T> From<CliResult<T>> for anyhow::Result<T> {
    fn from(res: CliResult<T>) -> Self {
        match res {
            CliResult::Ok(val) => Ok(val),
            CliResult::Err(err) => Err(anyhow::Error::from(err)),
        }
    }
}

