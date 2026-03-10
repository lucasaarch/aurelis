use axum::{Json, extract::{FromRequest, Query, Request}, http::StatusCode, response::{IntoResponse, Response}};
use serde::de::DeserializeOwned;
use validator::{Validate, ValidationErrors};

use crate::error::ErrorResponse;

pub struct ValidatedBody<T>(pub T);

impl<S, T> FromRequest<S> for ValidatedBody<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|err| err.into_response())?;

        if let Err(validation_errors) = value.validate() {
            let error = ErrorResponse::new(
                StatusCode::BAD_REQUEST,
                "VALIDATION_FAILED",
                format_validation_errors(&validation_errors),
            );

            return Err(error.into_response());
        }

        Ok(ValidatedBody(value))
    }
}

pub struct ValidatedQuery<T>(pub T);

impl<S, T> FromRequest<S> for ValidatedQuery<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Query(value) = Query::<T>::from_request(req, state)
            .await
            .map_err(|err| err.into_response())?;

        if let Err(validation_errors) = value.validate() {
           let error = ErrorResponse::new(
                StatusCode::BAD_REQUEST,
                "VALIDATION_FAILED",
                format_validation_errors(&validation_errors),
            );


            return Err(error.into_response());
        }

        Ok(ValidatedQuery(value))
    }
}


fn format_validation_errors(errors: &ValidationErrors) -> String {
    errors
        .field_errors()
        .iter()
        .map(|(field, errs)| {
            let messages = errs
                .iter()
                .map(|e| {
                    e.message
                        .as_deref()
                        .unwrap_or(e.code.as_ref())
                        .to_string()
                })
                .collect::<Vec<_>>()
                .join(", ");

            format!("{field}: {messages}")
        })
        .collect::<Vec<_>>()
        .join("; ")
}