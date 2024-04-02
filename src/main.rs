use actix_web::{App, HttpRequest, HttpServer, Responder, web};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

// HttpServer::run은 비동기 메서드이기 때문에 main은 비동기여야 한다.
// 그러나 바이너리 엔트리 포인트인 main은 비동기가 될 수 없다.
// 러스트에서의 비동기 프로그래밍은 Future 트레이트 위에 만들어져 있다.
// Future란 아직 설정되지 않은 값을 의미한다. 하나의 poll 메서드를 노출하며, 이를 호출하면 퓨처가 진행되어 결과적으로 최종값을 갖게 된다.
// 러스트의 퓨처는 lazy라고 생각해도 좋다.
// main 함수 위에서 비동기 런타임을 실행하고, 이를 사용해 퓨처가 완료되도록 해야 한다.
#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        // 애플리케이션 로직 (라우팅, 미들웨어, 요청 핸들러 등)이 동작하는 곳.
        // App은 유입 요청을 입력으로 받아 응답을 출력하는 컴포넌트이다.
        App::new()
            // APP에 새로운 엔드포인트를 추가할 때는 어떻게 해야하는가?
            // web::get()은 Route::new().guard(guard::Get())를 간략하게 표현 한 것.
            .route("/", web::get().to(greet))
            // path, Route 구조체의 인스턴스.  (Route는  하나의 핸들러와 일련의 가드들을 조합한 것이다.)
            .route("/{name}", web::get().to(greet))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
