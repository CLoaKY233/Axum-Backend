use axum::response::IntoResponse;
use err::{AppError, DatabaseError, EnvironmentError, Result, SshError};

#[test]
fn test_result_type_alias() {
    fn example_function(should_fail: bool) -> Result<String> {
        if should_fail {
            Err(DatabaseError::NotFound("test".into()).into())
        } else {
            Ok("success".to_string())
        }
    }

    let result = example_function(false);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "success");

    let error_result = example_function(true);
    assert!(error_result.is_err());
}

#[test]
fn test_error_propagation_with_question_mark() {
    fn inner_function() -> Result<()> {
        Err(DatabaseError::NotFound("user".into()).into())
    }

    fn outer_function() -> Result<String> {
        inner_function()?; // Error propagates automatically
        Ok("success".to_string())
    }

    let result = outer_function();
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        AppError::Database(DatabaseError::NotFound(_))
    ));
}

#[test]
fn test_all_domain_error_conversions() {
    // Database error conversion
    let db_err: AppError = DatabaseError::ConnectionError("test".into()).into();
    assert!(matches!(db_err, AppError::Database(_)));

    // SSH error conversion
    let ssh_err: AppError = SshError::TimeoutError("test".into()).into();
    assert!(matches!(ssh_err, AppError::Ssh(_)));

    // Environment error conversion
    let env_err: AppError = EnvironmentError::NotFoundError("test".into()).into();
    assert!(matches!(env_err, AppError::Environment(_)));
}

#[test]
fn test_http_response_mapping() {
    let test_cases = vec![
        (
            AppError::Database(DatabaseError::NotFound("test".into())),
            404,
        ),
        (
            AppError::Database(DatabaseError::AuthenticationError("test".into())),
            401,
        ),
        (AppError::Ssh(SshError::TimeoutError("test".into())), 408),
        (
            AppError::Environment(EnvironmentError::NotFoundError("test".into())),
            500,
        ),
    ];

    for (error, expected_status) in test_cases {
        let response = error.into_response();
        assert_eq!(response.status().as_u16(), expected_status);
    }
}

#[test]
fn test_error_source_chain() {
    use std::error::Error;
    let db_error = DatabaseError::QueryError("SQL error".to_string());
    let app_error: AppError = db_error.into();
    assert!(app_error.source().is_some());
}
