/// Manages an App Service Static Web App.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleStaticWebApp = static_web_app::create(
///         "exampleStaticWebApp",
///         StaticWebAppArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Static Web Apps can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appservice/staticWebApp:StaticWebApp example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Web/staticSites/my-static-site1
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod static_web_app {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StaticWebAppArgs {
        /// A key-value pair of App Settings.
        #[builder(into, default)]
        pub app_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `basic_auth` block as defined below.
        #[builder(into, default)]
        pub basic_auth: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appservice::StaticWebAppBasicAuth>,
        >,
        /// Should changes to the configuration file be permitted. Defaults to `true`.
        #[builder(into, default)]
        pub configuration_file_changes_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appservice::StaticWebAppIdentity>,
        >,
        /// The Azure Region where the Static Web App should exist. Changing this forces a new Static Web App to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Static Web App. Changing this forces a new Static Web App to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Are Preview (Staging) environments enabled. Defaults to `true`.
        #[builder(into, default)]
        pub preview_environments_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Should public network access be enabled for the Static Web App. Defaults to `true`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The name of the Resource Group where the Static Web App should exist. Changing this forces a new Static Web App to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the SKU size of the Static Web App. Possible values are `Free` or `Standard`. Defaults to `Free`.
        #[builder(into, default)]
        pub sku_size: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the SKU tier of the Static Web App. Possible values are `Free` or `Standard`. Defaults to `Free`.
        #[builder(into, default)]
        pub sku_tier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct StaticWebAppResult {
        /// The API key of this Static Web App, which is used for later interacting with this Static Web App from other clients, e.g. GitHub Action.
        pub api_key: pulumi_gestalt_rust::Output<String>,
        /// A key-value pair of App Settings.
        pub app_settings: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `basic_auth` block as defined below.
        pub basic_auth: pulumi_gestalt_rust::Output<
            Option<super::super::types::appservice::StaticWebAppBasicAuth>,
        >,
        /// Should changes to the configuration file be permitted. Defaults to `true`.
        pub configuration_file_changes_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// The default host name of the Static Web App.
        pub default_host_name: pulumi_gestalt_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::appservice::StaticWebAppIdentity>,
        >,
        /// The Azure Region where the Static Web App should exist. Changing this forces a new Static Web App to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Static Web App. Changing this forces a new Static Web App to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Are Preview (Staging) environments enabled. Defaults to `true`.
        pub preview_environments_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Should public network access be enabled for the Static Web App. Defaults to `true`.
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the Resource Group where the Static Web App should exist. Changing this forces a new Static Web App to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the SKU size of the Static Web App. Possible values are `Free` or `Standard`. Defaults to `Free`.
        pub sku_size: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the SKU tier of the Static Web App. Possible values are `Free` or `Standard`. Defaults to `Free`.
        pub sku_tier: pulumi_gestalt_rust::Output<Option<String>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: StaticWebAppArgs,
    ) -> StaticWebAppResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let app_settings_binding = args.app_settings.get_output(context).get_inner();
        let basic_auth_binding = args.basic_auth.get_output(context).get_inner();
        let configuration_file_changes_enabled_binding = args
            .configuration_file_changes_enabled
            .get_output(context)
            .get_inner();
        let identity_binding = args.identity.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let preview_environments_enabled_binding = args
            .preview_environments_enabled
            .get_output(context)
            .get_inner();
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let sku_size_binding = args.sku_size.get_output(context).get_inner();
        let sku_tier_binding = args.sku_tier.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appservice/staticWebApp:StaticWebApp".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "appSettings".into(),
                    value: &app_settings_binding,
                },
                register_interface::ObjectField {
                    name: "basicAuth".into(),
                    value: &basic_auth_binding,
                },
                register_interface::ObjectField {
                    name: "configurationFileChangesEnabled".into(),
                    value: &configuration_file_changes_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "previewEnvironmentsEnabled".into(),
                    value: &preview_environments_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "skuSize".into(),
                    value: &sku_size_binding,
                },
                register_interface::ObjectField {
                    name: "skuTier".into(),
                    value: &sku_tier_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        StaticWebAppResult {
            api_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("apiKey"),
            ),
            app_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("appSettings"),
            ),
            basic_auth: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("basicAuth"),
            ),
            configuration_file_changes_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("configurationFileChangesEnabled"),
            ),
            default_host_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultHostName"),
            ),
            identity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identity"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            preview_environments_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("previewEnvironmentsEnabled"),
            ),
            public_network_access_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publicNetworkAccessEnabled"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            sku_size: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("skuSize"),
            ),
            sku_tier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("skuTier"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
