use protobuf::reflect::MessageDescriptor;
use protobuf::reflect::OneofDescriptor;
use protobuf_codegen::Codegen;
use protobuf_codegen::Customize;
use protobuf_codegen::CustomizeCallback;

fn main() {
    struct GenSerde;

    impl CustomizeCallback for GenSerde {
        fn message(&self, _message: &MessageDescriptor) -> Customize {
            Customize::default().before("#[derive(::serde::Deserialize)]")
        }

        fn special_field(&self, _message: &MessageDescriptor, _field: &str) -> Customize {
            Customize::default().before("#[serde(skip)]")
        }

        fn oneof(&self, _oneof: &OneofDescriptor) -> Customize {
            Customize::default().before("#[derive(::serde::Deserialize)]")
        }
    }

    Codegen::new()
        .pure()
        .cargo_out_dir("protos")
        .include("src")
        .input("src/example.proto")
        .customize_callback(GenSerde)
        .run_from_script();
}
