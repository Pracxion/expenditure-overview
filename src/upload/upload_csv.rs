use std::collections::HashMap;

use axum::{extract::Multipart, http::StatusCode, response::IntoResponse};
use serde_json::Value;
use tracing::info;

use crate::database::csv_schema::SchemaDefinition;

const MAX_PREVIEW_LENGTH: usize = 20;

pub async fn upload_csv(mut multipart: Multipart) -> impl IntoResponse {
    let schema_path = "database/csv_schema.json";
    let schema = match SchemaDefinition::load_from_file(schema_path) {
        Ok(schema) => schema,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to load schema: {}", e)),
    };

    info!("Successfully loaded schema from {}", schema_path);

    let mut processed_files = 0;
    let mut failed_files = 0;

    while let Some(field) = multipart.next_field().await.unwrap_or_else(|e| {
        info!("Error getting next field: {}", e);
        return None;
    }) {
        let form_name = field.name().unwrap_or("unnamed").to_string();
        let file_name = field.file_name().unwrap_or("unnamed").to_string();

        if file_name.ends_with(".csv") == false {
            failed_files += 1
        }

        let data = match field.bytes().await {
            Ok(data) => data,
            Err(e) => {
                info!("Error reading field bytes: {}", e);
                failed_files += 1;
                continue;
            }
        };

        info!("Processing file: {} ({} bytes) (name: {})", file_name, data.len(), form_name);

        match schema.process_csv(data.to_vec()) {
            Ok(processed_data) => {
                log_csv_lines(processed_data, file_name);

                processed_files += 1;
            }
            Err(e) => {
                info!("Failed to process {}: {}", file_name, e);
                failed_files += 1;
            }
        }
    }

    if processed_files <= 0 {
        return (StatusCode::BAD_REQUEST, "No CSV files were provided".to_string());
    }

    if failed_files > 0 {
        return (
            StatusCode::PARTIAL_CONTENT,
            format!("Processed {} files successfully. Failed to process {} files.", processed_files, failed_files),
        );
    }

    return (StatusCode::OK, format!("Successfully processed {} CSV files", processed_files));
}

fn log_csv_lines(processed_data: Vec<HashMap<String, Value>>, file_name: String) {
    let total_entries = processed_data.len();
    info!("Successfully processed {}. Total entries: {}", file_name, total_entries);

    for (i, entry) in processed_data.iter().take(MAX_PREVIEW_LENGTH).enumerate() {
        let entry_json = serde_json::to_string(entry).unwrap_or_default();
        info!("File: {} - Line {}: {}", file_name, i, entry_json);
    }

    if total_entries > MAX_PREVIEW_LENGTH {
        info!("... and {} more entries", total_entries - MAX_PREVIEW_LENGTH);
    }
}
