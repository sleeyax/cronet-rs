use cronet_rs::client::{Body, Client};

fn main() {
    let client = Client::new();

    println!("sending GET request...");
    let request = http::Request::builder()
        .method("GET")
        .uri("https://httpbin.org/anything")
        .body(Body::default())
        .unwrap();
    let result = client.send(request);
    print_result(result);

    println!("sending POST request...");
    let request = http::Request::builder()
        .method("POST")
        .uri("https://httpbin.org/anything")
        .header("content-type", "application/x-www-form-urlencoded")
        .body(Body::from("Hello, world"))
        .unwrap();
    let result = client.send(request);
    print_result(result);
}

fn print_result(result: Result<http::Response<Body>, cronet_rs::client::ClientError>) {
    match result {
        Ok(response) => {
            println!("Status: {}", response.status());
            println!("Headers: {:#?}", response.headers());
            let body = response.body().as_bytes().unwrap();
            println!("Body: {}", String::from_utf8_lossy(body));
        }
        Err(error) => println!("Error: {:?}", error),
    }
}
