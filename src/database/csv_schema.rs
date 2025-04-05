use std::collections::HashMap;
use std::fs;
use std::io::Cursor;

use serde::{Deserialize,
            Serialize};
use serde_json::{Value,
                 json};

#[derive(Debug, Deserialize, Serialize)]
pub struct FieldDefinition {
    pub name:      String,
    pub data_type: String,
    pub required:  bool
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SchemaDefinition {
    pub fields: Vec<FieldDefinition>
}

impl SchemaDefinition {
    pub fn load_from_file(schema_path: &str) -> Result<Self, String> {
        let schema_content = fs::read_to_string(schema_path).map_err(|e| format!("Failed to read schema file: {}", e))?;

        serde_json::from_str(&schema_content).map_err(|e| format!("Failed to parse schema JSON: {}", e))
    }

    pub fn process_csv(&self, csv_data: Vec<u8>) -> Result<Vec<HashMap<String, Value>>, String> {
        let cursor = Cursor::new(csv_data);
        let mut csv_reader = csv::ReaderBuilder::new().flexible(true).has_headers(true).from_reader(cursor);

        let headers = match csv_reader.headers() {
            Ok(h) => h.clone(),
            Err(e) => return Err(format!("Failed to read CSV headers: {}", e))
        };

        let mut result = Vec::new();

        for record_result in csv_reader.records() {
            match record_result {
                Ok(record) => {
                    let mut row_data = HashMap::new();

                    for (i, field) in self.fields.iter().enumerate() {
                        if i >= headers.len() || i >= record.len() {
                            if field.required {
                                return Err(format!("Required field '{}' is missing", field.name));
                            }
                            continue;
                        }

                        let field_name = &field.name;
                        let value = record.get(i).unwrap_or_default();

                        // Convert the value based on the defined type
                        match field.data_type.as_str() {
                            "string" => row_data.insert(field_name.clone(), json!(value)),
                            "integer" => match value.parse::<i64>() {
                                Ok(v) => row_data.insert(field_name.clone(), json!(v)),
                                Err(_) => {
                                    if field.required {
                                        return Err(format!("Field '{}' could not be parsed as integer", field_name));
                                    }
                                    row_data.insert(field_name.clone(), json!(null))
                                }
                            },
                            "float" => match value.parse::<f64>() {
                                Ok(v) => row_data.insert(field_name.clone(), json!(v)),
                                Err(_) => {
                                    if field.required {
                                        return Err(format!("Field '{}' could not be parsed as float", field_name));
                                    }
                                    row_data.insert(field_name.clone(), json!(null))
                                }
                            },
                            "boolean" => match value.to_lowercase().as_str() {
                                "true" | "yes" | "1" => row_data.insert(field_name.clone(), json!(true)),
                                "false" | "no" | "0" => row_data.insert(field_name.clone(), json!(false)),
                                _ => {
                                    if field.required {
                                        return Err(format!("Field '{}' could not be parsed as boolean", field_name));
                                    }
                                    row_data.insert(field_name.clone(), json!(null))
                                }
                            },
                            _ => row_data.insert(field_name.clone(), json!(value))
                        };
                    }

                    result.push(row_data);
                }
                Err(e) => return Err(format!("Error parsing CSV record: {}", e))
            }
        }

        Ok(result)
    }
}
