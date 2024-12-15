//! Another failing example. A list of SSIS object metadata.
//! API Version: 2018-06-01.

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetIntegrationRuntimeObjectMetadatumArgs {
    /// The factory name.
    #[builder(into)]
    pub factory_name: pulumi_wasm_rust::Output<String>,
    /// The integration runtime name.
    #[builder(into)]
    pub integration_runtime_name: pulumi_wasm_rust::Output<String>,
    /// Metadata path.
    #[builder(into, default)]
    pub metadata_path: pulumi_wasm_rust::Output<Option<String>>,
    /// The resource group name.
    #[builder(into)]
    pub resource_group_name: pulumi_wasm_rust::Output<String>,
}

pub struct GetIntegrationRuntimeObjectMetadatumResult {
    /// The link to the next page of results, if any remaining results exist.
    pub next_link: pulumi_wasm_rust::Output<Option<String>>,
    /// List of SSIS object metadata.
    pub value: pulumi_wasm_rust::Output<Option<Vec<pulumi_wasm_provider_common::OneOf4<crate::types::SsisEnvironmentResponse, crate::types::SsisFolderResponse, crate::types::SsisPackageResponse, crate::types::SsisProjectResponse>>>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: GetIntegrationRuntimeObjectMetadatumArgs
) -> GetIntegrationRuntimeObjectMetadatumResult {

    let result = crate::bindings::pulumi::mypkg::get_integration_runtime_object_metadatum::invoke(
        &crate::bindings::pulumi::mypkg::get_integration_runtime_object_metadatum::Args {
                factory_name: &args.factory_name.get_inner(),
                integration_runtime_name: &args.integration_runtime_name.get_inner(),
                metadata_path: &args.metadata_path.get_inner(),
                resource_group_name: &args.resource_group_name.get_inner(),
        }
    );

    GetIntegrationRuntimeObjectMetadatumResult {
        next_link: crate::into_domain(result.next_link),
        value: crate::into_domain(result.value),
    }
}
