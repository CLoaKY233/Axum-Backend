# `hlt` Crate

A lightweight and extensible health check framework for Axum applications.

## Features

- Implement the `HealthCheck` trait to create custom health checks.
- Register checkers with the `HealthRegistry`.
- Run checks concurrently with timeouts.
- Expose results via an Axum handler.
