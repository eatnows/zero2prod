use std::net::TcpListener;
use sqlx::{Connection, PgPool};
use tracing_bunyan_formatter::{BunyanFormattingLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter};
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 모든 `log`의 이벤트를 구독자에게 리다이렉트한다.
    LogTracer::init().expect("Failed to set logger");

    // RUST_LOG 환경 변수가 설정되어 있지 않으면
    // info 레벨 및 그 이상의 모든 span을 출력한다.
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new(
        "zero2prod".into(),
        // 포맷이 적용된 span 들을 stdout으로 출력한다.
        std::io::stdout,
    );

    let subscriber = get_subscriber("zero2prod".into(), "info".into());
    init_subscriber(subscriber);

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
    run(listener, connection_pool)?.await?;
    Ok(())
}


