use std::path::PathBuf;

fn main() {
    let proto_file = "./proto/relx.proto";
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .file_descriptor_set_path(out_dir.join("relx.bin"))
        .out_dir("./src/service")
        .compile(&[proto_file], &["proto"])
        .unwrap();
}
