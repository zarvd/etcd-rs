#!/usr/bin/env bash

DOCKER_COMPOSE_FILE=$1
DOCKER_COMPOSE_FILE=${DOCKER_COMPOSE_FILE:-docker-compose.yaml}

VERSION=$2
VERSION=${VERSION:-v3.5.2}

WITH_TLS=$3
WITH_TLS=${WITH_TLS:-false}

IMAGE=quay.io/coreos/etcd:${VERSION}

# TODO generate TLS config

cat > "${DOCKER_COMPOSE_FILE}" <<EOF
version: "3"

services:
  etcd-1:
      image: ${IMAGE}
      container_name: etcd-1
      hostname: etcd-1
      command: >-
        /usr/local/bin/etcd
        -name etcd-1
        -advertise-client-urls http://etcd-1:12379 -listen-client-urls http://0.0.0.0:12379
        -initial-advertise-peer-urls http://etcd-1:12380 -listen-peer-urls http://0.0.0.0:12380
        -initial-cluster-token etcd-cluster
        -initial-cluster etcd-1=http://etcd-1:12380,etcd-2=http://etcd-2:22380,etcd-3=http://etcd-3:32380
        -initial-cluster-state new
      ports:
        - "12379:12379"
        - "12380:12380"    

  etcd-2:
    image: ${IMAGE}
    container_name: etcd-2
    hostname: etcd-2
    command: >-
      /usr/local/bin/etcd
      -name etcd-2
      -advertise-client-urls http://etcd-2:22379 -listen-client-urls http://0.0.0.0:22379
      -initial-advertise-peer-urls http://etcd-2:22380 -listen-peer-urls http://0.0.0.0:22380
      -initial-cluster-token etcd-cluster
      -initial-cluster etcd-1=http://etcd-1:12380,etcd-2=http://etcd-2:22380,etcd-3=http://etcd-3:32380
      -initial-cluster-state new
    ports:
      - "22379:22379"
      - "22380:22380"

  etcd-3:
    image: ${IMAGE}
    container_name: etcd-3
    hostname: etcd-3
    command: >-
      /usr/local/bin/etcd
      -name etcd-3
      -advertise-client-urls http://etcd-3:32379 -listen-client-urls http://0.0.0.0:32379
      -initial-advertise-peer-urls http://etcd-3:32380 -listen-peer-urls http://0.0.0.0:32380
      -initial-cluster-token etcd-cluster
      -initial-cluster etcd-1=http://etcd-1:12380,etcd-2=http://etcd-2:22380,etcd-3=http://etcd-3:32380
      -initial-cluster-state new
    ports:
      - "32379:32379"
      - "32380:32380"
EOF