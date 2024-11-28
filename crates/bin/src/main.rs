//! The main entry point.

use command::{Cli, Parser, TuiError};

#[tokio::main]
async fn main() -> Result<(), TuiError> {
    let parsed = Cli::<String>::parse();
    parsed.execute().await
}
