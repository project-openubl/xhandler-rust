//! Prints the JSON Schema (draft-07) of the models the bindings exchange with
//! `xh_invoke`. The bindings generate their types from it.

use schemars::generate::SchemaSettings;
use schemars::JsonSchema;
use xbuilder::prelude::*;

/// Root of the schema: references every type the bindings need.
#[allow(dead_code)]
#[derive(JsonSchema)]
#[schemars(title = "Xhandler")]
struct Root {
    document: Document,
    defaults: Defaults,
}

fn main() {
    let schema = SchemaSettings::draft07()
        .into_generator()
        .into_root_schema_for::<Root>();
    let mut schema = serde_json::to_value(schema).expect("schema json");
    mark_decimals(&mut schema);
    println!(
        "{}",
        serde_json::to_string_pretty(&schema).expect("schema json")
    );
}

/// Decimals travel as JSON strings. schemars describes them as a string or
/// number with a pattern; this turns them into `{"type": "string", "format":
/// "decimal"}` (nullable when optional) so generators can map them to the
/// decimal type of each language.
fn mark_decimals(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Object(map) => {
            let is_decimal = map
                .get("type")
                .and_then(|t| t.as_array())
                .is_some_and(|types| {
                    types.iter().any(|t| t == "number") && types.iter().any(|t| t == "string")
                });
            if is_decimal {
                let nullable = map["type"]
                    .as_array()
                    .is_some_and(|t| t.iter().any(|t| t == "null"));
                map.remove("pattern");
                map.insert("format".into(), "decimal".into());
                map.insert(
                    "type".into(),
                    if nullable {
                        serde_json::json!(["string", "null"])
                    } else {
                        "string".into()
                    },
                );
            }
            map.values_mut().for_each(mark_decimals);
        }
        serde_json::Value::Array(items) => items.iter_mut().for_each(mark_decimals),
        _ => {}
    }
}
