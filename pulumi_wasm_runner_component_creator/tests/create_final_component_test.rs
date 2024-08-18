use anyhow::Result;
use async_trait::async_trait;
use pulumi_wasm_runner_component_creator::source::{DefaultProviderSource, PulumiWasmSource};
use std::collections::BTreeMap;
use wac_graph::types::Package;
use wac_graph::CompositionGraph;
use wit_component::{dummy_module, embed_component_metadata, ComponentEncoder, StringEncoding};
use wit_parser::{PackageId, Resolve};

#[tokio::test]
async fn should_combine_wasm_components() -> Result<()> {
    let mut resolve = Resolve::new();
    resolve.add_pulumi_wasm("0.0.0-DEV").unwrap();
    resolve
        .add_provider("docker", "4.5.3", "0.0.0-DEV")
        .unwrap();

    let pkg = resolve
        .push_str(
            "test.wit",
            r#"
    package test:wit;

    world root {
        import component:pulumi-wasm/output-interface@0.0.0-DEV;
        export component:pulumi-wasm/pulumi-main@0.0.0-DEV;
        import pulumi:docker/container@4.5.3-DIVIDER-0.0.0-DEV;
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

    let result = pulumi_wasm_runner_component_creator::create(
        BTreeMap::<String, Box<_>>::new(),
        &TestDefaultProviderSource {},
        &TestProgramSource {},
        encoded.clone(),
    )
    .await
    .unwrap();

    assert_component_only_exports_main(&result)?;

    Ok(())
}

#[tokio::test]
async fn return_error_when_multiple_dependencies_on_the_same_provider_is_found() -> Result<()> {
    let mut resolve = Resolve::new();
    resolve.add_pulumi_wasm("0.0.0-DEV").unwrap();
    resolve
        .add_provider("docker", "4.5.3", "0.0.0-DEV")
        .unwrap();
    resolve
        .add_provider("docker", "4.5.4", "0.0.0-DEV")
        .unwrap();

    let pkg = resolve
        .push_str(
            "test.wit",
            r#"
    package test:wit;

    world root {
        import component:pulumi-wasm/output-interface@0.0.0-DEV;
        export component:pulumi-wasm/pulumi-main@0.0.0-DEV;
        import pulumi:docker/container@4.5.3-DIVIDER-0.0.0-DEV;
        import pulumi:docker/container@4.5.4-DIVIDER-0.0.0-DEV;
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

    let error = pulumi_wasm_runner_component_creator::create(
        BTreeMap::new(),
        &TestDefaultProviderSource {},
        &TestProgramSource {},
        encoded.clone(),
    )
    .await
    .expect_err("Expected creator to return error");

    assert_eq!(error.to_string(), "Provider \"docker\" is requested in multiple versions:\n- 4.5.3 that requires pulumi_wasm in version 0.0.0-DEV\n- 4.5.4 that requires pulumi_wasm in version 0.0.0-DEV".to_string());

    Ok(())
}

#[tokio::test]
async fn return_error_when_multiple_versions_of_pulumi_wasm_is_found() -> Result<()> {
    let mut resolve = Resolve::new();
    resolve.add_pulumi_wasm("0.0.0-DEV").unwrap();
    resolve.add_pulumi_wasm("0.0.1-DEV").unwrap();

    let pkg = resolve
        .push_str(
            "test.wit",
            r#"
    package test:wit;

    world root {
        import component:pulumi-wasm/output-interface@0.0.0-DEV;
        import component:pulumi-wasm/output-interface@0.0.1-DEV;
        export component:pulumi-wasm/pulumi-main@0.0.0-DEV;
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

    let error = pulumi_wasm_runner_component_creator::create(
        BTreeMap::new(),
        &TestDefaultProviderSource {},
        &TestProgramSource {},
        encoded.clone(),
    )
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
async fn return_error_when_multiple_versions_of_pulumi_wasm_in_providers_is_found() -> Result<()> {
    let mut resolve = Resolve::new();
    resolve.add_pulumi_wasm("0.0.0-DEV").unwrap();
    resolve.add_pulumi_wasm("0.0.1-DEV").unwrap();
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
        import component:pulumi-wasm/output-interface@0.0.0-DEV;
        export component:pulumi-wasm/pulumi-main@0.0.0-DEV;
        import pulumi:cloudflare/container@1.0.0-DIVIDER-0.0.1-DEV;
        import pulumi:docker/container@1.0.0-DIVIDER-0.0.0-DEV;
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

    let error = pulumi_wasm_runner_component_creator::create(
        BTreeMap::new(),
        &TestDefaultProviderSource {},
        &TestProgramSource {},
        encoded.clone(),
    )
    .await
    .expect_err("Expected creator to return error");

    assert_eq!(
        error.to_string(),
        "Found multiple Pulumi-Wasm versions: 0.0.0-DEV, 0.0.1-DEV. Ensure only one is used."
            .to_string()
    );

    Ok(())
}

fn assert_component_only_exports_main(result: &Vec<u8>) -> Result<()> {
    let mut graph = CompositionGraph::new();
    let main = Package::from_bytes("main", None, result.clone(), graph.types_mut()).unwrap();
    let main_package_id = graph.register_package(main.clone()).unwrap();

    let exports_names: Vec<String> = graph.types()[graph[main_package_id].ty()]
        .exports
        .iter()
        .map(|(name, _)| name.clone())
        .collect();

    assert_eq!(
        exports_names,
        vec!["component:pulumi-wasm/pulumi-main@0.0.0-DEV".to_string()]
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
impl PulumiWasmSource for TestProgramSource {
    async fn get(&self, version: &String) -> Result<Vec<u8>> {
        let mut resolve = Resolve::new();
        let pkg = resolve.add_pulumi_wasm(version).unwrap();

        let world = resolve.select_world(pkg, None).unwrap();

        let mut module = dummy_module(&resolve, world);

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

struct TestDefaultProviderSource {}

#[async_trait]
impl DefaultProviderSource for TestDefaultProviderSource {
    async fn get_component(
        &self,
        provider_name: &String,
        provider_version: &String,
        pulumi_wasm_version: &String,
    ) -> Result<Vec<u8>> {
        let mut resolve = Resolve::new();
        resolve.add_pulumi_wasm(pulumi_wasm_version).unwrap();
        let pkg = resolve
            .add_provider(provider_name, provider_version, pulumi_wasm_version)
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

        Ok(encoded)
    }
}

trait ResolveExt {
    fn add_provider(
        &mut self,
        provider_name: impl Into<String>,
        provider_version: impl Into<String>,
        pulumi_wasm_version: impl Into<String>,
    ) -> Result<PackageId>;
    fn add_pulumi_wasm(&mut self, pulumi_wasm_version: impl Into<String>) -> Result<PackageId>;
}

impl ResolveExt for Resolve {
    fn add_provider(
        &mut self,
        provider_name: impl Into<String>,
        provider_version: impl Into<String>,
        pulumi_wasm_version: impl Into<String>,
    ) -> Result<PackageId> {
        let provider_name = provider_name.into();
        let provider_version = provider_version.into();
        let pulumi_wasm_version = pulumi_wasm_version.into();
        self.push_str(
            format!("{provider_name}-{provider_version}-{pulumi_wasm_version}.wit"),
            format!(
                r#"
    package pulumi:{provider_name}@{provider_version}-DIVIDER-{pulumi_wasm_version};

    world root {{
        export container;
    }}

    interface container {{
        use component:pulumi-wasm/output-interface@{pulumi_wasm_version}.{{output}};
        test: func();
    }}
"#
            )
            .as_str(),
        )
    }

    fn add_pulumi_wasm(&mut self, pulumi_wasm_version: impl Into<String>) -> Result<PackageId> {
        let pulumi_wasm_version = pulumi_wasm_version.into();
        self.push_str(
            format!("pulumi-wasm-{pulumi_wasm_version}.wit"),
            format!(
                r#"
    package component:pulumi-wasm@{pulumi_wasm_version};

    world root {{
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
}
