use crate::database::csv_schema::SchemaDefinition;
use axum::{extract::Multipart, http::StatusCode, response::IntoResponse};
use tracing::info;

const MAX_PREVIEW_LENGTH: usize = 20;

pub async fn upload_csv(mut multipart: Multipart) -> impl IntoResponse {
    // Load schema from the server
    let schema_path = "database/csv_schema.json"; // Updated path to the schema file
    let schema = match SchemaDefinition::load_from_file(schema_path) {
        Ok(schema) => schema,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to load schema: {}", e)),
    };

    info!("Successfully loaded schema from {}", schema_path);

    // Process the uploaded CSV file
    let mut csv_data: Option<Vec<u8>> = None;

    while let Some(field) = multipart.next_field().await.unwrap_or_else(|e| {
        info!("Error getting next field: {}", e);
        return None;
    }) {
        let _name = field.name().unwrap_or("unnamed").to_string();
        let file_name = field.file_name().unwrap_or("unnamed").to_string();

        let data = match field.bytes().await {
            Ok(data) => data,
            Err(e) => {
                info!("Error reading field bytes: {}", e);
                return (StatusCode::BAD_REQUEST, format!("Failed to read uploaded file: {}", e));
            }
        };

        info!("Received file: {} ({} bytes)", file_name, data.len());

        // Only care about the CSV file now
        if file_name.ends_with(".csv") {
            csv_data = Some(data.to_vec());
        }
    }

    // Process the CSV using the schema
    match csv_data {
        Some(data) => {
            match schema.process_csv(data) {
                Ok(processed_data) => {
                    // Log a preview of the data
                    let total_entries = processed_data.len();
                    info!("Successfully processed CSV with schema. Total entries: {}", total_entries);

                    // Only log up to MAX_PREVIEW_LENGTH entries
                    for (i, entry) in processed_data.iter().take(MAX_PREVIEW_LENGTH).enumerate() {
                        let entry_json = serde_json::to_string(entry).unwrap_or_default();
                        info!("Line {}: {}", i, entry_json);
                    }

                    if total_entries > MAX_PREVIEW_LENGTH {
                        info!("... and {} more entries", total_entries - MAX_PREVIEW_LENGTH);
                    }

                    return (StatusCode::OK, "Successfully processed CSV data".to_string());
                }
                Err(e) => return (StatusCode::BAD_REQUEST, format!("Failed to process CSV: {}", e)),
            }
        }
        None => return (StatusCode::BAD_REQUEST, "No CSV data provided".to_string()),
    }
}
