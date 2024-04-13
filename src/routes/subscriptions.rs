use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use sqlx::types::chrono::Utc;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String
}

pub async fn subscribe(
    form: web::Form<FormData>,
    // 애플리케이션 상태에서 커넥션을 꺼낸다.
    pool: web::Data<PgPool>
) -> HttpResponse {
    // `Result` 는 `OK` 와 `Err` 라는 두 개의 변형 (variant)을 갖는다.
    // 첫 번째는 성공, 두 번째는 실패를 의미한다.
    // `match` 구문을 사용해서 결과에 따라 무엇을 수행할지 선택한다.
    // `Result`에 관해서는 차차 자세히 설명한다!
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
        // `get_ref` 를 사용해서 `web::Data`로 감싸진 `PgConnection`에 대한 불변 참조 (immutable reference)를 얻는다.
        .execute(pool.get_ref())
        .await
        {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(e) => {
                println!("Failed to execute query: {}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
}