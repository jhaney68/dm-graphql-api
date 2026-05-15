//! A GraphQL API implemented in Rust
//!
//! This crate provides the core functionality for the dm-graphql-api project,
//! including GraphQL schema definitions, resolvers, and data access layers.

/// Main module for the GraphQL API
pub mod api;

/// Module for database operations
pub mod database;

/// Module for GraphQL schema definitions
pub mod schema;

/// Module for utility functions
pub mod utils;

/// Re-export of the main GraphQL schema
pub use schema::create_schema;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_creation() {
        let schema = create_schema();
        assert!(schema.is_ok());
    }
}
