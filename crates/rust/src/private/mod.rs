pub mod constant;
pub mod output;
pub mod runner;

pub extern crate bon;
pub extern crate pulumi_gestalt_wit;
pub extern crate serde;
pub extern crate wit_bindgen;

use crate::Output;
use pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::output_interface::Output as WitOutput;

pub fn into_domain<F: serde::Serialize>(output: WitOutput) -> Output<F> {
    unsafe { Output::<F>::new_from_handle(output) }
}
