pub struct AppState {
    pub scylla_session: (),
}

impl AppState {
    pub fn init() -> Self {
        Self {
            scylla_session: (),
        }
    }
}