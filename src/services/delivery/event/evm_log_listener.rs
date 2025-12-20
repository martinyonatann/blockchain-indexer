use std::{
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use tower::Service;

use crate::services::usecase::{IndexLogUC, errors::AppError};

pub struct EVMLogListener<U> {
    /// Domain use case
    pub usecase: Arc<U>,

    /// Chain identifier
    pub chain_id: u64,

    /// Contract address to listen on
    pub address: String,
}

impl<U> EVMLogListener<U> {
    pub fn new(usecase: Arc<U>, chain_id: u64, address: String) -> Self {
        Self {
            usecase,
            chain_id,
            address,
        }
    }
}

impl<U> Service<()> for EVMLogListener<U>
where
    U: IndexLogUC + Send + Sync + 'static,
{
    type Response = ();
    type Error = AppError;
    type Future = Pin<Box<dyn Future<Output = Result<(), AppError>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _req: ()) -> Self::Future {
        let usecase = self.usecase.clone();
        let chain_id = self.chain_id;
        let address = self.address.clone();

        Box::pin(async move { usecase.execute(chain_id, address).await })
    }
}
