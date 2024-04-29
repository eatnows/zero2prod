use tracing::dispatcher::set_global_default;
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

// `impl Subscriber`를 반환 타입으로 사용해서 반환된 subscriber의 실제 타입에 관한 설명 (매우 복잡하다)를 피한다.
// 반환된 subscriber를 `init_subscriber`로 나중에 전달하기 위해,
// 명시적으로 `Send`이고 `Sync`임을 알려야 한다.
pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    // 이 이상한 구문은 higher-ranked trait bound (HRTB)이다.
    // 기본적으로 Sink가 모든 라이프타입 파라미터 `'a` 에 대해
    // `MakeWriter` 트레이트를 구현한다는 것을 의미한다.
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    let formatting_layer = BunyanFormattingLayer::new(name, sink);
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

// subscriber를 글로벌 기본값으로 등록해서 span 데이터를 처리한다.
// 한 차례만 호출되어야 한다!
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber.into()).expect("Failed to set subscriber");
}
