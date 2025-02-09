#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServiceArgs {
        /// The Name of the Search Service.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Search Service exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetServiceResult {
        /// Describes whether the search service is compliant or not with respect to having non-customer encrypted resources. If a service has more than one non-customer encrypted resource and `Enforcement` is `enabled` then the service will be marked as `NonCompliant`. If all the resources are customer encrypted, then the service will be marked as `Compliant`.
        pub customer_managed_key_encryption_compliance_status: pulumi_gestalt_rust::Output<
            String,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identities: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::search::GetServiceIdentity>,
        >,
        /// The name of this Query Key.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The number of partitions which have been created.
        pub partition_count: pulumi_gestalt_rust::Output<i32>,
        /// The Primary Key used for Search Service Administration.
        pub primary_key: pulumi_gestalt_rust::Output<String>,
        /// Whether or not public network access is enabled for this resource.
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<bool>,
        /// A `query_keys` block as defined below.
        pub query_keys: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::search::GetServiceQueryKey>,
        >,
        /// The number of replica's which have been created.
        pub replica_count: pulumi_gestalt_rust::Output<i32>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The Secondary Key used for Search Service Administration.
        pub secondary_key: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetServiceArgs,
    ) -> GetServiceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:search/getService:getService".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetServiceResult {
            customer_managed_key_encryption_compliance_status: o
                .get_field("customerManagedKeyEncryptionComplianceStatus"),
            id: o.get_field("id"),
            identities: o.get_field("identities"),
            name: o.get_field("name"),
            partition_count: o.get_field("partitionCount"),
            primary_key: o.get_field("primaryKey"),
            public_network_access_enabled: o.get_field("publicNetworkAccessEnabled"),
            query_keys: o.get_field("queryKeys"),
            replica_count: o.get_field("replicaCount"),
            resource_group_name: o.get_field("resourceGroupName"),
            secondary_key: o.get_field("secondaryKey"),
            tags: o.get_field("tags"),
        }
    }
}
