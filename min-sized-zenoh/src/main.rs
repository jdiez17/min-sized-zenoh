use deps::tokio::runtime::Builder;

async fn lol() {
    let session = deps::session().await;
    session.0.put("hello_world", "hello world! how are you").await.unwrap();
    println!("Hello, world!");
}

fn main() {
    let mut rt = Builder::new_multi_thread().build().unwrap();
    rt.block_on(lol());
}
