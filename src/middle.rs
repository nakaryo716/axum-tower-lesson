use std::{convert::Infallible, future::Future, pin::Pin};

use axum::{extract::Request, response::Response};
use http::StatusCode;
use tower::{Layer, Service};

#[derive(Debug, Clone)]
pub struct AppService<S> {
    inner: S
}

impl<S> AppService<S> {
    pub fn new(inner: S) -> Self {
        Self {
            inner,
        }
    }
}

impl<S> Service<Request> for AppService<S> 
where
    S: Service<Request, Error = Infallible>,
{
    type Response = Response<String>;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        println!("called");
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let url = req.uri().to_string();
        let res = if url == "/" {
            let response = Response::builder()
                .status(StatusCode::OK)
                .body("Hello application root page".to_string())
                .unwrap();
    
            Ok(response)
        } else {
            let response = Response::builder()
                .status(StatusCode::OK)
                .body("Hello other".to_string())
                .unwrap();
            Ok(response)
        };

        Box::pin(async{res})
    }
}

#[derive(Clone)]
pub struct AppLayer;

impl<S> Layer<S> for AppLayer {
    type Service = AppService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AppService::new(inner)
    }
}