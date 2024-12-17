
#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct RubberTreeArgs {
    #[builder(into, default)]
    pub container: pulumi_wasm_rust::Output<Option<crate::types::Container>>,
    #[builder(into)]
    pub diameter: pulumi_wasm_rust::Output<crate::types::Diameter>,
    #[builder(into, default)]
    pub farm: pulumi_wasm_rust::Output<Option<pulumi_wasm_provider_common::OneOf2<crate::types::Farm, String>>>,
    #[builder(into, default)]
    pub size: pulumi_wasm_rust::Output<Option<crate::types::TreeSize>>,
    #[builder(into)]
    pub type_: pulumi_wasm_rust::Output<crate::types::RubberTreeVariety>,
}

pub struct RubberTreeResult {
    pub container: pulumi_wasm_rust::Output<Option<crate::types::Container>>,
    pub diameter: pulumi_wasm_rust::Output<crate::types::Diameter>,
    pub farm: pulumi_wasm_rust::Output<Option<pulumi_wasm_provider_common::OneOf2<crate::types::Farm, String>>>,
    pub size: pulumi_wasm_rust::Output<Option<crate::types::TreeSize>>,
    pub type_: pulumi_wasm_rust::Output<crate::types::RubberTreeVariety>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: RubberTreeArgs) -> RubberTreeResult {

    let result = crate::bindings::pulumi::plant::tree_rubber_tree::invoke(name, &crate::bindings::pulumi::plant::tree_rubber_tree::Args {
        container: &args.container.get_inner(),
        diameter: &args.diameter.get_inner(),
        farm: &args.farm.get_inner(),
        size: &args.size.get_inner(),
        type_: &args.type_.get_inner(),
    });

    RubberTreeResult {
        container: crate::into_domain(result.container),
        diameter: crate::into_domain(result.diameter),
        farm: crate::into_domain(result.farm),
        size: crate::into_domain(result.size),
        type_: crate::into_domain(result.type_),
    }
}
