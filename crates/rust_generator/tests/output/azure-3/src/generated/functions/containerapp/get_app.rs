#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_app {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAppArgs {
        /// The name of the Container App.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where this Container App exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetAppResult {
        /// The ID of the Container App Environment this Container App is linked to.
        pub container_app_environment_id: pulumi_gestalt_rust::Output<String>,
        pub custom_domain_verification_id: pulumi_gestalt_rust::Output<String>,
        /// A `dapr` block as detailed below.
        pub daprs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::containerapp::GetAppDapr>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Resource ID for the User Assigned Managed identity to use when pulling from the Container Registry.
        pub identities: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::containerapp::GetAppIdentity>,
        >,
        /// An `ingress` block as detailed below.
        pub ingresses: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::containerapp::GetAppIngress>,
        >,
        pub latest_revision_fqdn: pulumi_gestalt_rust::Output<String>,
        pub latest_revision_name: pulumi_gestalt_rust::Output<String>,
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The max inactive revisions for this Container App.
        pub max_inactive_revisions: pulumi_gestalt_rust::Output<i32>,
        /// Name for the IP restriction rule.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub outbound_ip_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A `registry` block as detailed below.
        pub registries: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::containerapp::GetAppRegistry>,
        >,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The revision mode of the Container App.
        pub revision_mode: pulumi_gestalt_rust::Output<String>,
        /// One or more `secret` block as detailed below.
        pub secrets: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::containerapp::GetAppSecret>,
        >,
        /// A mapping of tags to assign to the Container App.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// A `template` block as detailed below.
        pub templates: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::containerapp::GetAppTemplate>,
        >,
        /// The name of the Workload Profile in the Container App Environment in which this Container App is running.
        pub workload_profile_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetAppArgs,
    ) -> GetAppResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:containerapp/getApp:getApp".into(),
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
        GetAppResult {
            container_app_environment_id: o.get_field("containerAppEnvironmentId"),
            custom_domain_verification_id: o.get_field("customDomainVerificationId"),
            daprs: o.get_field("daprs"),
            id: o.get_field("id"),
            identities: o.get_field("identities"),
            ingresses: o.get_field("ingresses"),
            latest_revision_fqdn: o.get_field("latestRevisionFqdn"),
            latest_revision_name: o.get_field("latestRevisionName"),
            location: o.get_field("location"),
            max_inactive_revisions: o.get_field("maxInactiveRevisions"),
            name: o.get_field("name"),
            outbound_ip_addresses: o.get_field("outboundIpAddresses"),
            registries: o.get_field("registries"),
            resource_group_name: o.get_field("resourceGroupName"),
            revision_mode: o.get_field("revisionMode"),
            secrets: o.get_field("secrets"),
            tags: o.get_field("tags"),
            templates: o.get_field("templates"),
            workload_profile_name: o.get_field("workloadProfileName"),
        }
    }
}
