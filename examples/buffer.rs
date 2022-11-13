use cronet_rs::Buffer;

fn main() {
  let buffer = Buffer::new();
  buffer.initWithAlloc(10);
  println!("Buffer size: {}", buffer.size());
  buffer.destroy();
}
