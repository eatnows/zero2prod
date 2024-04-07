#!/usr/bin/env bash
set -x
set -eo pipefail

# 먼저 psql 과 sqlx-cli 가 설치되어 있는지 확인한다.
if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: psql is not installed."
  exit 1
fi

if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: sqlx is not installed."
  echo >&2 "Use:"
  echo >&2 "  cargo install --version='~0.6' sqlx-cli \
--no-default-features --features rustls,postgres"
  echo >&2 "to install it."
  exit 1
fi

DB_USER="${POSTGRES_USER:=postgres}"
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=newsletter}"
DB_PORT="${POSTGRES_PORT:=5432}"

# 도커화된 Postgres 데이터베이스가 이미 실행 중이면 도커가 이 단계를 건너뛸 수 있게 한다.
if [[ -z "${SKIP_DOCKER}" ]]
then
  docker run \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    -p "${DB_PORT}":5432 \
    -v /Users/chodong/workspace/postgresql:/var/lib/postgresql/data \
    -d postgres \
    postgres -N 1000
fi


# Postgres가 명령어를 받아들일 준비가 될 때까지 핑을 유지한다.
export PGPASSWORD="${DB_PASSWORD}"
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
  >&2 echo "Postgres is still unavailable - sleeping"
  sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT} - running migrations now!"

DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
export DATABASE_URL
sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrate, ready to go!"
