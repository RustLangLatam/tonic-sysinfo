# tonic-sysinfo

## Features

- transport: Provides the ability to set the service by using the type system and the
`NamedService` trait. You can use it like that:
```rust
    let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
    let client = HealthClient::new(conn);
```
