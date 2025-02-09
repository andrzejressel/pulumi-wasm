#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServiceArgs {
        /// The display name for your managed Active Directory Domain Service resource. Changing this forces a new resource to be created.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group in which the Domain Service should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags assigned to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetServiceResult {
        /// A unique ID for the managed domain deployment.
        pub deployment_id: pulumi_gestalt_rust::Output<String>,
        /// The forest type used by the managed domain. One of `ResourceTrusting`, for a _Resource Forest_, or blank, for a _User Forest_.
        pub domain_configuration_type: pulumi_gestalt_rust::Output<String>,
        /// The Active Directory domain of the Domain Service. See [official documentation](https://docs.microsoft.com/azure/active-directory-domain-services/tutorial-create-instance#create-a-managed-domain) for constraints and recommendations.
        pub domain_name: pulumi_gestalt_rust::Output<String>,
        /// Whether group-based filtered sync (also called scoped synchronisation) is enabled.
        pub filtered_sync_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Azure location in which the replica set resides.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `notifications` block as defined below.
        pub notifications: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::domainservices::GetServiceNotification>,
        >,
        /// One or more `replica_set` blocks as defined below.
        pub replica_sets: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::domainservices::GetServiceReplicaSet>,
        >,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        pub resource_id: pulumi_gestalt_rust::Output<String>,
        /// A `secure_ldap` block as defined below.
        pub secure_ldaps: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::domainservices::GetServiceSecureLdap>,
        >,
        /// A `security` block as defined below.
        pub securities: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::domainservices::GetServiceSecurity>,
        >,
        /// The SKU of the Domain Service resource. One of `Standard`, `Enterprise` or `Premium`.
        pub sku: pulumi_gestalt_rust::Output<String>,
        pub sync_owner: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tenant_id: pulumi_gestalt_rust::Output<String>,
        pub version: pulumi_gestalt_rust::Output<i32>,
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
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:domainservices/getService:getService".into(),
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
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetServiceResult {
            deployment_id: o.get_field("deploymentId"),
            domain_configuration_type: o.get_field("domainConfigurationType"),
            domain_name: o.get_field("domainName"),
            filtered_sync_enabled: o.get_field("filteredSyncEnabled"),
            id: o.get_field("id"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            notifications: o.get_field("notifications"),
            replica_sets: o.get_field("replicaSets"),
            resource_group_name: o.get_field("resourceGroupName"),
            resource_id: o.get_field("resourceId"),
            secure_ldaps: o.get_field("secureLdaps"),
            securities: o.get_field("securities"),
            sku: o.get_field("sku"),
            sync_owner: o.get_field("syncOwner"),
            tags: o.get_field("tags"),
            tenant_id: o.get_field("tenantId"),
            version: o.get_field("version"),
        }
    }
}
