ETCD_CLUSTER_DOCKER_COMPOSE ?= etcd-docker-compose.yaml
ETCD_CLUSTER_WITH_TLS ?= false
ETCD_NODE ?= etcd-1

.PHONY: build
build:
	cargo build

.PHONY: test
test:
	RUST_BACKTRACE=1 cargo test -- --test-threads=1;
	cargo check --no-default-features

.PHONY: publish
publish:
	cargo package && cargo publish

.PHONY: setup-etcd-cluster
setup-etcd-cluster: teardown-etcd-cluster
	sh ./scripts/generate_etcd_cluster.sh ${ETCD_CLUSTER_DOCKER_COMPOSE} ${ETCD_CLUSTER_WITH_TLS};
	docker-compose -f ${ETCD_CLUSTER_DOCKER_COMPOSE} up -d

.PHONY: start-etcd-node
start-etcd-node:
	if [[ -f ${ETCD_CLUSTER_DOCKER_COMPOSE} ]]; \
	then docker-compose -f ${ETCD_CLUSTER_DOCKER_COMPOSE} start ${ETCD_NODE}; fi

.PHONY: stop-etcd-node
stop-etcd-node:
	if [[ -f ${ETCD_CLUSTER_DOCKER_COMPOSE} ]]; \
	then docker-compose -f ${ETCD_CLUSTER_DOCKER_COMPOSE} stop ${ETCD_NODE}; fi

.PHONY: teardown-etcd-cluster
teardown-etcd-cluster:
	if [[ -f ${ETCD_CLUSTER_DOCKER_COMPOSE} ]]; \
	then docker-compose -f ${ETCD_CLUSTER_DOCKER_COMPOSE} down && rm ${ETCD_CLUSTER_DOCKER_COMPOSE}; fi

.PHONY: clean
clean: teardown-etcd-cluster
	cargo clean
