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

#[test]
fn test2() {
    let s = "Cu4CCusCCgZmb29iYXISBmZvb2JhchoiY2xvdWRmbGFyZTppbmRleC9wYWdlUnVsZTpQYWdlUnVsZSI2CgZ6b25lSWQSLEIqChBjbG91ZGZsYXJlWm9uZUlkEhYKFBoSChBjbG91ZGZsYXJlWm9uZUlkIksKBnRhcmdldBJBEj8KCAoGEgRzdWIuCihCJgoOY2xvdWRmbGFyZVpvbmUSFAoSGhAKDmNsb3VkZmxhcmVab25lCgkKBxIFL3BhZ2UiFwoIcHJpb3JpdHkSCwoJGQAAAAAAAPA/IpYBCgdhY3Rpb25zEooBIocBCkwKCG1pbmlmaWVzEkAqPgo8IjoKEAoCanMSChIICgYKBBICb24KEwoEaHRtbBILEgkKBwoFEgNvZmYKEQoDY3NzEgoSCAoGCgQSAm9uChcKA3NzbBIQEg4KDAoKEghmbGV4aWJsZQoeChBlbWFpbE9iZnVzY2F0aW9uEgoSCAoGCgQSAm9uEhQKCmNsb3VkZmxhcmUSBjUuNDMuMQ==";
    let content = pulumi_wasm_generator_lib::generate_example(s.to_string()).unwrap();
    println!("{}", content);
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