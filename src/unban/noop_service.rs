use async_trait::async_trait;
use crate::api::UnBanRequest;
use crate::model::UnBanEntity;
use crate::unban::{UnBanner, UnbanStatus};

pub struct NoOpService {}

#[async_trait]
impl UnBanner for NoOpService {
    async fn unban(&self, _ur: UnBanRequest) -> Result<(), Vec<UnbanStatus>> {
        Ok(())
    }
}
