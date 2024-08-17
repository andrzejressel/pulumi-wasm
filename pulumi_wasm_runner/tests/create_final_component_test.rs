// use wit_component;

use anyhow::Result;
use std::fs;
use wit_component::{dummy_module, embed_component_metadata, ComponentEncoder, StringEncoding};
use wit_parser::Resolve;

#[test]
fn test() -> Result<()> {
    let mut resolve = Resolve::default();

    // For inputs which have a single package and only one world `None`
    // can be specified.
    let id = resolve.push_str(
        "./my-test.wit",
        r#"
                package example:wit1;

                world foo {
                    import output-interface;
                }

                interface output-interface {

                    resource output {
                        constructor(value: string);
                        map: func(function-name: string) -> output;
                        duplicate: func() -> output;
                    }
                    combine: func(outputs: list<output>) -> output;
                }
            "#,
    )?;

    let world_id = resolve.select_world(id, "foo".into())?;

    let vec = dummy_module(&resolve, world_id);

    fs::write("test.wasm", vec)?;

    Ok(())
}

#[test]
fn abcabc() {
    let mut resolve = Resolve::new();
    resolve
        .push_str("pulumi-wasm.wit",
                  r#"

package component:pulumi-wasm@0.0.0-DEV;


        interface output-interface {

    resource output {
        constructor(value: string);
        map: func(function-name: string) -> output;
        duplicate: func() -> output;
    }
    combine: func(outputs: list<output>) -> output;
}


        "#).unwrap();

    resolve.push_str("docker.wit",
                     r#"

                     package pulumi:docker@4.5.3-ZERO.ZERO.ZERO-DEV;

                     interface container {
                        use component:pulumi-wasm/output-interface@0.0.0-DEV.{output};
                     }

                     "#,
    ).unwrap();

    let pkg = resolve.push_str(
        "test.wit",
        r#"
package test:wit;

world root {
    import component:pulumi-wasm/output-interface@0.0.0-DEV;
    import pulumi:docker/container@4.5.3-ZERO.ZERO.ZERO-DEV;
}
"#,
    )
        .unwrap();
    let world = resolve.select_world(pkg, None).unwrap();

    let mut module = dummy_module(&resolve, world);

    embed_component_metadata(&mut module, &resolve, world, StringEncoding::UTF8).unwrap();

    let encoded = ComponentEncoder::default()
        .module(&module)
        .unwrap()
        .validate(true)
        .encode()
        .unwrap();

    let wat = wasmprinter::print_bytes(&encoded).unwrap();
    // assert!(wat.contains("unlocked-dep=<foo:bar/foo@{>=1.0.0 <1.1.0}>"));
    // assert!(wat.contains("locked-dep=<foo:bar/i@1.2.3>"));
    // fs::write("test2.wasm", &encoded).unwrap();

    println!("{}", wat);
}

fn generate_pulumi_wasm_component() {}

fn generate_component() {}