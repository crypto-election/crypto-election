extern crate exonum_build;

use exonum_build::ProtobufGenerator;

fn main() {
    ProtobufGenerator::with_mod_name("protobuf_mod.rs")
        .with_input_dir("../proto")
        .with_crypto()
        .generate();
}
