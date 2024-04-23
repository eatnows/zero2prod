use std::fs::Metadata;

pub mod configuration;
pub mod routes;
pub mod startup;
pub mod telemetry;

// 로거에 필요한 작업을 캡슐화하는 트레이트
pub trait Log: Sync + Send {
    // 지정한 메타데이터의 로그 메시지를 로깅할지 결정한다.
    // `log_enabled!` 매크로는 이를 사용해서 메시지가 어쨋든 버려져야 한다면, 호출자가 값비싼 로그 메시지 인수 연산을 피하도록 한다.

    fn enabled(&self, metadata: &Metadata) -> bool;

    // `Record`를 로깅한다.
    // `enabled`를 이 메서드 전에 호출할 필요는 없다.
    // `log`의 구현은 모든 필요한 펄터링을 내부적으로 수행해야 한다.
    // fn log(&self, record: &Record);

    // 버퍼에 들어 있는 모든 레코드를 비운다.
    fn flush(&self);
}


