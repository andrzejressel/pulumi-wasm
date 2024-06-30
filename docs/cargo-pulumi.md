## cargo pulumi

Command that compiles and combines Pulumi WASM components into a single WASM file. You can either run it from workspace
subdirectory using `cargo pulumi` or from root using `cargo pulumi -p <subproject>`. In both cases file named `composed.wasm`
will be created in `target/wasm32-wasip1/debug` directory.