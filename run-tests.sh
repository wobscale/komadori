#!/bin/bash

DEBUG=${DEBUG:-false}

set -eu
if [[ "$DEBUG" == "true" ]]; then
  set -x
fi

export UID

compose() {
  docker-compose -f docker-compose-integ.yaml -p komadori-integ $@
}

compose kill || true
compose rm -f || true
compose up -d

if [[ "${DEBUG}" != "true" ]]; then
  trap 'compose kill || true; compose rm -f || true' EXIT
fi

pid=$(docker inspect -f "{{ .State.Pid }}" komadoriinteg_hydra_1)
loops=20
for i in {1..$loops}; do
  if sudo nsenter -t $pid -n -- curl -s --fail -H "Accept: application/json" "http://127.0.0.1:4444/health/status">/dev/null; then
    break
  fi
  if [[ "$i" == "$loops" ]]; then
    echo "Hydra not ready in time"
    exit 2
  fi
  sleep 2
done

for i in {1..10}; do
  if docker exec -i komadoriinteg_db_1 psql --host localhost --user postgres postgres <<<"SELECT 1">/dev/null; then
    break
  fi
  if [[ "$i" == "10" ]]; then
    echo "Postgres not ready in time"
    exit 2
  fi
  sleep 2
done

source <(sed 's/^/export /g' ./hack/compose/komadori_env)

export HYDRA_URL="http://localhost:4444"
export DATABASE_URL="postgres://komadori:birdpassword@localhost:5432/komadori?sslmode=disable"

U=$USER

extra_args=()
"$DEBUG" == "true" && extra_args+="--nocapture"

sudo -E nsenter -t $pid -n -- sudo -E -u "$U" -- $(which cargo) test -- --test-threads=1 "${extra_args[@]}"
