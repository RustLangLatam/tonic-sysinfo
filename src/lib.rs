//! A `tonic` based gRPC system implementation.
//!
//! # Example
//!
//! An example can be found [here].
//!
//! [here]: https://github.com/RustLangLatam/tonic-sysinfo/blob/master/examples/server.rs

#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]
#![doc(
    html_logo_url = "https://avatars.githubusercontent.com/u/108101999?v=4"
)]
#![deny(rustdoc::broken_intra_doc_links)]
#![doc(html_root_url = "https://docs.rs/tonic-sysinfo/0.1.0")]
#![doc(issue_tracker_base_url = "https://github.com/RustLangLatam/tonic-sysinfo/issues/")]
#![doc(test(no_crate_inject, attr(deny(rust_2018_idioms))))]
#![cfg_attr(docsrs, feature(doc_cfg))]

/// Generated protobuf types from the `grpc.sysinfo.v1` package.
pub mod pb {
    #![allow(unreachable_pub)]
    #![allow(missing_docs)]
    include!("generated/grpc.sysinfo.v1.rs");

    /// Byte encoded FILE_DESCRIPTOR_SET.
    pub const FILE_DESCRIPTOR_SET: &[u8] = include_bytes!("generated/grpc_sysinfo_v1.bin");

    #[cfg(test)]
    mod tests {
        use super::FILE_DESCRIPTOR_SET;
        use prost::Message as _;

        #[test]
        fn file_descriptor_set_is_valid() {
            prost_types::FileDescriptorSet::decode(FILE_DESCRIPTOR_SET).unwrap();
        }
    }
}

pub mod server;
mod impls;
