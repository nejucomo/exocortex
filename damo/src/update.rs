use crate::ProviderBase;

/// Types which can mutate in response to an update request `U`
pub trait Update<U>: ProviderBase {
    /// Atomically mutate `self` based on `request`
    fn update(&mut self, request: U) -> Result<(), Self::UpdateError>;
}
