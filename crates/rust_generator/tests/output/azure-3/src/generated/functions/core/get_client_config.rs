#[allow(clippy::doc_lazy_continuation)]
pub mod get_client_config {
    #[allow(dead_code)]
    pub struct GetClientConfigResult {
        /// is set to the Azure Client ID (Application Object ID).
        pub client_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// is set to the Azure Object ID.
        pub object_id: pulumi_gestalt_rust::Output<String>,
        /// is set to the Azure Subscription ID.
        pub subscription_id: pulumi_gestalt_rust::Output<String>,
        /// is set to the Azure Tenant ID.
        pub tenant_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
    ) -> GetClientConfigResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:core/getClientConfig:getClientConfig".into(),
            version: super::super::super::get_version(),
            object: Vec::from([]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetClientConfigResult {
            client_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clientId"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            object_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("objectId"),
            ),
            subscription_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subscriptionId"),
            ),
            tenant_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tenantId"),
            ),
        }
    }
}
