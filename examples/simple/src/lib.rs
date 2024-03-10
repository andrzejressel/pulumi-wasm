use anyhow::{Context, Error};
use log::{error, info};
use pulumi_rust_wasm::HASHMAP;
use crate::bindings::exports::component::pulumi_wasm::pulumi_main::Guest;

use pulumi_rust_wasm::output::Output;
use crate::bindings::component::pulumi_wasm::function_reverse_callback::{FunctionInvocationRequest, FunctionInvocationResult, get_functions, set_functions};
use crate::bindings::component::pulumi_wasm::output_interface::{combine_outputs, describe_outputs, non_done_exists};
use crate::random::*;

#[allow(clippy::all)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_unsafe)]
mod bindings;
bindings::export!(Component with_types_in bindings);

mod random;

struct Component {}

impl Guest for Component {
    fn main() {
        wasm_common::setup_logger();

        let length: Output<i32> = Output::new(&1).map(|i: i32| i * 3);

        let _v = create_random_string(RandomStringArgs {
            name: "test123456",
            length,
        });

        // let new_length = v.map(|s| s.len() as i32);
        //
        // let v = create_random_string(RandomStringArgs {
        //     name: "test123_2",
        //     length: new_length,
        // });

        run_loop().unwrap();

        info!("{}", describe_outputs());

        if non_done_exists() {
            error!("Non done exists");
            panic!("Non done exists");
        }
    }
}

fn run_loop() -> Result<(), Error> {
    loop {
        if !run_all_function()? && !combine_outputs() {
            return Ok(());
        }
    }
}

fn run_all_function(
    // store: &mut Store<SimplePluginCtx>,
    // plugin: &PulumiWasm,
) -> Result<bool, Error> {
    let functions = get_functions("source");

    if functions.is_empty() {
        info!("Functions are empty");
        return Ok(false)
    }

    info!("Functions are not empty");

    let functions_map = HASHMAP.lock().unwrap();

    let mapped: Result<Vec<_>, _> = functions
        .iter()
        .map(
            |FunctionInvocationRequest {
                 id,
                 function_id,
                 value,
             }| {
                info!("Invoking function [{function_id}] with value [{value:?}]");
                let v = rmpv::decode::read_value(&mut value.clone().as_slice())?;
                info!("Invoking function [{function_id}] with value [{v:?}]");
                let f = functions_map.get(function_id).context(format!("Function with id {function_id} not found"))?;
                Ok(FunctionInvocationResult {
                    id,
                    value: f(value.to_vec())?,
                })
            },
        )
        .collect();

    // mapped

    let mapped = match mapped {
        Ok(mapped) => mapped,
        Err(e) => {
            error!("Failed to invoke functions due to [{e}]");
            return Err(e);
        }
    };

    info!("Setting functions");
    set_functions(&mapped);
    info!("run_all_function completed");

    Ok(true)
}

