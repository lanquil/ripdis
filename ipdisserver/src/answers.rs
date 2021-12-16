use crate::bytes::safe_format_bytes;
use crate::inventory::{ExecuteInventory, InternalInventory, InventoryFile};
use bytes::Bytes;
use color_eyre::eyre::Report;
use serde_json;
use serde_json::value::Value;
use std::fmt;
use std::path::Path;
use tracing::{debug, error, instrument, trace, warn};

const FALLBACK_INFO_KEY: &str = "info";

pub type BeaconInfos = serde_json::map::Map<String, Value>;

/// Expecting one ore more lines formatted according to https://docs.mender.io/3.0/client-installation/inventory
pub trait FromCmdOutput {
    fn from_cmd_output(lines: &str) -> Result<BeaconInfos, Report>;
}

impl FromCmdOutput for BeaconInfos {
    fn from_cmd_output(lines: &str) -> Result<BeaconInfos, Report> {
        let separator = "=";
        let mut res = BeaconInfos::new();
        for line in lines.lines() {
            if let Some((key, value)) = line.to_string().split_once(separator) {
                match res.get_mut(&key.to_string()) {
                    None => {
                        res.insert(key.into(), Value::String(value.to_string()));
                    }
                    Some(previous_value) => {
                        match previous_value {
                            Value::String(previous_string) => {
                                *previous_value = Value::Array(vec![
                                    Value::String(previous_string.to_string()),
                                    Value::String(value.to_string()),
                                ]);
                            }
                            Value::Array(previous_array) => {
                                previous_array.push(Value::String(value.to_string()));
                                *previous_value = Value::Array(previous_array.to_vec());
                            }
                            _ => unreachable!(), // Only inserting String or Array!
                        };
                    }
                };
            }
        }
        Ok(res)
    }
}

/// Message returned to the scanner (JSON formatted).
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Answer(pub Bytes);

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.safe_format())
    }
}

impl From<&Bytes> for Answer {
    fn from(bytes: &Bytes) -> Self {
        Self(bytes.clone())
    }
}

impl From<&[u8]> for Answer {
    fn from(bytes: &[u8]) -> Self {
        Self(Bytes::copy_from_slice(bytes))
    }
}

impl From<String> for Answer {
    fn from(bytes: String) -> Self {
        Self(Bytes::from(bytes))
    }
}

impl Answer {
    fn safe_format(&self) -> String {
        let res = match serde_json::from_slice(&self.0) {
            Ok(p) => p,
            Err(e) => {
                warn!(?e, "Error deserializing Answer payload.");
                let mut info = BeaconInfos::new();
                info.insert(FALLBACK_INFO_KEY.into(), safe_format_bytes(&self.0).into());
                info
            }
        };
        match serde_json::to_string(&res) {
            Ok(f) => f,
            Err(e) => {
                error!(?e, ?res, ?self, "Error formatting Answer.");
                format!("{:?}", res)
            }
        }
    }
}

#[instrument(skip(inventory_files))]
pub fn get_answer<P>(inventory_files: &[P]) -> Result<Answer, Report>
where
    P: AsRef<Path>,
{
    get_answer_hostname_and_files(InternalInventory::default(), inventory_files)
}

fn get_answer_hostname_and_files<P>(
    hostname_inventory: InternalInventory,
    inventory_files: &[P],
) -> Result<Answer, Report>
where
    P: AsRef<Path>,
{
    let mut hostname_answer = get_internal_inventory_answer(hostname_inventory);
    debug!(?hostname_answer);
    let mut inventory_answer = get_inventory_files_answer(inventory_files);
    debug!(?inventory_answer);
    let answer = Answer::from(serde_json::to_string(&join_answers(
        &mut hostname_answer,
        &mut inventory_answer,
    ))?);
    Ok(answer)
}

fn join_answers(first: &mut BeaconInfos, second: &mut BeaconInfos) -> BeaconInfos {
    first.append(second);
    first.clone()
}

#[instrument(skip(inventory))]
fn get_internal_inventory_answer(inventory: InternalInventory) -> BeaconInfos {
    inventory.execute().output
}

