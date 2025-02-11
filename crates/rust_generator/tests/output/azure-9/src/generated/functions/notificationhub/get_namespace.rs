#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_namespace {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNamespaceArgs {
        /// Specifies the Name of the Notification Hub Namespace.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the Name of the Resource Group within which the Notification Hub exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetNamespaceResult {
        /// Is this Notification Hub Namespace enabled?
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region in which this Notification Hub Namespace exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the SKU to use for this Notification Hub Namespace. Possible values are `Free`, `Basic` or `Standard.`
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Type of Namespace, such as `Messaging` or `NotificationHub`.
        pub namespace_type: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        pub servicebus_endpoint: pulumi_gestalt_rust::Output<String>,
        /// A `sku` block as defined below.
        pub sku: pulumi_gestalt_rust::Output<
            super::super::super::types::notificationhub::GetNamespaceSku,
        >,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetNamespaceArgs,
    ) -> GetNamespaceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:notificationhub/getNamespace:getNamespace".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetNamespaceResult {
            enabled: o.get_field("enabled"),
            id: o.get_field("id"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            namespace_type: o.get_field("namespaceType"),
            resource_group_name: o.get_field("resourceGroupName"),
            servicebus_endpoint: o.get_field("servicebusEndpoint"),
            sku: o.get_field("sku"),
            tags: o.get_field("tags"),
        }
    }
}
