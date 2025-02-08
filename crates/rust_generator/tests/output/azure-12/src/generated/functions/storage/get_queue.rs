#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_queue {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetQueueArgs {
        /// A mapping of MetaData for this Queue.
        #[builder(into, default)]
        pub metadata: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the Queue.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Storage Account where the Queue exists.
        #[builder(into)]
        pub storage_account_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetQueueResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A mapping of MetaData for this Queue.
        pub metadata: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Resource Manager ID of this Storage Queue.
        pub resource_manager_id: pulumi_gestalt_rust::Output<String>,
        pub storage_account_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetQueueArgs,
    ) -> GetQueueResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let metadata_binding = args.metadata.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let storage_account_name_binding = args
            .storage_account_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:storage/getQueue:getQueue".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "metadata".into(),
                    value: &metadata_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountName".into(),
                    value: &storage_account_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetQueueResult {
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            metadata: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("metadata"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_manager_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceManagerId"),
            ),
            storage_account_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageAccountName"),
            ),
        }
    }
}
