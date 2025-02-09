#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_ontap_storage_virtual_machines {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOntapStorageVirtualMachinesArgs {
        /// Configuration block. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::super::types::fsx::GetOntapStorageVirtualMachinesFilter,
                >,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct GetOntapStorageVirtualMachinesResult {
        pub filters: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::super::types::fsx::GetOntapStorageVirtualMachinesFilter,
                >,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// List of all SVM IDs found.
        pub ids: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetOntapStorageVirtualMachinesArgs,
    ) -> GetOntapStorageVirtualMachinesResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filters_binding = args.filters.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:fsx/getOntapStorageVirtualMachines:getOntapStorageVirtualMachines"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: filters_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetOntapStorageVirtualMachinesResult {
            filters: o.get_field("filters"),
            id: o.get_field("id"),
            ids: o.get_field("ids"),
        }
    }
}
