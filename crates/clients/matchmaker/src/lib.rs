use std::{task::{Context, Poll}, sync::Arc};

use ethers::{signers::Signer, types::H256, utils::keccak256};
use types::{SendBundleResponse, BundleRequest};
use jsonrpsee::{http_client::{HttpClientBuilder, HttpClient, transport::HttpBackend}};
use jsonrpsee::core::client::ClientT;
use jsonrpsee::core::Error as RpcError;
use tower_http::set_header::SetRequestHeaderLayer;
use tower::{Service, ServiceExt, ServiceBuilder, Layer};
use http::{Request, header::{self, HeaderValue}, HeaderName};
use hyper::Body;
use std::time::Duration;
use futures_util::future::BoxFuture;
use futures_util::StreamExt;



pub mod types;


pub const MAINNET_URL: &str = "https://relay.flashbots.net";
pub const GOERLI_URL: &str = "https://relay-goerli.flashbots.net";


pub struct Client<S> {
    pub http_client: HttpClient<SigningService<S, HttpBackend>> ,
    pub signer: Arc<S>, 
}

#[derive(Clone)]
pub struct SigningLayer<S> {
    signer: Arc<S>,
}

impl<S> SigningLayer<S> {
    pub fn new(signer: Arc<S>) -> Self {
        SigningLayer { signer }
    }
}

impl<S: Clone, I> Layer<I> for SigningLayer<S> {
    type Service = SigningService<S, I>;

    fn layer(&self, inner: I) -> Self::Service {
        SigningService {
            signer: self.signer.clone(),
            inner,
        }
    }
}

// This service implements the Log behavior
#[derive(Clone)]
pub struct SigningService<S, I> {
    signer: Arc<S>,
    inner: I,
}

impl<S, I> Service<Request<Body>> for SigningService<S, I>
where
    I: Service<Request<Body>> + Clone + Send + 'static,
    I::Future: Send, 
    S: Signer + Clone + Send + 'static,
{
    type Response = I::Response;
    type Error = I::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request<Body>) -> Self::Future {

        let mut inner = self.inner.clone();
        let signer = self.signer.clone();

        let (mut parts, body) = request.into_parts();

        Box::pin(async move {

            let body_bytes = hyper::body::to_bytes(body).await.unwrap();

            let signature = signer
                .sign_message(format!(
                    "0x{:x}",
                    H256::from(keccak256(body_bytes.clone())))).await.unwrap();

            let header_name = HeaderName::from_static("x-flashbots-signature");
            let header_val = HeaderValue::from_str(&format!("{:?}:0x{}", signer.address(), signature)).unwrap();
            parts.headers.insert(header_name, header_val);

            let request = Request::from_parts(parts, Body::from(body_bytes.clone()));
            
            inner.call(request).await
        })

    }
}



impl<S: Signer + Clone + 'static> Client<S> {
    pub fn new(signer: Arc<S>) -> Self {

        let middleware = SigningLayer::new(signer.clone());

        let service_builder = ServiceBuilder::new().layer(middleware);

        // TODO: don't unwrap, handle mainnet vs goerli
        let http_client = HttpClientBuilder::default()
            .set_middleware(service_builder)
            .build(MAINNET_URL)
            .unwrap();

        Self { http_client, signer }
    }

    pub async fn send_bundle(
        &self,
        bundle: &BundleRequest,
    ) -> Result<SendBundleResponse, RpcError> {
        self.http_client.request("mev_sendBundle", [bundle]).await
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
