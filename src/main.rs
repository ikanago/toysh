use event::ShellState;
use shell::Shell;
use tracing_subscriber::{self, fmt, prelude::*, EnvFilter};

mod event;
mod parser;
mod process;
mod shell;

fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    ShellState::new(Shell).run();
}
