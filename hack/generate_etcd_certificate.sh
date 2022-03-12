#!/usr/bin/env bash

CUR_DIR=$(dirname "$0")

echo ${CUR_DIR}

cfssl gencert -initca ${CUR_DIR}/cfssl/ca-csr.json | cfssljson -bare ca -

rm -rf ${CUR_DIR}/certs
mkdir ${CUR_DIR}/certs
mv ca.csr ${CUR_DIR}/certs
mv ca.pem ${CUR_DIR}/certs
mv ca-key.pem ${CUR_DIR}/certs

create_certificate() {
  NODE=$1
  CN="lodrem-etcd-cluster-${NODE}"

  echo "{\"CN\": \"${CN}\", \"hosts\": [\"\"], \"key\":{\"algo\": \"rsa\", \"size\": 2048}}" | \
    cfssl gencert \
      -ca="${CUR_DIR}/certs/ca.pem" \
      -ca-key="${CUR_DIR}/certs/ca-key.pem" \
      -config="${CUR_DIR}/cfssl/ca-config.json" \
      -profile=server \
      -hostname="127.0.0.1,localhost,${NODE}" - | cfssljson -bare "${NODE}"

  echo "{\"CN\": \"${CN}\", \"hosts\": [\"\"], \"key\":{\"algo\": \"rsa\", \"size\": 2048}}" | \
    cfssl gencert \
      -ca="${CUR_DIR}/certs/ca.pem" \
      -ca-key="${CUR_DIR}/certs/ca-key.pem" \
      -config="${CUR_DIR}/cfssl/ca-config.json" \
      -profile=peer \
      -hostname="127.0.0.1,localhost,${NODE}" - | cfssljson -bare "${NODE}"

  mv ${NODE}.csr ${CUR_DIR}/certs/
  mv ${NODE}.pem ${CUR_DIR}/certs/
  mv ${NODE}-key.pem ${CUR_DIR}/certs/
}

NODES=(etcd-1 etcd-2 etcd-3)

for NODE in ${NODES[@]}; do
  create_certificate ${NODE}
done