#[instrument(skip(inventory_file_paths))]
fn get_inventory_files_answer<P>(inventory_file_paths: &[P]) -> BeaconInfos
where
    P: AsRef<Path>,
{
    let mut res = BeaconInfos::new();
    for inventory_path in inventory_file_paths {
        let inventory = InventoryFile::from(inventory_path.as_ref());
        trace!(?inventory, "Executing inventory file.");
        let mut inventory_result = inventory.execute();
        trace!(?inventory_result, ?inventory, "Inventory file executed.");
        res = join_answers(&mut res, &mut inventory_result.output);
        trace!(?res, "Updated inventory info.");
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;
    use std::os::unix::fs::PermissionsExt;
    use std::path::PathBuf;

    #[test]
    #[tracing_test::traced_test]
    fn test_safe_format_answer_rust() {
        assert_eq!(
            Answer::from(
                serde_json::to_vec(&serde_json::json!(HashMap::from([("a bool", true)])))
                    .unwrap()
                    .as_slice()
            )
            .safe_format(),
            r#"{"a bool":true}"#
        );
    }

    #[test]
    #[tracing_test::traced_test]
    fn test_safe_format_answer_json() {
        let json = r#"{"key string": [1, "two", 3.4, false, null], "2": "another string"}"#;
        let data: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(
            serde_json::from_str::<'_, serde_json::Value>(
                &Answer::from(json.as_bytes()).safe_format()
            )
            .unwrap(),
            data
        );
    }

    #[test]
    #[tracing_test::traced_test]
    fn test_safe_format_answer_not_json() {
        let expected = r#"{"info":"not json"}"#;
        assert_eq!(
            serde_json::from_str::<'_, serde_json::Value>(
                &Answer(Bytes::from("not json".as_bytes())).safe_format()
            )
            .unwrap()
            .to_string(),
            expected.to_string()
        );
    }

    fn write_inventory_file(filename: &str, content: &str) -> PathBuf {
        let datadir = std::env::temp_dir()
            .as_path()
            .join("rust-ipdisserver-test-answers-datadir/");
        // TODO: windows
        if let Err(error) = std::fs::create_dir(&datadir) {
            match error.kind() {
                std::io::ErrorKind::AlreadyExists => (),
                _ => panic!(),
            }
        };
        let path = datadir.join(filename);
        std::fs::write(&path, content).unwrap();
        let file = std::fs::File::open(&path).unwrap();
        let metadata = file.metadata().unwrap();
        let mut permissions = metadata.permissions();
        permissions.set_mode(0o755);
        file.set_permissions(permissions).unwrap();
        path.into()
    }

    #[test]
    #[tracing_test::traced_test]
    fn test_get_answer() {
        let echo_list_content = "#!/bin/sh\necho 'foo=bar\nfoo=baz'";
        let echo_multiple_lines_content =
            "#!/bin/sh\necho 'foo1=1'\necho 'foo2=2'\necho\necho 'foo3 = 3'\necho";
        let echo_nothing_conent = "#!/bin/sh\necho";
        let wrong_format_conent = "#!/bin/sh\necho 'foo wrong'";
        let return_error_conent = "#!/bin/sh\necho 'some error'\nexit 1";

        let echo_list_path = write_inventory_file("echo-list", echo_list_content);
        let echo_multiple_lines_path =
            write_inventory_file("echo-multiple_lines", echo_multiple_lines_content);
        let echo_nothing_path = write_inventory_file("echo-nothing", echo_nothing_conent);
        let wrong_format_path = write_inventory_file("wrong-format", wrong_format_conent);
        let return_error_path = write_inventory_file("return-error", return_error_conent);
        let empty_file_path = write_inventory_file("empty-file", "");
        let nonexisting_path = PathBuf::from("non-existing-file");

        let inventory_files = vec![
            echo_list_path,
            echo_multiple_lines_path,
            echo_nothing_path,
            wrong_format_path,
            return_error_path,
            empty_file_path,
            nonexisting_path,
        ];
        let expected = r#"{"foo":["bar","baz"],"foo1":"1","foo2":"2","foo3 ":" 3","hostname":"dummy-hostname"}"#;
        assert_eq!(
            std::str::from_utf8(
                &get_answer_hostname_and_files(
                    InternalInventory {
                        key: "hostname".to_string(),
                        source: Box::new(|| "dummy-hostname".to_string())
                    }, // mock hostname
                    inventory_files.as_slice()
                )
                .unwrap()
                .0
            )
            .unwrap(),
            expected
        );
    }

    #[test]
    #[tracing_test::traced_test]
    fn test_safe_format_answer_not_bytes() {
        let expected = r#"{"info":"INVALID UTF-8: [1F, 20, FF]"}"#;
        assert_eq!(
            serde_json::from_str::<'_, serde_json::Value>(
                &Answer::from(vec![31u8, 32, 255].as_slice()).safe_format()
            )
            .unwrap()
            .to_string(),
            expected.to_string()
        );
    }
}
