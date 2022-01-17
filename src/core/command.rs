#[async_trait]
pub trait CommandHandler<TResult> {
    async fn execute(&self) -> TResult;
}
