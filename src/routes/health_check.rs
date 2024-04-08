use actix_web::HttpResponse;

// `impl Responder`를 초반에 반환한다.
// `actix-web`에 익숙해졌으므로, 주어진 타입을 명시적으로 기술한다.
// 이로 인한 성능의 차이는 없다. 그저 스타일과 관련된 선택일 뿐이다.
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}