pub trait CommandHandler<TResult> {
    fn execute(&self) -> TResult;
}