# API Documentation for dm-graphql-api

## Overview

This document describes the GraphQL API endpoints available in the dm-graphql-api project. The API is built using Rust with async-graphql and axum for high-performance GraphQL operations.

## Endpoints

### GraphQL Endpoint
- **Path**: `/graphql`
- **Method**: `POST`
- **Description**: Main GraphQL endpoint for executing queries and mutations

### GraphQL Playground
- **Path**: `/playground`
- **Method**: `GET`
- **Description**: Interactive GraphQL playground for testing queries and mutations

## Getting Started

### Basic Query
```graphql
query {
  # Your queries here
}
```

### Basic Mutation
```graphql
mutation {
  # Your mutations here
}
```

## Authentication

The API supports authentication through headers. Add the following header to your requests:

```
Authorization: Bearer <your-token>
```

## Error Handling

All errors are returned in the GraphQL error format with:
- `message`: Human-readable error description
- `locations`: Location of the error in the query
- `path`: Path to the field that caused the error

## Rate Limiting

The API implements rate limiting to prevent abuse:
- Default limit: 1000 requests per hour
- Custom limits may be applied based on authentication

## Versioning

This API follows semantic versioning. The current version is v1.0.0.

## Examples

### Fetching Data
```graphql
query {
  users {
    id
    name
    email
  }
}
```

### Creating Data
```graphql
mutation {
  createUser(input: {
    name: "John Doe"
    email: "john@example.com"
  }) {
    id
    name
    email
  }
}
```

### Updating Data
```graphql
mutation {
  updateUser(id: "1", input: {
    name: "Jane Doe"
  }) {
    id
    name
    email
  }
}
```

### Deleting Data
```graphql
mutation {
  deleteUser(id: "1") {
    success
  }
}
```

## Response Format

All responses follow the GraphQL specification with:
- `data`: The result of the query/mutation
- `errors`: Array of errors (if any)
- `extensions`: Additional information (if any)

## Status Codes

- `200 OK`: Successful request
- `400 Bad Request`: Invalid query or mutation
- `401 Unauthorized`: Authentication required
- `403 Forbidden`: Access denied
- `500 Internal Server Error`: Server error

## Future Enhancements

- WebSockets for real-time subscriptions
- Enhanced caching mechanisms
- More detailed rate limiting controls
- Improved monitoring and logging