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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetSpringCloudAppArgs,
    ) -> GetSpringCloudAppResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let service_name_binding = args.service_name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:appplatform/getSpringCloudApp:getSpringCloudApp".into(),
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
                    name: "serviceName".into(),
                    value: &service_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetSpringCloudAppResult {
            fqdn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("fqdn")),
            https_only: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("httpsOnly"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            identities: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identities"),
            ),
            is_public: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("isPublic"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            persistent_disks: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("persistentDisks"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            service_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceName"),
            ),
            tls_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tlsEnabled"),
            ),
            url: pulumi_gestalt_rust::__private::into_domain(o.extract_field("url")),
        }
    }
}
