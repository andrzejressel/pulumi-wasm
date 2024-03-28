use crate::bindings::component::pulumi_wasm::function_reverse_callback::{
    get_functions, set_functions, FunctionInvocationRequest, FunctionInvocationResult,
};
use crate::bindings::component::pulumi_wasm::stack_interface::finish;
use crate::output::HASHMAP;
use anyhow::{Context, Error};
use log::{error, info};

pub fn run<F>(f: F) -> Result<(), Error>
where
    F: Fn() -> Result<(), Error>,
{
    let outer = || {
        wasm_common::setup_logger();
        f()?;
        run_loop()?;
        Ok(())
    };

    let result = outer();

    match result {
        Ok(()) => Ok(()),
        Err(e) => {
            error!("Error running pulumi wasm: [{e}]");
            Err(e)
        }
    }
}

fn run_loop() -> Result<(), Error> {
    loop {
        if !run_all_function()? && !finish() {
            return Ok(());
        }
    }
}

fn run_all_function(// store: &mut Store<SimplePluginCtx>,
    // plugin: &PulumiWasm,
) -> Result<bool, Error> {
    let functions = get_functions("source");

    if functions.is_empty() {
        info!("Functions are empty");
        return Ok(false);
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
                info!("Invoking function [{function_id}] with decoded value [{v:?}]");
                let f = functions_map
                    .get(function_id)
                    .context(format!("Function with id {function_id} not found"))?;
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
