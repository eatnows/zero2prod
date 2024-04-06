// tokio::test 는 테스팅에 있어서 `tokio::main` 과 동등하다.
// #[test] 속성을 지정하는 수고를 덜 수 있다.

use std::net::TcpListener;

// cargo expand --test health_check (<- 테스트 파일의 이름) 을 사용해서
// 코드가 무엇을 생성하는지 확인할 수 있다.
#[tokio::test]
async fn health_check_works() {
    // Arrange (준비)
    let address = spawn_app();
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

fn spawn_app() -> String {
    // OS에서 0번 포트는 특별하다.
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");

    // OS가 할당한 포트 번호를 추출한다.
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    // 애플리케이션 주소를 호출자에게 반환한다.
    format!("http://127.0.0.1:{}", port)
}

