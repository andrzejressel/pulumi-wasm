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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetServiceArgs,
    ) -> GetServiceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:domainservices/getService:getService".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetServiceResult {
            deployment_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deploymentId"),
            ),
            domain_configuration_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("domainConfigurationType"),
            ),
            domain_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("domainName"),
            ),
            filtered_sync_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filteredSyncEnabled"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            notifications: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("notifications"),
            ),
            replica_sets: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("replicaSets"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            resource_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceId"),
            ),
            secure_ldaps: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secureLdaps"),
            ),
            securities: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("securities"),
            ),
            sku: pulumi_gestalt_rust::__private::into_domain(o.extract_field("sku")),
            sync_owner: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("syncOwner"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tenant_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tenantId"),
            ),
            version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("version"),
            ),
        }
    }
}
