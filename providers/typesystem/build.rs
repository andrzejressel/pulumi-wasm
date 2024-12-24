use pulumi_wasm_generator;
fn main() {
    pulumi_wasm_generator::generate_from_schema("typesystem.json".as_ref());
}