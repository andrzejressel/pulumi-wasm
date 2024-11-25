#[test]
fn test() {
    let content = pulumi_wasm_generator_lib::generate_example("ClIKUAoTZXhhbXBsZUxheWVyVmVyc2lvbhITZXhhbXBsZUxheWVyVmVyc2lvbhokYXdzOmxhbWJkYS9sYXllclZlcnNpb246TGF5ZXJWZXJzaW9uCo4BCosBCg9leGFtcGxlRnVuY3Rpb24SD2V4YW1wbGVGdW5jdGlvbhocYXdzOmxhbWJkYS9mdW5jdGlvbjpGdW5jdGlvbiJJCgZsYXllcnMSPyo9CjtCOQoTZXhhbXBsZUxheWVyVmVyc2lvbhIiChcaFQoTZXhhbXBsZUxheWVyVmVyc2lvbgoHCgUKA2FybhINCgNhd3MSBjYuMjguMg==".to_string()).unwrap();

    let syntax_tree = syn::parse_file(&content).unwrap();
    let formatted = prettyplease::unparse(&syntax_tree);

    assert_eq!(r#"
#version 330

in vec4 v_color;
out vec4 color;

void main() {
    color = v_color;
};
"#.trim(), formatted)
}
