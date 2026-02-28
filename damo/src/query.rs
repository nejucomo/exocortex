use crate::ProviderBase;

/// Types which can be queried without mutation
pub trait Query<'a, Q>: ProviderBase {
    /// The type of data returned on success
    type Answer;

    /// Query the value with `request` to produce an answer or error
    fn query(&'a self, query: Q) -> Result<Self::Answer, Self::QueryError>;
}
