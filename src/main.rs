use std::fmt::format;
use std::net::TcpListener;
use env_logger::Env;
use sqlx::{Connection, PgPool};
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // `init` 은 `set_logger` 를 호출한다. 다른 작업은 필요하지 않다.
    // RUST_LOG 환경 변수가 설정되어 있지 않으면
    // info 및 그 이상의 레벨의 모든 로그를 출력한다.
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // 구성을 읽을 수 없으면 패닉에 빠진다.
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(
        &configuration.database.connection_string()
    )
        .await
        .expect("Failed to connect to Postgres");

    // 하드 코딩했던 `8000` 을 제거했다. 해당 값은 세팅에서 얻는다.
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}


