use std::fmt::format;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 구성을 읽을 수 없으면 패닉에 빠진다.
    let configuration = get_configuration().expect("Failed to read configuration.");
    // 하드 코딩했던 `8000` 을 제거했다. 해당 값은 세팅에서 얻는다.
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}


