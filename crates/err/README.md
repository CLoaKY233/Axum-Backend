# `err` Crate

This crate provides centralized error handling for the application.

## Features

- A unified `AppError` enum.
- Domain-specific error types for database, SSH, and environment errors.
- Automatic conversion of errors to Axum responses.
