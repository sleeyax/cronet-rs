use cronet_rs::{Buffer, Destroy};

fn main() {
    let buffer = Buffer::new();
    buffer.init_size(10);
    println!("Buffer size: {}", buffer.size());
    buffer.destroy();
}
