use crate::ProviderBase;

/// Types which can be queried without mutation
pub trait Queryable<Q: Query>: ProviderBase {
    /// Query the value with `request` to produce an answer or error
    fn query(&'p self, query: Q) -> Result<Q::Answer<'p>, Self::QueryError>;
}

/// A query is parameterized by the Provider's lifetime and has an associated answer
pub trait Query {
    /// The answer for this query, which may have the [Provider](crate::Provider)'s lifetime `'p`
    type Answer<'p>;
}
