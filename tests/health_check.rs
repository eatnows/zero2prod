// tokio::test 는 테스팅에 있어서 `tokio::main` 과 동등하다.
// #[test] 속성을 지정하는 수고를 덜 수 있다.

use chrono::format;
use once_cell::sync::Lazy;
use secrecy::ExposeSecret;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::fmt::format;
use std::net::TcpListener;
use tracing_subscriber::fmt::init;
use uuid::Uuid;
use zero2prod::configuration::{get_configuration, DatabaseSettings};
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

// cargo expand --test health_check (<- 테스트 파일의 이름) 을 사용해서
// 코드가 무엇을 생성하는지 확인할 수 있다.
#[tokio::test]
async fn health_check_works() {
    // Arrange (준비)
    let address = spawn_app().await.address;
    // `reqwest` 를 사용해서 애플리케이션에 대한 HTTP 요청을 수행한다.
    let client = reqwest::Client::new();

    // Act (조작)
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Filed to execute request.");

    // Assert (결과 확인)
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// `once_cell`을 사용해서 `tracing` 스택이 한 번만 초기화되는 것을 보장한다.
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    // `get_subscriber`의 출력을 `TEST_LOG`의 값에 기반해서 변수에 할당할 수 없다.
    // 왜냐하면 해당 sink는 `get_subscriber`에 의해 반환된 타입의 일부이고,
    // 그들의 타입이 같지 않기 때문이다. 이 상황을 회피할 수는 있지만,
    // 이 방법이 이후 과정을 진행할 수 있는 가장 직관적인 방법이다.

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

async fn spawn_app() -> TestApp {
    // `initialize`가 첫 번째 호출되면 `TRACING` 안의 코드가 실행된다.
    // 다른 모든 호출은 실행을 건너뛴다.
    Lazy::force(&TRACING);

    // OS에서 0번 포트는 특별하다.
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    // OS가 할당한 포트 번호를 추출한다.
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_configuration().expect("Failed to read configuration.");
    configuration.database.database_name = format!("newsletter_{}", Uuid::new_v4().to_string());

    let connection_pool = configure_database(&configuration.database).await;

    let server = run(listener, connection_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    TestApp {
        address,
        db_pool: connection_pool,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // 데이터베이스를 생성한다.
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres.");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    // 데이터베이스를 마이그레이션한다.
    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.");
    // sqlx-cli 에서 sqlx migrate run 을 실행할 때 사용한 것과 동일한 매크로이다.
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let app_address = spawn_app().await.address;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // 테스트 실패 시 출력할 커스터마이즈된 추가 오류 메시지
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        )
    }
}
