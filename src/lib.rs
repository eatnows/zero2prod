use std::net::TcpListener;
use actix_web::{App, HttpResponse, HttpServer, web};
use actix_web::dev::Server;

// `impl Responder`를 초반에 반환한다.
// `actix-web`에 익숙해졌으므로, 주어진 타입을 명시적으로 기술한다.
// 이로 인한 성능의 차이는 없다. 그저 스타일과 관련된 선택일 뿐이다.
async fn health_check() -> HttpResponse {

    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

// `run`을 `public`으로 마크해야한다.
// `run`은 더 이상 바이너리 엔트리 포인트가 아니므로, proc-macro 주문 없이 async로 마크할 수 있다.
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        // 애플리케이션 로직 (라우팅, 미들웨어, 요청 핸들러 등)이 동작하는 곳.
        // App은 유입 요청을 입력으로 받아 응답을 출력하는 컴포넌트이다.
        App::new()
            // APP에 새로운 엔드포인트를 추가할 때는 어떻게 해야하는가?
            // web::get()은 Route::new().guard(guard::Get())를 간략하게 표현 한 것.
            // path, Route 구조체의 인스턴스.  (Route는  하나의 핸들러와 일련의 가드들을 조합한 것이다.)
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
        .listen(listener)?
        .run();
        // .await
    Ok(server)
}