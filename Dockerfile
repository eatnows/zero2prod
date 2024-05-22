# We use the latest Rust stable release as base image
# (기본 이미지로 최신 러스트 stable 릴리스를 사용한다.)
FROM rust:1.77.0

# Let's switch our working directory to `app` (equivalent to `cd app`)
# The `app` folder will be created for us by Docker in case it does not exist already.
# (작업 디렉터리를 `app`으로 변경한다. (`cd app`과 동일).)
# (`app` 폴더가 존재하지 않는 경우 도커가 해당 폴더를 생성한다.)
WORKDIR /app
# Install the required system dependencies for our linking configuration
# (구성을 연결하기 위해 필요한 시스템 디펜던시를 설치한다.)
RUN apt update && apt install lld clang -y
# Copy all files form out working environment to our Docker image
# (작업 환경의 모든 파일을 도커 이미지로 복사한다.)
COPY . .
ENV SQLX_OFFLINE true
# Let's build our binary!
# We'll use the release profile to make it faaast
# (바이너리를 빌드하자.)
# (빠르게 빌드하기 위해 release 프로파일을 사용한다.)
RUN cargo build --release
ENV APP_ENVIRONMENT production
# When `docker run` is executed, launch the binary!
# (`docker run`이 실행되면, 바이너리를 구동한다.)
ENTRYPOINT ["./target/release/zero2prod"]