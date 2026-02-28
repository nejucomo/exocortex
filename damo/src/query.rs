use crate::ProviderBase;

/// Types which can be queried without mutation
pub trait Query<Q>: ProviderBase {
    /// The type of data returned on success
    type Answer;

    /// Query the value with `request` to produce an answer or error
    fn query(self, request: Q) -> Result<Self::Answer, Self::QueryError>;
}
