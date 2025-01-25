use crate::output::HASHMAP;
use crate::PulumiContext;
use anyhow::{Context, Error, Result};
use log::{error, info};
use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::stack_interface::{
    finish, FunctionInvocationRequest, FunctionInvocationResult,
};

pub fn run<F>(in_preview_u8: u8, f: F) -> Result<(), Error>
where
    F: Fn(&PulumiContext) -> Result<(), Error>,
{
    let in_preview = in_preview_u8 == 1;
    let engine = PulumiContext::new(in_preview);
    let outer = |e: &PulumiContext| {
        pulumi_wasm_common::setup_logger();
        f(&engine)?;
        run_loop(&engine)?;
        Ok(())
    };

    let result = outer(&engine);

    match result {
        Ok(()) => Ok(()),
        Err(e) => {
            error!("Error running pulumi wasm: [{e}]");
            Err(e)
        }
    }
}

fn run_loop(engine: &PulumiContext) -> Result<(), Error> {
    run_all_function(engine)
}

fn run_all_function(engine: &PulumiContext) -> Result<(), Error> {
    let mut functions = finish(&engine.wit_engine, &[]);

    loop {
        if functions.is_empty() {
            return Ok(());
        }
        let mapped = map_functions(&functions)?;
        functions = finish(&engine.wit_engine, &mapped);
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
                let f = functions_map
                    .get(function_id)
                    .context(format!("Function with id {function_id} not found"))?;
                Ok(FunctionInvocationResult {
                    id,
                    value: f(value)?,
                })
            },
        )
        .collect()
}
