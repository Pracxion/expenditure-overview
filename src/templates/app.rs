use askama::Template;
use axum::response::IntoResponse;

use super::base::HtmlTemplate;

pub async fn app() -> impl IntoResponse {
    let template = AppTemplate {};
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "src/app/app.html")]
struct AppTemplate;
