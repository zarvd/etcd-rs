#!/usr/bin/env bash

CUR_DIR=$(dirname "$0")

DOCKER_COMPOSE_FILE=$1
DOCKER_COMPOSE_FILE=${DOCKER_COMPOSE_FILE:-docker-compose.yaml}

VERSION=$2
VERSION=${VERSION:-v3.5.2}

WITH_TLS=$3
WITH_TLS=${WITH_TLS:-false}

IMAGE=quay.io/coreos/etcd:${VERSION}

SCHEME="http"
ETCD_1_TLS_OPTIONS=""
ETCD_2_TLS_OPTIONS=""
ETCD_3_TLS_OPTIONS=""
if [[ ${WITH_TLS} != 'false' ]]; then
  SCHEME="https"

  ETCD_1_TLS_OPTIONS=$(cat <<-EOF
      --client-cert-auth
      --trusted-ca-file /opt/certs/ca.pem
      --cert-file /opt/certs/etcd-1.pem
      --key-file /opt/certs/etcd-1-key.pem
      --peer-client-cert-auth
      --peer-trusted-ca-file /opt/certs/ca.pem
      --peer-cert-file /opt/certs/etcd-1.pem
      --peer-key-file /opt/certs/etcd-1-key.pem
EOF
)

  ETCD_2_TLS_OPTIONS=$(cat <<-EOF
      --client-cert-auth
      --trusted-ca-file /opt/certs/ca.pem
      --cert-file /opt/certs/etcd-2.pem
      --key-file /opt/certs/etcd-2-key.pem
      --peer-client-cert-auth
      --peer-trusted-ca-file /opt/certs/ca.pem \
      --peer-cert-file /opt/certs/etcd-2.pem
      --peer-key-file /opt/certs/etcd-2-key.pem
EOF
)

  ETCD_3_TLS_OPTIONS=$(cat <<-EOF
      --client-cert-auth
      --trusted-ca-file /opt/certs/ca.pem
      --cert-file /opt/certs/etcd-3.pem
      --key-file /opt/certs/etcd-3-key.pem
      --peer-client-cert-auth
      --peer-trusted-ca-file /opt/certs/ca.pem \
      --peer-cert-file /opt/certs/etcd-3.pem
      --peer-key-file /opt/certs/etcd-3-key.pem
EOF
)

fi


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
      -advertise-client-urls ${SCHEME}://etcd-1:12379 -listen-client-urls ${SCHEME}://0.0.0.0:12379
      -initial-advertise-peer-urls ${SCHEME}://etcd-1:12380 -listen-peer-urls ${SCHEME}://0.0.0.0:12380
      -initial-cluster-token etcd-cluster
      -initial-cluster etcd-1=${SCHEME}://etcd-1:12380,etcd-2=${SCHEME}://etcd-2:22380,etcd-3=${SCHEME}://etcd-3:32380
      -initial-cluster-state new
${ETCD_1_TLS_OPTIONS}
    volumes:
      - ${CUR_DIR}/certs:/opt/certs
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
      -advertise-client-urls ${SCHEME}://etcd-2:22379 -listen-client-urls ${SCHEME}://0.0.0.0:22379
      -initial-advertise-peer-urls ${SCHEME}://etcd-2:22380 -listen-peer-urls ${SCHEME}://0.0.0.0:22380
      -initial-cluster-token etcd-cluster
      -initial-cluster etcd-1=${SCHEME}://etcd-1:12380,etcd-2=${SCHEME}://etcd-2:22380,etcd-3=${SCHEME}://etcd-3:32380
      -initial-cluster-state new
${ETCD_2_TLS_OPTIONS}
    volumes:
      - ${CUR_DIR}/certs:/opt/certs
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
      -advertise-client-urls ${SCHEME}://etcd-3:32379 -listen-client-urls ${SCHEME}://0.0.0.0:32379
      -initial-advertise-peer-urls ${SCHEME}://etcd-3:32380 -listen-peer-urls ${SCHEME}://0.0.0.0:32380
      -initial-cluster-token etcd-cluster
      -initial-cluster etcd-1=${SCHEME}://etcd-1:12380,etcd-2=${SCHEME}://etcd-2:22380,etcd-3=${SCHEME}://etcd-3:32380
      -initial-cluster-state new
${ETCD_3_TLS_OPTIONS}
    volumes:
      - ${CUR_DIR}/certs:/opt/certs
    ports:
      - "32379:32379"
      - "32380:32380"
EOF