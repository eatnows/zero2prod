// tokio::test 는 테스팅에 있어서 `tokio::main` 과 동등하다.
// #[test] 속성을 지정하는 수고를 덜 수 있다.

// cargo expand --test health_check (<- 테스트 파일의 이름) 을 사용해서
// 코드가 무엇을 생성하는지 확인할 수 있다.
#[tokio::test]
async fn health_check_works() {
    // Arrange (준비)
    spawn_app();
    // `reqwest` 를 사용해서 애플리케이션에 대한 HTTP 요청을 수행한다.
    let client = reqwest::Client::new();

    // Act (조작)
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Filed to execute request.");

    // Assert (결과 확인)
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}

