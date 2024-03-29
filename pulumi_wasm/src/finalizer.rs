use crate::bindings::component::pulumi_wasm::external_world;
use crate::bindings::component::pulumi_wasm::external_world::{get_root_resource, is_in_preview};
use crate::output::{access_map, output_map, OutputContent};
use crate::{grpc, output};
use log::info;
use prost::Message;
use rmpv::Value;
use std::ops::Deref;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;

static STACK_OUTPUT_SENT: AtomicBool = AtomicBool::new(false);

pub(crate) fn finish() -> bool {
    invoke_exporting_if_needed();
    combine_outputs()
}

fn invoke_exporting_if_needed() {
    if STACK_OUTPUT_SENT.swap(true, Relaxed) {
        return;
    }
    let outputs = output_map();

    if is_in_preview() {
        info!("Skipping invoke_exporting because we are in preview");
    } else if outputs.is_empty() {
        info!("Skipping invoke_exporting because there are no outputs")
    } else {
        info!("Invoking exporting");
        let outputs = output_map();
        let outputs = outputs.iter().collect::<Vec<_>>();
        let names = outputs
            .iter()
            .map(|(name, _)| name.to_string())
            .collect::<Vec<_>>();

        output::map_internal(
            outputs.iter().map(|(_, o)| o.output.clone()).collect(),
            move |values| {
                let names = &names;
                let object = crate::Component::create_protobuf_struct(names, &values);
                info!("Resulting object: [{object:?}]");

                let root = get_root_resource();
                info!("Root resource: [{root}]");

                let request = grpc::RegisterResourceOutputsRequest {
                    urn: root,
                    outputs: Some(object),
                };
                info!("Request: [{request:?}]");
                external_world::register_resource_outputs(request.encode_to_vec().as_slice());
                Value::Nil
            },
        );
    }
}

fn combine_outputs() -> bool {
    wasm_common::setup_logger();
    let outputs = access_map();
    let mut changed = false;

    outputs.iter().for_each(|o| {
        let ref_cell = o.borrow();
        let content = ref_cell.deref();

        let new_value = match content {
            OutputContent::Func(refcells, f) => {
                info!("Found func");
                let data = refcells
                    .iter()
                    .flat_map(|r| {
                        let ref_cell = r.borrow();
                        let content = ref_cell.deref();
                        match content {
                            OutputContent::Done(v) => Some(v.clone()),
                            OutputContent::Mapped(_, _, _)
                            | OutputContent::Func(_, _)
                            | OutputContent::Nothing => None,
                        }
                    })
                    .collect::<Vec<_>>();

                if data.len() == refcells.len() {
                    info!("Map");
                    Some(f(data))
                } else {
                    info!("Cannot map");
                    None
                }
            }
            OutputContent::Done(_) => None,
            OutputContent::Mapped(_, _, _) => None,
            OutputContent::Nothing => None,
        };

        drop(ref_cell);
        match new_value {
            None => {}
            Some(i) => {
                changed = true;
                o.replace(OutputContent::Done(i));
            }
        };
    });

    changed
}
