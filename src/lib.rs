use std::net::TcpListener;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use actix_web::dev::Server;

async fn health_check() -> impl Responder {
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
    })
        .listen(listener)?
        .run();
        // .await
    Ok(server)
}