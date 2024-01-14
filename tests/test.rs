use std::thread::sleep;

use tokio::task::JoinHandle;

pub fn run_testing_environment() -> JoinHandle<()> {
    let app = tracks::serve_application();
    let app_handle = tokio::spawn(app);
    // delay for 500ms to allow the server to start
    sleep(std::time::Duration::from_millis(500));

    // TO DO: Add database setup

    app_handle
}