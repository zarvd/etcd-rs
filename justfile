DOCKER_COMPOSE_SPEC := "etcd-docker-compose.yaml"

default:
    just --list

# Build the project
build:
    cargo build

# Format code with rust
fmt:
    cargo fmt

# Lint code with clippy
lint:
    cargo fmt --all -- --check
    cargo clippy --all-targets --all-features

# Run unit tests
unit-test:
    cargo nextest run
    cargo test --doc

# Run integration tests
integration-test:
    #!/usr/bin/env bash
    set -e
    pushd integration-tests
    cargo nextest run --test-threads=1 --retries 5
    popd

# Download etcdctl
etcdctl:
    ./hack/download_etcd.sh

setup-etcd-cluster tls="false" version="v3.5.2": teardown-etcd-cluster
    ./hack/generate_etcd_cluster.sh {{ DOCKER_COMPOSE_SPEC }} {{ version }} {{ tls }}
    docker-compose -f {{ DOCKER_COMPOSE_SPEC }} up -d

teardown-etcd-cluster:
    docker-compose -f {{ DOCKER_COMPOSE_SPEC }} down || true

start-etcd-node node:
    docker-compose -f {{ DOCKER_COMPOSE_SPEC }} start {{ node }}

stop-etcd-node node:
    docker-compose -f {{ DOCKER_COMPOSE_SPEC }} stop {{ node }}

etcd-cluster-status:
    docker-compose -f {{ DOCKER_COMPOSE_SPEC }} ps
    etcd/etcdctl endpoint status --endpoints=127.0.0.1:12379,127.0.0.1:22379,127.0.0.1:32379 -w table