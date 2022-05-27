use crate::model::UnBanEntity;
use crate::unban::{UnBanner, UnbanStatus};

pub struct NoOpService {}

#[async_trait]
impl UnBanner for NoOpService {
    async fn unban(&self, _ut: UnBanEntity) -> Result<(), Vec<UnbanStatus>> {
        Ok(())
    }
}
