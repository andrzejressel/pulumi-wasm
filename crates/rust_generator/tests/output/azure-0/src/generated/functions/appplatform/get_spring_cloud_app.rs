#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_spring_cloud_app {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSpringCloudAppArgs {
        /// The name of the Spring Cloud Application.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Spring Cloud Application exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Spring Cloud Service.
        #[builder(into)]
        pub service_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetSpringCloudAppResult {
        /// The Fully Qualified DNS Name.
        pub fqdn: pulumi_gestalt_rust::Output<String>,
        /// Is only HTTPS allowed?
        pub https_only: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identities: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::appplatform::GetSpringCloudAppIdentity>,
        >,
        /// Does the Spring Cloud Application have public endpoint?
        pub is_public: pulumi_gestalt_rust::Output<bool>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `persistent_disk` block as defined below.
        pub persistent_disks: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::appplatform::GetSpringCloudAppPersistentDisk>,
        >,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        pub service_name: pulumi_gestalt_rust::Output<String>,
        /// Is End to End TLS Enabled?
        pub tls_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The public endpoint of the Spring Cloud Application.
        pub url: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetSpringCloudAppArgs,
    ) -> GetSpringCloudAppResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let service_name_binding = args.service_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:appplatform/getSpringCloudApp:getSpringCloudApp".into(),
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
                    name: "serviceName".into(),
                    value: service_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSpringCloudAppResult {
            fqdn: o.get_field("fqdn"),
            https_only: o.get_field("httpsOnly"),
            id: o.get_field("id"),
            identities: o.get_field("identities"),
            is_public: o.get_field("isPublic"),
            name: o.get_field("name"),
            persistent_disks: o.get_field("persistentDisks"),
            resource_group_name: o.get_field("resourceGroupName"),
            service_name: o.get_field("serviceName"),
            tls_enabled: o.get_field("tlsEnabled"),
            url: o.get_field("url"),
        }
    }
}
