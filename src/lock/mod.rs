use async_trait::async_trait;

#[async_trait]
pub trait LockOp {
    async fn lock(&self);
}
