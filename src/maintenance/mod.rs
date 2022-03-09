use async_trait::async_trait;

#[async_trait]
pub trait MaintenanceOp {
    async fn alarm(&self);
    async fn status(&self);
    async fn defragment(&self);
    async fn hash(&self);
    async fn hash_kv(&self);
    async fn snapshot(&self);
    async fn move_leader(&self);
}
