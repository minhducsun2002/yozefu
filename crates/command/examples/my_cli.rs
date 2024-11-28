use clap::Parser;
use rdkafka::ClientConfig;
use strum::{Display, EnumIter, EnumString};
use tui::TuiError;
use yozefu_command::Cli;

#[derive(Debug, Clone, PartialEq, Eq, Display, EnumString, EnumIter)]
#[strum(serialize_all = "lowercase")]
enum Cluster {
    Localhost,
    Test,
    Development,
    Production,
}

#[derive(Parser)]
struct MyCli {
    #[clap(flatten)]
    command: Cli<Cluster>,
}

impl MyCli {
    pub fn kafka_client_config(&self) -> ClientConfig {
        let mut config = ClientConfig::new();
        config.set_log_level(rdkafka::config::RDKafkaLogLevel::Emerg);
        match self.command.default_command.cluster() {
            Cluster::Localhost => {
                config.set("bootstrap.servers", "kafka-localhost.acme:9092".to_string())
            }
            Cluster::Test => config.set("bootstrap.servers", "kafka-test.acme:9092".to_string()),
            Cluster::Development => config.set(
                "bootstrap.servers",
                "kafka-development.acme:9092".to_string(),
            ),
            Cluster::Production => config.set(
                "bootstrap.servers",
                "kafka-production.acme:9092".to_string(),
            ),
        };

        config
    }

    pub async fn execute(&self) -> Result<(), TuiError> {
        self.command.execute_with(self.kafka_client_config()).await
    }
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let parsed = MyCli::parse();
    parsed.execute().await.map_err(|e| e.to_string())
}
