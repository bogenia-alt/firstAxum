use log::info;

pub async fn hello_handler() -> &'static str {
    info!("[hello_handler] get query");
    "Hello from Axum!\n"
}
