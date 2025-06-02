use tonic_build::compile_protos;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match compile_protos("proto/chat/chat.proto") {
        Ok(()) => Ok(()),
        Err(error) => Err(error.into()),
    }
}
