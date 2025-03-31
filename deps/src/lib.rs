pub use zenoh::*;
pub mod tokio;

pub struct SessionHandle(pub zenoh::Session);

#[inline(never)]
pub async fn session() -> SessionHandle {
    SessionHandle(zenoh::open(zenoh::Config::default()).await.unwrap())
}