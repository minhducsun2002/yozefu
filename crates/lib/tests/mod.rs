use insta::{assert_debug_snapshot, glob};
use std::fs;
use yozefu_lib::{parse_search_query, ExportedKafkaRecord, KafkaRecord};

#[test]
fn test_inputs() {
    glob!("inputs/*.sql", |path| {
        unsafe {
            use std::env;
            // Set the timezone to Paris to have a fixed timezone for the tests
            env::set_var("TZ", "Europe/Paris");
        }

        let input = fs::read_to_string(path).unwrap();
        let input = input.trim();
        insta::with_settings!({
            description => input.replace("\n", " "),
            filters => vec![
            ("[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}\\.[0-9]{6,}\\+[0-9]{2}:[0-9]{2}", "[datetime]"),
        ]}, {
            assert_debug_snapshot!(parse_search_query(input));
        });
    });
}

#[test]
fn test_exported_record() {
    glob!("inputs/record*.json", |path| {
        let input = fs::read_to_string(path).unwrap();
        let record: KafkaRecord = serde_json::from_str(&input).unwrap();
        assert_debug_snapshot!(ExportedKafkaRecord::from(&record));
    });
}
