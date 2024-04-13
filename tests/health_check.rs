// tokio::test 는 테스팅에 있어서 `tokio::main` 과 동등하다.
// #[test] 속성을 지정하는 수고를 덜 수 있다.

use std::fmt::format;
use std::net::TcpListener;
use chrono::format;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use zero2prod::configuration::{DatabaseSettings, get_configuration};
use zero2prod::startup::run;

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
        .get(&format!("{}/health_check",  &address))
        .send()
        .await
        .expect("Filed to execute request.");

    // Assert (결과 확인)
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

async fn spawn_app() -> TestApp {
    // OS에서 0번 포트는 특별하다.
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");

    // OS가 할당한 포트 번호를 추출한다.
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_configuration().expect("Failed to read configuration.");
    configuration.database.database_name = format!("newsletter_{}", Uuid::new_v4().to_string());

    let connection_pool = configure_database(&configuration.database).await;

    // let connection_pool = PgPool::connect(
    //     &configuration.database.connection_string()
    // )
    //     .await
    //     .expect("Failed to connect to Postgres");

    let server = run(listener, connection_pool.clone())
        .expect("Failed to bind address");
    let _ = tokio::spawn(server);
    TestApp {
        address,
        db_pool: connection_pool,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // 데이터베이스를 생성한다.
    let mut connection = PgConnection::connect(
        &config.connection_string_without_db()
    )
        .await
        .expect("Failed to connect to Postgres.");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    // 데이터베이스를 마이그레이션한다.
    let connection_pool = PgPool::connect(&config.connection_string())
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


