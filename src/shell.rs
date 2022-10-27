use tracing::debug;

use crate::{process::ExitStatus, parser};

pub struct Shell;

impl Shell {
    pub fn run_script(&mut self, script: &str) -> ExitStatus {
        match parser::parse(script) {
            Ok(ast) => {
                debug!(?ast);
                ExitStatus::ExitedWith(0)
            }
            Err(parser::ParseError::Empty) => {
                ExitStatus::ExitedWith(0)
            }
            Err(parser::ParseError::Fatal(err)) => {
                debug!("Parse error: {}", err);
                ExitStatus::ExitedWith(-1)
            }
        }
    }
}