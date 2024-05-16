use std::fmt;
use std::fmt::Debug;
use std::process::{ExitCode, Termination};

fn main() -> CliResult {
    CliResult(real_main())
}

fn real_main() -> anyhow::Result<()> {
    Err(CliError::retryable("This is a retryable error".to_string()))
}

impl Termination for CliResult {
    fn report(self) -> ExitCode {
        match self.0 {
            Ok(_) => ExitCode::SUCCESS,
            Err(x) => {
                if let Ok(cli_error) = x.downcast::<CliError>() {
                    cli_error.into()
                } else {
                    ExitCode::FAILURE
                }
            }
        }
    }
}

struct CliResult(anyhow::Result<()>);


#[derive(Debug)]
pub enum CliError {
    Processing(String),
    Retryable(String),
}

impl CliError {
    pub fn processing<T: Debug>(err: T) -> anyhow::Error {
        CliError::Processing(format!("{:?}", err)).into()
    }

    pub fn retryable<T: Debug>(err: T) -> anyhow::Error {
        CliError::Retryable(format!("{:?}", err)).into()
    }
}

impl std::error::Error for CliError {}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CliError::Processing(err) => write!(f, "Processing error: {}", err),
            CliError::Retryable(err) => write!(f, "Retryable error: {}", err),
        }
    }
}

impl From<CliError> for ExitCode {
    fn from(err: CliError) -> ExitCode {
        match err {
            CliError::Processing(_) => ExitCode::from(2),
            CliError::Retryable(_) => ExitCode::from(100),
        }
    }
}



