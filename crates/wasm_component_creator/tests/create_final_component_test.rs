use anyhow::Result;
use async_trait::async_trait;
use pulumi_gestalt_wasm_component_creator::source::WasmComponentSource;
use wac_graph::types::Package;
use wac_graph::CompositionGraph;
use wit_component::{dummy_module, embed_component_metadata, ComponentEncoder, StringEncoding};
use wit_parser::ManglingAndAbi::Standard32;
use wit_parser::{PackageId, Resolve};

#[tokio::test]
async fn should_combine_wasm_components() -> Result<()> {
    let mut resolve = Resolve::new();
    resolve.add_pulumi_gestalt_stable().unwrap();
    resolve.add_pulumi_gestalt("0.0.0-DEV").unwrap();

    let pkg = resolve
        .push_str(
            "test.wit",
            r#"
    package test:wit;

    world root {
        import component:pulumi-gestalt/output-interface@0.0.0-DEV;
        export component:pulumi-gestalt-external/pulumi-main@0.0.0-STABLE-DEV;
        import component:pulumi-gestalt-external/pulumi-settings@0.0.0-STABLE-DEV;
    }
"#,
        )
        .unwrap();

    let world = resolve.select_world(pkg, None).unwrap();

    let mut module = dummy_module(&resolve, world, Standard32);

    embed_component_metadata(&mut module, &resolve, world, StringEncoding::UTF8).unwrap();

    let encoded = ComponentEncoder::default()
        .module(&module)
        .unwrap()
        .validate(true)
        .encode()
        .unwrap();

    let result =
        pulumi_gestalt_wasm_component_creator::create(&TestProgramSource {}, encoded.clone(), true)
            .await
            .unwrap();

    assert_component_only_exports_main_and_settings(&result)?;

    Ok(())
}

#[tokio::test]
async fn return_error_when_multiple_versions_of_pulumi_gestalt_is_found() -> Result<()> {
    let mut resolve = Resolve::new();
    resolve.add_pulumi_gestalt_stable().unwrap();
    resolve.add_pulumi_gestalt("0.0.0-DEV").unwrap();
    resolve.add_pulumi_gestalt("0.0.1-DEV").unwrap();

    let pkg = resolve
        .push_str(
            "test.wit",
            r#"
    package test:wit;

    world root {
        import component:pulumi-gestalt/output-interface@0.0.0-DEV;
        import component:pulumi-gestalt/output-interface@0.0.1-DEV;
        export component:pulumi-gestalt-external/pulumi-main@0.0.0-STABLE-DEV;
    }
"#,
        )
        .unwrap();

    let world = resolve.select_world(pkg, None).unwrap();

    let mut module = dummy_module(&resolve, world, Standard32);

    embed_component_metadata(&mut module, &resolve, world, StringEncoding::UTF8).unwrap();

    let encoded = ComponentEncoder::default()
        .module(&module)
        .unwrap()
        .validate(true)
        .encode()
        .unwrap();

    let error =
        pulumi_gestalt_wasm_component_creator::create(&TestProgramSource {}, encoded.clone(), true)
            .await
            .expect_err("Expected creator to return error");

    assert_eq!(
        error.to_string(),
        "Found multiple Pulumi-Wasm versions: 0.0.0-DEV, 0.0.1-DEV. Ensure only one is used."
            .to_string()
    );

    Ok(())
}

#[tokio::test]
async fn return_error_when_multiple_versions_of_pulumi_gestalt_in_providers_is_found() -> Result<()>
{
    let mut resolve = Resolve::new();
    resolve.add_pulumi_gestalt_stable().unwrap();
    resolve.add_pulumi_gestalt("0.0.0-DEV").unwrap();
    resolve.add_pulumi_gestalt("0.0.1-DEV").unwrap();
    resolve
        .add_provider("docker", "1.0.0", "0.0.0-DEV")
        .unwrap();
    resolve
        .add_provider("cloudflare", "1.0.0", "0.0.1-DEV")
        .unwrap();

    let pkg = resolve
        .push_str(
            "test.wit",
            r#"
    package test:wit;

    world root {
        import component:pulumi-gestalt/output-interface@0.0.0-DEV;
        export component:pulumi-gestalt-external/pulumi-main@0.0.0-STABLE-DEV;
        import pulumi:cloudflare/container@1.0.0--0.0.1-DEV;
        import pulumi:docker/container@1.0.0--0.0.0-DEV;
    }
"#,
        )
        .unwrap();

    let world = resolve.select_world(pkg, None).unwrap();

    let mut module = dummy_module(&resolve, world, Standard32);

    embed_component_metadata(&mut module, &resolve, world, StringEncoding::UTF8).unwrap();

    let encoded = ComponentEncoder::default()
        .module(&module)
        .unwrap()
        .validate(true)
        .encode()
        .unwrap();

    let error =
        pulumi_gestalt_wasm_component_creator::create(&TestProgramSource {}, encoded.clone(), true)
            .await
            .expect_err("Expected creator to return error");

    assert_eq!(
        error.to_string(),
        "Found multiple Pulumi-Wasm versions: 0.0.0-DEV, 0.0.1-DEV. Ensure only one is used."
            .to_string()
    );

    Ok(())
}

