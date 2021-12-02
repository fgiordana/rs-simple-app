use anyhow::Result;
use std::env;
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

const STACK_VAR: &str = "STACK";

#[derive(Error, Debug)]
pub enum ParseStackError {
    #[error("Unrecognized stack: {0}")]
    UnrecognizedStack(String),
}

#[derive(Clone, Debug)]
pub enum Stack {
    Prod,
    Test,
}

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stack_str = format!("{:?}", self).to_lowercase();
        write!(f, "{}", stack_str)
    }
}

impl FromStr for Stack {
    type Err = ParseStackError;

    fn from_str(s: &str) -> Result<Stack, ParseStackError> {
        match s {
            "prod" => Ok(Stack::Prod),
            "test" => Ok(Stack::Test),
            _ => Err(ParseStackError::UnrecognizedStack(s.to_string())),
        }
    }
}

impl Stack {
    pub fn from_env() -> Result<Self> {
        env::var(STACK_VAR)?.parse::<Stack>().map_err(|e| {
            anyhow::Error::new(e)
                .context(format!("Error parsing stack from env var '{}'", STACK_VAR))
        })
    }
}
