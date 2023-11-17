use cronet_rs::client::{Body, Client};

fn main() {
    let client = Client::new();
    let request = http::Request::builder()
        .method("GET")
        .uri("https://httpbin.org/anything")
        .body(Body::default())
        .unwrap();
    let result = client.send(request);
    println!("{:?}", result);
}
