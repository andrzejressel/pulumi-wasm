/// Manages an IoT Central Application
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resource
///       location: West Europe
///   exampleApplication:
///     type: azure:iotcentral:Application
///     name: example
///     properties:
///       name: example-iotcentral-app
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       subDomain: example-iotcentral-app-subdomain
///       displayName: example-iotcentral-app-display-name
///       sku: ST1
///       template: iotc-default@1.0.0
///       tags:
///         Foo: Bar
/// ```
///
/// ## Import
///
/// The IoT Central Application can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:iotcentral/application:Application example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.IoTCentral/iotApps/app1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod application {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationArgs {
        /// A `display_name` name. Custom display name for the IoT Central application. Default is resource name.
        ///
        /// > **NOTE:** Due to a bug in the provider, the default value of `display_name` of a newly created IoT Central App will be the Resource Group Name, it will be fixed and use resource name in 4.0. For an existing IoT Central App, this could be fixed by specifying the `display_name` explicitly.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::iotcentral::ApplicationIdentity>,
        >,
        /// Specifies the supported Azure location where the resource has to be create. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the IotHub resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether public network access is allowed for the IoT Central Application. Defaults to `true`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The name of the resource group under which the IotHub resource has to be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `sku` name. Possible values is `ST0`, `ST1`, `ST2`, Default value is `ST1`
        #[builder(into, default)]
        pub sku: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `sub_domain` name. Subdomain for the IoT Central URL. Each application must have a unique subdomain.
        #[builder(into)]
        pub sub_domain: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `template` name. IoT Central application template name. Defaults to `iotc-pnp-preview@1.0.0`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub template: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ApplicationResult {
        /// A `display_name` name. Custom display name for the IoT Central application. Default is resource name.
        ///
        /// > **NOTE:** Due to a bug in the provider, the default value of `display_name` of a newly created IoT Central App will be the Resource Group Name, it will be fixed and use resource name in 4.0. For an existing IoT Central App, this could be fixed by specifying the `display_name` explicitly.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::iotcentral::ApplicationIdentity>,
        >,
        /// Specifies the supported Azure location where the resource has to be create. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the IotHub resource. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Whether public network access is allowed for the IoT Central Application. Defaults to `true`.
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the resource group under which the IotHub resource has to be created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `sku` name. Possible values is `ST0`, `ST1`, `ST2`, Default value is `ST1`
        pub sku: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `sub_domain` name. Subdomain for the IoT Central URL. Each application must have a unique subdomain.
        pub sub_domain: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `template` name. IoT Central application template name. Defaults to `iotc-pnp-preview@1.0.0`. Changing this forces a new resource to be created.
        pub template: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApplicationArgs,
    ) -> ApplicationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let display_name_binding = args.display_name.get_output(context);
        let identity_binding = args.identity.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let sku_binding = args.sku.get_output(context);
        let sub_domain_binding = args.sub_domain.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let template_binding = args.template.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:iotcentral/application:Application".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: identity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: public_network_access_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sku".into(),
                    value: sku_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subDomain".into(),
                    value: sub_domain_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "template".into(),
                    value: template_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ApplicationResult {
            display_name: o.get_field("displayName"),
            identity: o.get_field("identity"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            public_network_access_enabled: o.get_field("publicNetworkAccessEnabled"),
            resource_group_name: o.get_field("resourceGroupName"),
            sku: o.get_field("sku"),
            sub_domain: o.get_field("subDomain"),
            tags: o.get_field("tags"),
            template: o.get_field("template"),
        }
    }
}
