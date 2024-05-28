FROM lukemathwalker/cargo-chef:latest-rust-1.77.0 as chef
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef as planner
COPY . .
# Compute a lock-like file for our project
# (프로젝트를 위한 lock 유사 파일을 계산한다.)
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our applicaiotn!
# (애플리케이션이 아닌 프로젝트 디펜던시를 빌드한다.)
RUN cargo chef cook --release --recipe-path recipe.json
# Up to this point, if our dependency tree stays the same,
# all layers should be cached.
# (이 지점까지 디펜던시 트리가 이전과 동일하게 유지되면,
# 모든 레이어는 캐시되어야 한다.)
COPY . .
ENV SQLX_OFFLINE true
# Build our project
# (프로젝트를 빌드한다.)
RUN cargo build --release --bin zero2prod

# Runtime stage(Runtime 단계)
FROM debian:bookworm-slim AS runtime

WORKDIR /app
# Install OpenSSL - it is dynamically linked by some of our dependencies
# (OpenSSL을 설치한다. - 일부 디펜던시에 의해 동적으로 링크된다.)
# Install ca-certificates - it is needed to verify TLS certificates
# when establishing HTTPS connections
# (ca-certificatates를 설치한다. - HTTPS 연결을 수립할 때 TSL 인증을 검증할 때 필요하다.)
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up (클린 업)
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
# Copy the compiled binary from the builder environment
# to our runtime environment
# (컴파일된 바이너리를 builder 환경에서 runtime 환경으로 복사한다.)
COPY --from=builder /app/target/release/zero2prod zero2prod
# We need the configuration file at runtime!
# (runtime에서의 구성 파일이 필요하다!)
COPY configuration configuration
ENV APP_ENVIRONMENT production
# When `docker run` is executed, launch the binary!
# (`docker run`이 실행되면, 바이너리를 구동한다.)
ENTRYPOINT ["./zero2prod"]