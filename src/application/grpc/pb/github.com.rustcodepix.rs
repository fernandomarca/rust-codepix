#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PixKeyRegistration {
    #[prost(string, tag="1")]
    pub kind: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub account_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PixKey {
    #[prost(string, tag="1")]
    pub kind: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Account {
    #[prost(string, tag="1")]
    pub account_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub account_number: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub bank_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub bank_name: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub owner_name: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub created_at: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PixKeyInfo {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub kind: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub key: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub account: ::core::option::Option<Account>,
    #[prost(string, tag="5")]
    pub created_at: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PixKeyCreatedResult {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub status: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub error: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod pix_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct PixServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PixServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> PixServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Default + Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> PixServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            PixServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn register_pix_key(
            &mut self,
            request: impl tonic::IntoRequest<super::PixKeyRegistration>,
        ) -> Result<tonic::Response<super::PixKeyCreatedResult>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/github.com.rustcodepix.PixService/RegisterPixKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn find(
            &mut self,
            request: impl tonic::IntoRequest<super::PixKey>,
        ) -> Result<tonic::Response<super::PixKeyInfo>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/github.com.rustcodepix.PixService/Find",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
