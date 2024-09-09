// This file is @generated by prost-build.
/// Memory information in the system.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct MemoryInfo {
    /// Total memory in the system (in bytes)
    #[prost(uint32, tag = "1")]
    pub total: u32,
    /// Free memory in the system (in bytes)
    #[prost(uint32, tag = "2")]
    pub free: u32,
    /// Available memory in the system (in bytes)
    #[prost(uint32, tag = "3")]
    pub available: u32,
    /// Total swap space in the system (in bytes)
    #[prost(uint32, tag = "4")]
    pub swap_total: u32,
    /// Free swap space in the system (in bytes)
    #[prost(uint32, tag = "5")]
    pub swap_free: u32,
}
/// CPU information in the system.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CpuInfo {
    /// Name of the CPU
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// CPU usage as a floating-point value
    #[prost(float, tag = "2")]
    pub usage: f32,
    /// CPU frequency (in Hz)
    #[prost(uint64, tag = "3")]
    pub frequency: u64,
    /// Vendor ID of the CPU
    #[prost(string, tag = "4")]
    pub vendor_id: ::prost::alloc::string::String,
    /// Brand/model of the CPU
    #[prost(string, tag = "5")]
    pub brand: ::prost::alloc::string::String,
}
/// Disk information in the system.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiskInfo {
    /// Name of the disk or storage device
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Type of file system used on the disk
    #[prost(string, tag = "2")]
    pub file_system: ::prost::alloc::string::String,
    /// Mount point of the disk in the file system
    #[prost(string, tag = "3")]
    pub mount_point: ::prost::alloc::string::String,
    /// Total available space on the disk (in bytes)
    #[prost(uint32, tag = "4")]
    pub total_space: u32,
    /// Currently available space on the disk (in bytes)
    #[prost(uint32, tag = "5")]
    pub available_space: u32,
}
/// System information request
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct SystemInfoRequest {
    #[prost(bool, tag = "1")]
    pub include_memory_info: bool,
    #[prost(bool, tag = "2")]
    pub include_cpu_info: bool,
    #[prost(bool, tag = "3")]
    pub include_disk_info: bool,
    /// Interval to execute a refresh (in seconds)
    #[prost(uint32, tag = "4")]
    pub refresh_interval: u32,
}
/// System information response
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemInfoResponse {
    #[prost(message, optional, tag = "1")]
    pub memory_info: ::core::option::Option<MemoryInfo>,
    #[prost(message, repeated, tag = "2")]
    pub cpu_info: ::prost::alloc::vec::Vec<CpuInfo>,
    #[prost(message, repeated, tag = "3")]
    pub disk_info: ::prost::alloc::vec::Vec<DiskInfo>,
}
/// Generated client implementations.
pub mod system_info_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// System information service
    #[derive(Debug, Clone)]
    pub struct SystemInfoServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SystemInfoServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SystemInfoServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            SystemInfoServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn get_system_info(
            &mut self,
            request: impl tonic::IntoRequest<super::SystemInfoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SystemInfoResponse>,
            tonic::Status,
        > {
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
                "/grpc.sysinfo.v1.SystemInfoService/GetSystemInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("grpc.sysinfo.v1.SystemInfoService", "GetSystemInfo"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn watch_system_info(
            &mut self,
            request: impl tonic::IntoRequest<super::SystemInfoRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::SystemInfoResponse>>,
            tonic::Status,
        > {
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
                "/grpc.sysinfo.v1.SystemInfoService/WatchSystemInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "grpc.sysinfo.v1.SystemInfoService",
                        "WatchSystemInfo",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod system_info_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with SystemInfoServiceServer.
    #[async_trait]
    pub trait SystemInfoService: std::marker::Send + std::marker::Sync + 'static {
        async fn get_system_info(
            &self,
            request: tonic::Request<super::SystemInfoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SystemInfoResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the WatchSystemInfo method.
        type WatchSystemInfoStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::SystemInfoResponse, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn watch_system_info(
            &self,
            request: tonic::Request<super::SystemInfoRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::WatchSystemInfoStream>,
            tonic::Status,
        >;
    }
    /// System information service
    #[derive(Debug)]
    pub struct SystemInfoServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> SystemInfoServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for SystemInfoServiceServer<T>
    where
        T: SystemInfoService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/grpc.sysinfo.v1.SystemInfoService/GetSystemInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetSystemInfoSvc<T: SystemInfoService>(pub Arc<T>);
                    impl<
                        T: SystemInfoService,
                    > tonic::server::UnaryService<super::SystemInfoRequest>
                    for GetSystemInfoSvc<T> {
                        type Response = super::SystemInfoResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SystemInfoRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SystemInfoService>::get_system_info(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetSystemInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/grpc.sysinfo.v1.SystemInfoService/WatchSystemInfo" => {
                    #[allow(non_camel_case_types)]
                    struct WatchSystemInfoSvc<T: SystemInfoService>(pub Arc<T>);
                    impl<
                        T: SystemInfoService,
                    > tonic::server::ServerStreamingService<super::SystemInfoRequest>
                    for WatchSystemInfoSvc<T> {
                        type Response = super::SystemInfoResponse;
                        type ResponseStream = T::WatchSystemInfoStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SystemInfoRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SystemInfoService>::watch_system_info(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = WatchSystemInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", tonic::Code::Unimplemented as i32)
                                .header(
                                    http::header::CONTENT_TYPE,
                                    tonic::metadata::GRPC_CONTENT_TYPE,
                                )
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T> Clone for SystemInfoServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "grpc.sysinfo.v1.SystemInfoService";
    impl<T> tonic::server::NamedService for SystemInfoServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
