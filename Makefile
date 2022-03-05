ETCD_CLUSTER_DOCKER_COMPOSE ?= etcd-docker-compose.yaml
ETCD_CLUSTER_WITH_TLS ?= false
ETCD_NODE ?= etcd-1
ETCD_VERSION ?= v3.5.2

.PHONY: build
build:
	cargo build

.PHONY: test
test:
	RUST_BACKTRACE=full cargo test -- --test-threads=1 --nocapture;
	cargo check --no-default-features

.PHONY: publish
publish:
	cargo package && cargo publish

.PHONY: setup-etcd-cluster
setup-etcd-cluster: teardown-etcd-cluster
	./scripts/generate_etcd_cluster.sh ${ETCD_CLUSTER_DOCKER_COMPOSE} ${ETCD_VERSION} ${ETCD_CLUSTER_WITH_TLS};
	docker-compose -f ${ETCD_CLUSTER_DOCKER_COMPOSE} up -d

.PHONY: start-etcd-node
start-etcd-node:
ifneq ("$(wildcard ${ETCD_CLUSTER_DOCKER_COMPOSE})","")
	docker-compose -f ${ETCD_CLUSTER_DOCKER_COMPOSE} start ${ETCD_NODE}
endif

.PHONY: stop-etcd-node
stop-etcd-node:
ifneq ("$(wildcard ${ETCD_CLUSTER_DOCKER_COMPOSE})","")
	docker-compose -f ${ETCD_CLUSTER_DOCKER_COMPOSE} stop ${ETCD_NODE}
endif

.PHONY: teardown-etcd-cluster
teardown-etcd-cluster:
ifneq ("$(wildcard ${ETCD_CLUSTER_DOCKER_COMPOSE})","")
	docker-compose -f ${ETCD_CLUSTER_DOCKER_COMPOSE} down;
	rm ${ETCD_CLUSTER_DOCKER_COMPOSE}
endif

etcd/etcdctl:
	./scripts/download_etcd.sh

.PHONY: etcd-cluster-status
etcd-cluster-status: etcd/etcdctl
ifneq ("$(wildcard ${ETCD_CLUSTER_DOCKER_COMPOSE})","")
	docker-compose -f ${ETCD_CLUSTER_DOCKER_COMPOSE} ps;
	etcd/etcdctl endpoint status --endpoints=127.0.0.1:12379,127.0.0.1:22379,127.0.0.1:32379 -w table;
endif

.PHONY: clean
clean: teardown-etcd-cluster
	cargo clean
