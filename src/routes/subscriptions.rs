use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use sqlx::types::chrono::Utc;
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(
    form: web::Form<FormData>,
    // 애플리케이션 상태에서 커넥션을 꺼낸다.
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let request_id = Uuid::new_v4();
    // Spans 는 logs와 같은 연관 레벨을 갖는다.
    // info_span 은 info 레벨의 span을 생성한다.
    // %: tracing에게 로깅 목적으로 이들의 Display 구현을 사용하라는 의미
    let request_span = tracing::info_span!(
        "Adding a new subscriber.",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name,
    );
    let _request_span_guard = request_span.enter();

    // query span에 대해 `.enter`를 호출하지 않는다.
    // `.instrument` 쿼리 퓨처 수명 주기 안에서 적절한 시점에 이를 관리한다.
    let query_span = tracing::info_span!(
        "Saving new subscriber details in the database"
    );
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
        // 먼저 인스트루멘테이션을 붙윈 뒤, 대기 (`.await`) 한다.
        .instrument(query_span)
        .await
    {
        Ok(_) => {
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            // 그렇다, 이 오류 로그는 `query_span` 밖으로 떨어진다.
            // 맹세컨대, 나중에 바로잡을 것이다.
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}