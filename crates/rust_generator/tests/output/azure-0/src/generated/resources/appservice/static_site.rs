/// Manages an App Service Static Site.
///
/// > **NOTE:** The `azure.appservice.StaticSite` resource is deprecated in favour of `azure.appservice.StaticWebApp` and will be removed in a future major release.
///
/// ->**NOTE:** After the Static Site is provisioned, you'll need to associate your target repository, which contains your web app, to the Static Site, by following the [Azure Static Site document](https://docs.microsoft.com/azure/static-web-apps/github-actions-workflow).
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
///     let exampleStaticSite = static_site::create(
///         "exampleStaticSite",
///         StaticSiteArgs::builder()
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
/// $ pulumi import azure:appservice/staticSite:StaticSite example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Web/staticSites/my-static-site1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod static_site {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StaticSiteArgs {
        /// A key-value pair of App Settings.
        #[builder(into, default)]
        pub app_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appservice::StaticSiteIdentity>,
        >,
        /// The Azure Region where the Static Web App should exist. Changing this forces a new Static Web App to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Static Web App. Changing this forces a new Static Web App to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
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
    pub struct StaticSiteResult {
        /// The API key of this Static Web App, which is used for later interacting with this Static Web App from other clients, e.g. GitHub Action.
        pub api_key: pulumi_gestalt_rust::Output<String>,
        /// A key-value pair of App Settings.
        pub app_settings: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The default host name of the Static Web App.
        pub default_host_name: pulumi_gestalt_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::appservice::StaticSiteIdentity>,
        >,
        /// The Azure Region where the Static Web App should exist. Changing this forces a new Static Web App to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Static Web App. Changing this forces a new Static Web App to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
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
        args: StaticSiteArgs,
    ) -> StaticSiteResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let app_settings_binding_1 = args.app_settings.get_output(context);
        let app_settings_binding = app_settings_binding_1.get_inner();
        let identity_binding_1 = args.identity.get_output(context);
        let identity_binding = identity_binding_1.get_inner();
        let location_binding_1 = args.location.get_output(context);
        let location_binding = location_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let sku_size_binding_1 = args.sku_size.get_output(context);
        let sku_size_binding = sku_size_binding_1.get_inner();
        let sku_tier_binding_1 = args.sku_tier.get_output(context);
        let sku_tier_binding = sku_tier_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appservice/staticSite:StaticSite".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "appSettings".into(),
                    value: &app_settings_binding,
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
        StaticSiteResult {
            api_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("apiKey"),
            ),
            app_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("appSettings"),
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
