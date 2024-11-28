use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{LazyLock, Mutex},
};

use extism::{convert::Json, Plugin};
use itertools::Itertools;
use lib::{
    search::filter::{Filter, FilterInput},
    FilterResult,
};

use super::{Search, SearchContext};

// This is evil, TODO context
pub static CACHED_FILTERS: LazyLock<Mutex<HashMap<String, Plugin>>> =
    LazyLock::new(|| HashMap::new().into());

// This is evil, TODO context
pub static FILTERS_DIR: LazyLock<Mutex<PathBuf>> = LazyLock::new(|| PathBuf::new().into());

pub const MATCHES_FUNCTION_NAME: &str = "matches";
pub const PARSE_PARAMETERS_FUNCTION_NAME: &str = "parse_parameters";

impl Search for Filter {
    fn matches(&self, context: &SearchContext) -> bool {
        let mut filters = CACHED_FILTERS.lock().unwrap();
        let plugin = &mut filters.get_mut(&self.name).unwrap();
        let input = FilterInput {
            record: context.record.clone(),
            params: self.parameters.iter().map(|e| e.json()).collect_vec(),
        };

        match plugin
            .call::<String, Json<FilterResult>>(
                MATCHES_FUNCTION_NAME,
                serde_json::to_string(&input).unwrap(),
            )
            .map(|e| e.0)
        {
            Ok(res) => res.r#match,
            Err(_e) => true,
        }
    }

    fn filters(&self) -> Vec<Filter> {
        vec![]
    }
}
