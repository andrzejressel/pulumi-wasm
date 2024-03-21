use std::process::Command;
use assert_cmd::prelude::*; // Add methods on commands

use anyhow::{Error};
use fs_extra::dir::CopyOptions;
use wasmtime::component::{Component, Linker, ResourceTable};
use wasmtime::Store;
use wasmtime_wasi::WasiCtx;
use wasmtime_wasi::WasiCtxBuilder;
use wasmtime_wasi::WasiView;
use crate::server::Runner;

mod server {
    wasmtime::component::bindgen!({
        path: "tests/fixtures/example/service.wit",
        world: "runner"
    });
}

#[test]
fn errors_out_when_cargo_toml_not_available() -> Result<(), Error> {
    let s = Command::cargo_bin("cargo-pulumi")?
        .current_dir("tests/fixtures/no_project")
        .assert()
        .failure()
        .to_string();

    assert!(s.contains("Command run in directory that is not cargo project"), "Output was: {}", s);
    Ok(())
}

#[test]
fn errors_out_in_invalid_package() -> Result<(), Error> {
    let dir = testdir::testdir!();
    fs_extra::dir::copy("tests/fixtures/example", &dir, &CopyOptions::new())?;

    let s = Command::cargo_bin("cargo-pulumi")?
        .args(["-p", "invalid_package"])
        .current_dir(dir.join("example"))
        .assert()
        .failure()
        .to_string();

    assert!(s.contains("Cannot find package [invalid_package] in workspace"), "Output was: {}", s);
    Ok(())
}

#[test]
fn run_from_subdirectory() -> Result<(), Error> {
    let dir = testdir::testdir!();
    fs_extra::dir::copy("tests/fixtures/example", &dir, &CopyOptions::new())?;

    Command::new("cargo")
        .args(["component", "build", "-p", "common", "-p", "provider_a", "-p", "main"])
        .current_dir(dir.join("example"))
        .assert()
        .success();

    Command::cargo_bin("cargo-pulumi")?
        .current_dir(dir.join("example").join("main"))
        .assert()
        .success();

    let (mut store, plugin) = create_engine(dir.join("example/target/wasm32-wasi/debug/composed.wasm").to_str().unwrap())?;
    let result = plugin.example_service_main_interface().call_main(&mut store)?;

    assert_eq!(result, "Hello from main: [Hello from provider-a-lib: Hello from provider-a: Hello from common-lib: run_common] [Hello from common-lib: run_common]".to_string());

    Ok(())
}

#[test]
fn run_from_main_directory() -> Result<(), Error> {
    let dir = testdir::testdir!();
    fs_extra::dir::copy("tests/fixtures/example", &dir, &CopyOptions::new())?;

    Command::new("cargo")
        .args(["component", "build", "-p", "common", "-p", "provider_a", "-p", "main"])
        .current_dir(dir.join("example"))
        .assert()
        .success();

    Command::cargo_bin("cargo-pulumi")?
        .args(["-p", "main"])
        .current_dir(dir.join("example"))
        .assert()
        .success();

    let (mut store, plugin) = create_engine(dir.join("example/target/wasm32-wasi/debug/composed.wasm").to_str().unwrap())?;
    let result = plugin.example_service_main_interface().call_main(&mut store)?;

    assert_eq!(result, "Hello from main: [Hello from provider-a-lib: Hello from provider-a: Hello from common-lib: run_common] [Hello from common-lib: run_common]".to_string());

    Ok(())
}

struct SimplePluginCtx {
    table: ResourceTable,
    context: WasiCtx,
}

impl WasiView for SimplePluginCtx {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.context
    }
}

fn create_engine(file: &str) -> Result<(Store<SimplePluginCtx>, Runner), Error> {
    let mut engine_config = wasmtime::Config::new();
    engine_config.wasm_component_model(true);
    engine_config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
    engine_config.debug_info(true);

    let engine = wasmtime::Engine::new(&engine_config).unwrap();

    let mut linker: Linker<SimplePluginCtx> = Linker::new(&engine);

    wasmtime_wasi::command::sync::add_to_linker(&mut linker).unwrap();

    let table = ResourceTable::new();

    let wasi_ctx = WasiCtxBuilder::new()
        .inherit_stdin()
        .inherit_stdout()
        .build();

    let mut store = Store::new(
        &engine,
        SimplePluginCtx {
            table,
            context: wasi_ctx
        },
    );

    let component = Component::from_file(&engine, file)?;

    let (plugin, _instance) = Runner::instantiate(&mut store, &component, &linker)?;
    Ok((store, plugin))
}
