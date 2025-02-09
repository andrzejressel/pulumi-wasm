#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_containers {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetContainersArgs {
        /// A prefix match used for the Storage Container `name` field.
        #[builder(into, default)]
        pub name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Storage Account that the Storage Containers reside in.
        #[builder(into)]
        pub storage_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetContainersResult {
        /// A `containers` block as defined below.
        pub containers: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::storage::GetContainersContainer>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name_prefix: pulumi_gestalt_rust::Output<Option<String>>,
        pub storage_account_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetContainersArgs,
    ) -> GetContainersResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_prefix_binding_1 = args.name_prefix.get_output(context);
        let name_prefix_binding = name_prefix_binding_1.get_inner();
        let storage_account_id_binding_1 = args.storage_account_id.get_output(context);
        let storage_account_id_binding = storage_account_id_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:storage/getContainers:getContainers".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "namePrefix".into(),
                    value: &name_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountId".into(),
                    value: &storage_account_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetContainersResult {
            containers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("containers"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            name_prefix: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("namePrefix"),
            ),
            storage_account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageAccountId"),
            ),
        }
    }
}
