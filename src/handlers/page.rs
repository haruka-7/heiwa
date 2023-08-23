use crate::cli::serve::AppState;
use crate::entities::page::Page;
use crate::utils::markdown::markdown_to_html;
use crate::utils::template::minify_html;
use axum::body::StreamBody;
use axum::extract::{Path, State};
use axum::http::header;
use axum::response::{Html, IntoResponse, Response};
use gray_matter::ParsedEntity;
use std::path::PathBuf;
use std::sync::Arc;
use tera::Context;
use tokio_util::io::ReaderStream;

pub async fn show(Path(path): Path<String>, State(state): State<Arc<AppState>>) -> Response {
    if path.contains("medias/") {
        let file_path: String = format!("./pages/{}", path);
        let pathbuf = PathBuf::from(&path);
        let filename = pathbuf.file_name().unwrap();
        let file = tokio::fs::File::open(&file_path).await.unwrap();
        let content_type = mime_guess::from_path(&file_path).first_raw().unwrap();
        let stream = ReaderStream::new(file);
        let body = StreamBody::new(stream);
        let headers = [
            (header::CONTENT_TYPE, content_type),
            (
                header::CONTENT_DISPOSITION,
                &format!("attachment; filename=\"{:?}\"", filename),
            ),
        ];
        (headers, body).into_response()
    } else {
        let mut html_output: String = String::new();
        let result: ParsedEntity =
            markdown_to_html(&mut html_output, path, state.mk_parser_options);

        let page: Page = Page::new(result);

        let mut context = Context::new();
        context.insert("meta_title", "Meta title");
        context.insert("meta_description", "Meta description");
        context.insert("site_title", state.config.title.as_str());
        context.insert("title", page.title.as_str());
        context.insert("content", html_output.as_str());
        let html = state.tera.render("page.html", &context).unwrap();
        Html(minify_html(html)).into_response()
    }
}
