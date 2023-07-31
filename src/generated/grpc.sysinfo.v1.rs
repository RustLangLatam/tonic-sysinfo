/// MemInfo message contains information about memory and swap usage in the system.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MemInfo {
    /// Total memory in the system (in bytes).
    #[prost(uint64, tag = "1")]
    pub mem_total: u64,
    /// Free memory in the system (in bytes).
    #[prost(uint64, tag = "2")]
    pub mem_free: u64,
    /// Available memory in the system (in bytes).
    #[prost(uint64, tag = "3")]
    pub mem_available: u64,
    /// Total swap space in the system (in bytes).
    #[prost(uint64, tag = "4")]
    pub swap_total: u64,
    /// Free swap space in the system (in bytes).
    #[prost(uint64, tag = "5")]
    pub swap_free: u64,
}
/// CpuInfo message contains information about CPU usage and details in the system.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CpuInfo {
    /// Name of the CPU.
    #[prost(string, tag = "1")]
    pub cpu_name: ::prost::alloc::string::String,
    /// CPU usage as a floating-point value.
    #[prost(float, tag = "2")]
    pub cpu_usage: f32,
    /// CPU frequency (in Hz).
    #[prost(uint64, tag = "4")]
    pub frequency: u64,
    /// Vendor ID of the CPU.
    #[prost(string, tag = "5")]
    pub vendor_id: ::prost::alloc::string::String,
    /// Brand/model of the CPU.
    #[prost(string, tag = "6")]
    pub brand: ::prost::alloc::string::String,
}
/// Represents information about a disk or storage device in the system.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiskInfo {
    /// The name of the disk or storage device.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The type of file system used on the disk.
    #[prost(string, tag = "2")]
    pub file_system: ::prost::alloc::string::String,
    /// The mount point of the disk in the file system.
    #[prost(string, tag = "3")]
    pub mount_point: ::prost::alloc::string::String,
    /// The total available space on the disk, expressed in bytes.
    #[prost(uint64, tag = "4")]
    pub total_space: u64,
    /// The currently available space on the disk, expressed in bytes.
    #[prost(uint64, tag = "5")]
    pub available_space: u64,
}
/// SysInfoCheckRequest message is a request containing an InfoType enumeration that specifies the type of system information to retrieve.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SysInfoCheckRequest {
    /// List of InfoType values indicating the types of system information to retrieve.
    #[prost(enumeration = "sys_info_check_request::InfoType", repeated, tag = "1")]
    pub info_type: ::prost::alloc::vec::Vec<i32>,
}
/// Nested message and enum types in `SysInfoCheckRequest`.
pub mod sys_info_check_request {
    /// Enumeration of possible system information types.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum InfoType {
        MemInfo = 0,
        CpuInfo = 1,
        DiskInfo = 2,
    }
    impl InfoType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                InfoType::MemInfo => "MEM_INFO",
                InfoType::CpuInfo => "CPU_INFO",
                InfoType::DiskInfo => "DISK_INFO",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MEM_INFO" => Some(Self::MemInfo),
                "CPU_INFO" => Some(Self::CpuInfo),
                "DISK_INFO" => Some(Self::DiskInfo),
                _ => None,
            }
        }
    }
}
/// SysInfoCheckResponse message is a response containing a System message with system information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SysInfoCheckResponse {
    /// System message containing the requested system information.
    #[prost(message, optional, tag = "1")]
    pub info: ::core::option::Option<sys_info_check_response::System>,
}
/// Nested message and enum types in `SysInfoCheckResponse`.
pub mod sys_info_check_response {
    /// Nested System message containing system information.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct System {
        /// Memory information in the system.
        #[prost(message, optional, tag = "1")]
        pub mem_info: ::core::option::Option<super::MemInfo>,
        /// List of CPU information in the system.
        #[prost(message, repeated, tag = "2")]
        pub cpu_info: ::prost::alloc::vec::Vec<super::CpuInfo>,
        /// List of Disk information in the system.
        #[prost(message, repeated, tag = "3")]
        pub disk_info: ::prost::alloc::vec::Vec<super::DiskInfo>,
    }
}
/// Generated client implementations.
pub mod sys_info_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// SysInfoService is an RPC service that defines two methods:
    /// 1. Check: A unary RPC method that takes a SysInfoCheckRequest and returns a SysInfoCheckResponse. Used to perform a single check for system information.
    /// 2. Watch: A server-streaming RPC method that takes a SysInfoCheckRequest and returns a stream of SysInfoCheckResponse messages. Used to observe real-time changes in system information.
    #[derive(Debug, Clone)]
    pub struct SysInfoServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SysInfoServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
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
        ) -> SysInfoServiceClient<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + Send + Sync,
        {
            SysInfoServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Check method performs a single check for system information.
        pub async fn check(
            &mut self,
            request: impl tonic::IntoRequest<super::SysInfoCheckRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SysInfoCheckResponse>,
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
                "/grpc.sysinfo.v1.SysInfoService/Check",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("grpc.sysinfo.v1.SysInfoService", "Check"));
            self.inner.unary(req, path, codec).await
        }
        /// Watch method returns a stream of system information for real-time observation of changes.
        pub async fn watch(
            &mut self,
            request: impl tonic::IntoRequest<super::SysInfoCheckRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::SysInfoCheckResponse>>,
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
                "/grpc.sysinfo.v1.SysInfoService/Watch",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("grpc.sysinfo.v1.SysInfoService", "Watch"));
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod sys_info_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with SysInfoServiceServer.
    #[async_trait]
    pub trait SysInfoService: Send + Sync + 'static {
        /// Check method performs a single check for system information.
        async fn check(
            &self,
            request: tonic::Request<super::SysInfoCheckRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SysInfoCheckResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the Watch method.
        type WatchStream: futures_core::Stream<
                Item = std::result::Result<super::SysInfoCheckResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Watch method returns a stream of system information for real-time observation of changes.
        async fn watch(
            &self,
            request: tonic::Request<super::SysInfoCheckRequest>,
        ) -> std::result::Result<tonic::Response<Self::WatchStream>, tonic::Status>;
    }
    /// SysInfoService is an RPC service that defines two methods:
    /// 1. Check: A unary RPC method that takes a SysInfoCheckRequest and returns a SysInfoCheckResponse. Used to perform a single check for system information.
    /// 2. Watch: A server-streaming RPC method that takes a SysInfoCheckRequest and returns a stream of SysInfoCheckResponse messages. Used to observe real-time changes in system information.
    #[derive(Debug)]
    pub struct SysInfoServiceServer<T: SysInfoService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: SysInfoService> SysInfoServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for SysInfoServiceServer<T>
    where
        T: SysInfoService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
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
            let inner = self.inner.clone();
            match req.uri().path() {
                "/grpc.sysinfo.v1.SysInfoService/Check" => {
                    #[allow(non_camel_case_types)]
                    struct CheckSvc<T: SysInfoService>(pub Arc<T>);
                    impl<
                        T: SysInfoService,
                    > tonic::server::UnaryService<super::SysInfoCheckRequest>
                    for CheckSvc<T> {
                        type Response = super::SysInfoCheckResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SysInfoCheckRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).check(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CheckSvc(inner);
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
                "/grpc.sysinfo.v1.SysInfoService/Watch" => {
                    #[allow(non_camel_case_types)]
                    struct WatchSvc<T: SysInfoService>(pub Arc<T>);
                    impl<
                        T: SysInfoService,
                    > tonic::server::ServerStreamingService<super::SysInfoCheckRequest>
                    for WatchSvc<T> {
                        type Response = super::SysInfoCheckResponse;
                        type ResponseStream = T::WatchStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SysInfoCheckRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).watch(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = WatchSvc(inner);
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
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: SysInfoService> Clone for SysInfoServiceServer<T> {
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
    impl<T: SysInfoService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: SysInfoService> tonic::server::NamedService for SysInfoServiceServer<T> {
        const NAME: &'static str = "grpc.sysinfo.v1.SysInfoService";
    }
}
