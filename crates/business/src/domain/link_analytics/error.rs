use crate::domain::common::error::BaseDomainError;

#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub enum AnalyticsDomainError {
    #[error(transparent)]
    Base(#[from] BaseDomainError),
}
