use axum_sessions::SessionHandle;

pub fn get_writable_session() {
    handle_session().write()
}

pub fn get_readable_session() {
    handle_session().read()
}

fn handle_session() -> SessionHandle{
    request.extensions().get::<SessionHandle>().unwrap()
}