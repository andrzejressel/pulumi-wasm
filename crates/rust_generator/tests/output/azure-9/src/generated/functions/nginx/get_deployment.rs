#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_deployment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDeploymentArgs {
        /// The name of this NGINX Deployment.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the NGINX Deployment exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetDeploymentResult {
        /// An `auto_scale_profile` block as defined below.
        pub auto_scale_profiles: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::nginx::GetDeploymentAutoScaleProfile>,
        >,
        /// The automatic upgrade channel for this NGINX deployment.
        pub automatic_upgrade_channel: pulumi_gestalt_rust::Output<String>,
        /// The number of NGINX capacity units for this NGINX Deployment.
        pub capacity: pulumi_gestalt_rust::Output<i32>,
        /// Whether metrics are exported to Azure Monitor.
        pub diagnose_support_enabled: pulumi_gestalt_rust::Output<bool>,
        /// Preferred email associated with the NGINX Deployment.
        pub email: pulumi_gestalt_rust::Output<String>,
        /// A `frontend_private` block as defined below.
        pub frontend_privates: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::nginx::GetDeploymentFrontendPrivate>,
        >,
        /// A `frontend_public` block as defined below.
        pub frontend_publics: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::nginx::GetDeploymentFrontendPublic>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A `identity` block as defined below.
        pub identities: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::nginx::GetDeploymentIdentity>,
        >,
        /// The list of Public IP Resource IDs for this NGINX Deployment.
        pub ip_address: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the NGINX Deployment exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// A `logging_storage_account` block as defined below.
        pub logging_storage_accounts: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::nginx::GetDeploymentLoggingStorageAccount>,
        >,
        pub managed_resource_group: pulumi_gestalt_rust::Output<String>,
        /// Name of the autoscaling profile.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `network_interface` block as defined below.
        pub network_interfaces: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::nginx::GetDeploymentNetworkInterface>,
        >,
        /// NGINX version of the Deployment.
        pub nginx_version: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The NGINX Deployment SKU.
        pub sku: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags assigned to the NGINX Deployment.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetDeploymentArgs,
    ) -> GetDeploymentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:nginx/getDeployment:getDeployment".into(),
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
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDeploymentResult {
            auto_scale_profiles: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("autoScaleProfiles"),
            ),
            automatic_upgrade_channel: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("automaticUpgradeChannel"),
            ),
            capacity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("capacity"),
            ),
            diagnose_support_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("diagnoseSupportEnabled"),
            ),
            email: pulumi_gestalt_rust::__private::into_domain(o.extract_field("email")),
            frontend_privates: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("frontendPrivates"),
            ),
            frontend_publics: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("frontendPublics"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            identities: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identities"),
            ),
            ip_address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipAddress"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            logging_storage_accounts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("loggingStorageAccounts"),
            ),
            managed_resource_group: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("managedResourceGroup"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            network_interfaces: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkInterfaces"),
            ),
            nginx_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nginxVersion"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            sku: pulumi_gestalt_rust::__private::into_domain(o.extract_field("sku")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
