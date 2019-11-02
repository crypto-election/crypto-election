extern crate exonum_build;

use exonum_build::{get_exonum_protobuf_files_path, protobuf_generate};

fn main() {
    let exonum_protos = get_exonum_protobuf_files_path();
    let proto_path = "src/proto".to_owned();
    protobuf_generate(
        &proto_path,
        &[&proto_path, &exonum_protos],
        "protobuf_mod.rs",
    );
}
