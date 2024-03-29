use std::sync::{Arc, Mutex};

use actix_service::Service;
use futures::TryFutureExt;
use reqwest::{Client, Error};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::data::api_call_delegate::ApiCallError::{
    DtoToJsonStringError, JsonToDtoError, SendError,
};

pub struct ApiCallDelegate {
    api_client: Arc<Mutex<Client>>,
}

impl ApiCallDelegate {
    pub fn new(api_client: Arc<Mutex<Client>>) -> ApiCallDelegate {
        ApiCallDelegate { api_client }
    }

    // static string causes hidden lifetime
    pub async fn execute_post_call<T, O>(
        &self,
        url: String,
        dto: &T,
    ) -> Result<Box<O>, ApiCallError>
        where
            T: ?Sized + Serialize + DeserializeOwned,
            O: ?Sized + Serialize + DeserializeOwned,
    {
        let body = serde_json::to_string(dto).map_err(|_| DtoToJsonStringError)?;
        let result = self
            .api_client
            .lock()
            .unwrap()
            .post(url)
            .body(body)
            .send()
            .await;

        match &result {
            Ok(response) => {
                println!("{}", response.status());
            }
            Err(error) => {
                println!("{}", error);
            }
        }

        result
            .map_err(|err| Self::map_and_log_error(err, SendError))?
            .json::<Box<O>>()
            .await
            .map_err(|err| Self::map_and_log_error(err, JsonToDtoError))
    }

    pub async fn execute_get_call<O>(
        &self,
        url: String,
    ) -> Result<Box<O>, ApiCallError>
        where
            O: ?Sized + Serialize + DeserializeOwned,
    {
        let result = self
            .api_client
            .lock()
            .unwrap()
            .get(url)
            .send()
            .await;

        match &result {
            Ok(response) => {
                println!("{}", response.status());
            }
            Err(error) => {
                println!("{}", error);
            }
        }

        result
            .map_err(|err| Self::map_and_log_error(err, SendError))?
            .json::<Box<O>>()
            .await
            .map_err(|err| Self::map_and_log_error(err, JsonToDtoError))
    }


    fn map_and_log_error(err: Error, api_call_error: ApiCallError) -> ApiCallError {
        println!("{}", err);
        api_call_error
    }
}

pub enum ApiCallError {
    DtoToJsonStringError,
    SendError,
    JsonToDtoError,
}
