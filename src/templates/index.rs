use askama::Template;
use axum::response::IntoResponse;

use super::base::HtmlTemplate;

pub async fn index() -> impl IntoResponse {
    let template = IndexTemplate {};
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "src/index.html")]
struct IndexTemplate;