fn assert_component_only_exports_main_and_settings(result: &[u8]) -> Result<()> {
    let mut graph = CompositionGraph::new();
    let main = Package::from_bytes("main", None, result, graph.types_mut()).unwrap();
    let main_package_id = graph.register_package(main.clone()).unwrap();

    let exports_names: Vec<String> = graph.types()[graph[main_package_id].ty()]
        .exports
        .iter()
        .map(|(name, _)| name.clone())
        .collect();

    assert_eq!(
        exports_names,
        vec!["component:pulumi-gestalt-external/pulumi-main@0.0.0-STABLE-DEV".to_string()]
    );

    let imports_names: Vec<_> = graph.types()[graph[main_package_id].ty()]
        .imports
        .iter()
        .map(|(name, _)| name.clone())
        .collect();

    assert_eq!(imports_names, Vec::<String>::new());
    Ok(())
}

struct TestProgramSource {}

#[async_trait]
impl WasmComponentSource for TestProgramSource {
    async fn get(&self, version: &str, _debug: bool) -> Result<Vec<u8>> {
        let mut resolve = Resolve::new();
        resolve.add_pulumi_gestalt_stable().unwrap();
        let pkg = resolve.add_pulumi_gestalt(version).unwrap();

        let world = resolve.select_world(pkg, None).unwrap();

        let mut module = dummy_module(&resolve, world, Standard32);

        embed_component_metadata(&mut module, &resolve, world, StringEncoding::UTF8).unwrap();

        let encoded = ComponentEncoder::default()
            .module(&module)
            .unwrap()
            .validate(true)
            .encode()
            .unwrap();

        Ok(encoded)
    }
}

trait ResolveExt {
    fn add_provider(
        &mut self,
        provider_name: impl Into<String>,
        provider_version: impl Into<String>,
        pulumi_gestalt_version: impl Into<String>,
    ) -> Result<PackageId>;
    fn add_pulumi_gestalt(
        &mut self,
        pulumi_gestalt_version: impl Into<String>,
    ) -> Result<PackageId>;
    fn add_pulumi_gestalt_stable(&mut self) -> Result<()>;
}

impl ResolveExt for Resolve {
    fn add_provider(
        &mut self,
        provider_name: impl Into<String>,
        provider_version: impl Into<String>,
        pulumi_gestalt_version: impl Into<String>,
    ) -> Result<PackageId> {
        let provider_name = provider_name.into();
        let provider_version = provider_version.into();
        let pulumi_gestalt_version = pulumi_gestalt_version.into();
        self.push_str(
            format!("{provider_name}-{provider_version}-{pulumi_gestalt_version}.wit"),
            format!(
                r#"
    package pulumi:{provider_name}@{provider_version}--{pulumi_gestalt_version};

    world root {{
        export container;
    }}

    interface container {{
        use component:pulumi-gestalt/output-interface@{pulumi_gestalt_version}.{{output}};
        test: func();
    }}
"#
            )
            .as_str(),
        )
    }

    fn add_pulumi_gestalt(
        &mut self,
        pulumi_gestalt_version: impl Into<String>,
    ) -> Result<PackageId> {
        let pulumi_gestalt_version = pulumi_gestalt_version.into();
        self.push_str(
            format!("pulumi-gestalt-{pulumi_gestalt_version}.wit"),
            format!(
                r#"
    package component:pulumi-gestalt@{pulumi_gestalt_version};

    world root {{
        export component:pulumi-gestalt-external/pulumi-settings@0.0.0-STABLE-DEV;
        export output-interface;
    }}

    interface pulumi-main {{
        main: func();
    }}

    interface output-interface {{

        resource output {{
            constructor(value: string);
            map: func(function-name: string) -> output;
            duplicate: func() -> output;
        }}
        combine: func(outputs: list<output>) -> output;
    }}
"#
            )
            .as_str(),
        )
    }

    fn add_pulumi_gestalt_stable(&mut self) -> Result<()> {
        self.push_str(
            "pulumi-gestalt-stable.wit",
            r#"
    package component:pulumi-gestalt-external@0.0.0-STABLE-DEV;

    interface pulumi-main {
        main: func();
    }

    interface pulumi-settings {
        set-in-preview: func(in-preview: bool);
    }

"#,
        )?;

        Ok(())
    }
}
