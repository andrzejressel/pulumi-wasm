use crate::bindings::component::pulumi_wasm::stack_interface::{
    finish, FunctionInvocationRequest, FunctionInvocationResult,
};
use crate::output::HASHMAP;
use anyhow::{Context, Error, Result};
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
    run_all_function()
}

fn run_all_function() -> Result<(), Error> {
    let mut functions = finish(&[]);

    loop {
        if functions.is_empty() {
            return Ok(());
        }
        let mapped = map_functions(&functions)?;
        functions = finish(&mapped);
    }
}

fn map_functions(functions: &[FunctionInvocationRequest]) -> Result<Vec<FunctionInvocationResult>> {
    let functions_map = HASHMAP.lock().unwrap();

    functions
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
        .collect()
}
