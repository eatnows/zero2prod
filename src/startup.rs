use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use actix_web::web::Data;
use tracing_actix_web::TracingLogger;

// `run`을 `public`으로 마크해야한다.
// `run`은 더 이상 바이너리 엔트리 포인트가 아니므로, proc-macro 주문 없이 async로 마크할 수 있다.
pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // 커넥션을 스마트 포인터로 감싼다.
    let db_pool = Data::new(db_pool);
    // 주변 환경으로부터 `connection`을 잡아낸다.
    let server = HttpServer::new(move || {
        App::new()
            // `Logger::default` 를 대신한다.
            .wrap(TracingLogger::default())
            // APP에 새로운 엔드포인트를 추가할 때는 어떻게 해야하는가?
            // web::get()은 Route::new().guard(guard::Get())를 간략하게 표현 한 것.
            // path, Route 구조체의 인스턴스.  (Route는  하나의 핸들러와 일련의 가드들을 조합한 것이다.)
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // 포인터 사본을 얻어 애플리케이션 상태에 추가한다.
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    // .await
    Ok(server)
}
