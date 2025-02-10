/// Manage an Azure Spring Cloud Application.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleSpringCloudService:
///     type: azure:appplatform:SpringCloudService
///     name: example
///     properties:
///       name: example-springcloud
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///   exampleSpringCloudApp:
///     type: azure:appplatform:SpringCloudApp
///     name: example
///     properties:
///       name: example-springcloudapp
///       resourceGroupName: ${example.name}
///       serviceName: ${exampleSpringCloudService.name}
///       identity:
///         type: SystemAssigned
/// ```
///
/// ## Import
///
/// Spring Cloud Application can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appplatform/springCloudApp:SpringCloudApp example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/myresourcegroup/providers/Microsoft.AppPlatform/spring/myservice/apps/myapp
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod spring_cloud_app {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpringCloudAppArgs {
        /// A JSON object that contains the addon configurations of the Spring Cloud Service.
        #[builder(into, default)]
        pub addon_json: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `custom_persistent_disk` block as defined below.
        #[builder(into, default)]
        pub custom_persistent_disks: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::appplatform::SpringCloudAppCustomPersistentDisk>,
            >,
        >,
        /// Is only HTTPS allowed? Defaults to `false`.
        #[builder(into, default)]
        pub https_only: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appplatform::SpringCloudAppIdentity>,
        >,
        /// An `ingress_settings` block as defined below.
        #[builder(into, default)]
        pub ingress_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appplatform::SpringCloudAppIngressSettings>,
        >,
        /// Does the Spring Cloud Application have public endpoint? Defaults to `false`.
        #[builder(into, default)]
        pub is_public: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the name of the Spring Cloud Application. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An `persistent_disk` block as defined below.
        #[builder(into, default)]
        pub persistent_disk: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appplatform::SpringCloudAppPersistentDisk>,
        >,
        /// Should the App in vnet injection instance exposes endpoint which could be accessed from Internet?
        #[builder(into, default)]
        pub public_endpoint_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the name of the resource group in which to create the Spring Cloud Application. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Spring Cloud Service resource. Changing this forces a new resource to be created.
        #[builder(into)]
        pub service_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Is End to End TLS Enabled? Defaults to `false`.
        #[builder(into, default)]
        pub tls_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct SpringCloudAppResult {
        /// A JSON object that contains the addon configurations of the Spring Cloud Service.
        pub addon_json: pulumi_gestalt_rust::Output<String>,
        /// A `custom_persistent_disk` block as defined below.
        pub custom_persistent_disks: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::types::appplatform::SpringCloudAppCustomPersistentDisk>,
            >,
        >,
        /// The Fully Qualified DNS Name of the Spring Application in the service.
        pub fqdn: pulumi_gestalt_rust::Output<String>,
        /// Is only HTTPS allowed? Defaults to `false`.
        pub https_only: pulumi_gestalt_rust::Output<Option<bool>>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::appplatform::SpringCloudAppIdentity>,
        >,
        /// An `ingress_settings` block as defined below.
        pub ingress_settings: pulumi_gestalt_rust::Output<
            super::super::types::appplatform::SpringCloudAppIngressSettings,
        >,
        /// Does the Spring Cloud Application have public endpoint? Defaults to `false`.
        pub is_public: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the name of the Spring Cloud Application. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// An `persistent_disk` block as defined below.
        pub persistent_disk: pulumi_gestalt_rust::Output<
            super::super::types::appplatform::SpringCloudAppPersistentDisk,
        >,
        /// Should the App in vnet injection instance exposes endpoint which could be accessed from Internet?
        pub public_endpoint_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the name of the resource group in which to create the Spring Cloud Application. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Spring Cloud Service resource. Changing this forces a new resource to be created.
        pub service_name: pulumi_gestalt_rust::Output<String>,
        /// Is End to End TLS Enabled? Defaults to `false`.
        pub tls_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The public endpoint of the Spring Cloud Application.
        pub url: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SpringCloudAppArgs,
    ) -> SpringCloudAppResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let addon_json_binding = args.addon_json.get_output(context);
        let custom_persistent_disks_binding = args
            .custom_persistent_disks
            .get_output(context);
        let https_only_binding = args.https_only.get_output(context);
        let identity_binding = args.identity.get_output(context);
        let ingress_settings_binding = args.ingress_settings.get_output(context);
        let is_public_binding = args.is_public.get_output(context);
        let name_binding = args.name.get_output(context);
        let persistent_disk_binding = args.persistent_disk.get_output(context);
        let public_endpoint_enabled_binding = args
            .public_endpoint_enabled
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let service_name_binding = args.service_name.get_output(context);
        let tls_enabled_binding = args.tls_enabled.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:appplatform/springCloudApp:SpringCloudApp".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "addonJson".into(),
                    value: addon_json_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customPersistentDisks".into(),
                    value: custom_persistent_disks_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "httpsOnly".into(),
                    value: https_only_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: identity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ingressSettings".into(),
                    value: ingress_settings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "isPublic".into(),
                    value: is_public_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "persistentDisk".into(),
                    value: persistent_disk_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicEndpointEnabled".into(),
                    value: public_endpoint_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceName".into(),
                    value: service_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tlsEnabled".into(),
                    value: tls_enabled_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SpringCloudAppResult {
            addon_json: o.get_field("addonJson"),
            custom_persistent_disks: o.get_field("customPersistentDisks"),
            fqdn: o.get_field("fqdn"),
            https_only: o.get_field("httpsOnly"),
            identity: o.get_field("identity"),
            ingress_settings: o.get_field("ingressSettings"),
            is_public: o.get_field("isPublic"),
            name: o.get_field("name"),
            persistent_disk: o.get_field("persistentDisk"),
            public_endpoint_enabled: o.get_field("publicEndpointEnabled"),
            resource_group_name: o.get_field("resourceGroupName"),
            service_name: o.get_field("serviceName"),
            tls_enabled: o.get_field("tlsEnabled"),
            url: o.get_field("url"),
        }
    }
}
