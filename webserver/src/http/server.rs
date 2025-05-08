use galvyn::core::{DatabaseSetup, GalvynRouter, SchemalessJson};
use galvyn::error::GalvynError;
use galvyn::openapi::OpenAPI;
use galvyn::rorm::Database;
use galvyn::{Galvyn, get};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub async fn start() -> Result<(), GalvynError> {
    Galvyn::new()
        .register_module::<Database>(DatabaseSetup::default())
        .init_modules()
        .await?
        .add_routes(GalvynRouter::new().nest("/api/v1", GalvynRouter::new().handler(get_openapi)))
        .start(SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 8080))
        .await?;
    Ok(())
}

#[get("/openapi.json")]
async fn get_openapi() -> SchemalessJson<&'static OpenAPI> {
    SchemalessJson(galvyn::openapi::get_openapi())
}
