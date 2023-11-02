use crate::cli::serve::AppState;
use crate::entities::page::Page;
use crate::utils::file::read_file;
use axum::body::StreamBody;
use axum::extract::{Path, State};
use axum::http::header;
use axum::response::{Html, IntoResponse, Response};
use std::path::PathBuf;
use std::sync::Arc;
use tera::Context;
use tokio_util::io::ReaderStream;

pub async fn show(Path(file_path): Path<String>, State(state): State<Arc<AppState>>) -> Response {
    if file_path.contains("medias/") {
        let file_path: String = format!("{}/pages/{}", state.path, file_path);
        let pathbuf = PathBuf::from(&file_path);
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
        let content_file: String = read_file(&format!("{}/pages/{}.md", state.path, file_path));
        let page: Page = Page::new(file_path.clone(), content_file, state.mk_parser_options);

        let mut context = Context::new();
        context.insert("meta_title", &page.title);
        context.insert("meta_description", &page.description);
        context.insert("site_title", state.config.title.as_str());
        context.insert(
            "mastodon_verification_link",
            &state.config.mastodon_verification_link,
        );
        context.insert("page", &page);

        Html(state.tera.render("page.html", &context).unwrap()).into_response()
    }
}
