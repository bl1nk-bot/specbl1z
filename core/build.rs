use std::path::Path;

fn main() {
    let proto_file = "../schema/bl1nk.proto";

    // ตรวจสอบว่าไฟล์มีอยู่จริงเพื่อป้องกัน Panic ที่ไม่ชัดเจน
    if !Path::new(proto_file).exists() {
        panic!("Proto file not found at: {}", proto_file);
    }

    prost_build::compile_protos(&[proto_file], &["../schema/"]).unwrap();
}
