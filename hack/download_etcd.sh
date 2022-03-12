#!/usr/bin/env bash

ETCD_VERSION=$1
ETCD_VERSION=${ETCD_VERSION:-v3.5.2}

ARCH=$2
ARCH=${ARCH:-linux-amd64}

wget https://github.com/etcd-io/etcd/releases/download/${ETCD_VERSION}/etcd-${ETCD_VERSION}-${ARCH}.tar.gz

tar -xf etcd-${ETCD_VERSION}-${ARCH}.tar.gz

mv etcd-${ETCD_VERSION}-${ARCH} etcd

echo "etcd ${ETCD_VERSION} ${ARCH} downloaded"