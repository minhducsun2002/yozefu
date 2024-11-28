//! `MainCommandWithClient` is a wrapper around `MainCommand`.
//! When using `MainCommandWithClient`, you can pass a custom `ClientConfig` instead of using the one generated automatically from the config file.
//! I need it because I want to be able to pass a custom `ClientConfig` with specific kafka consumer properties that are not stored in the config file.

use std::fmt::Display;
use std::str::FromStr;

use rdkafka::ClientConfig;
use tui::error::TuiError;

use crate::log::init_logging_stderr;

use super::main_command::MainCommand;

pub struct MainCommandWithClient<T>
where
    T: Display + Clone + Sync + Send + 'static + FromStr,
    <T as FromStr>::Err: Display,
{
    command: MainCommand<T>,
    client_config: ClientConfig,
}

/// T must implement the trait FromStr.
impl<T> MainCommandWithClient<T>
where
    T: Display + Clone + Sync + Send + 'static + FromStr,
    <T as FromStr>::Err: Display,
{
    pub fn new(command: MainCommand<T>, client_config: ClientConfig) -> Self {
        Self {
            command,
            client_config,
        }
    }

    pub async fn execute(self) -> Result<(), TuiError> {
        match self.command.headless {
            true => {
                let _ = init_logging_stderr(self.command.debug);
                self.command
                    .headless(self.client_config)
                    .await
                    .map_err(|e| e.into())
            }
            false => {
                // Ignore the result, we just want to make sure the logger is initialized
                self.command.tui(self.client_config).await
            }
        }
    }
}
