/// A CDN Endpoint is the entity within a CDN Profile containing configuration information regarding caching behaviours and origins. The CDN Endpoint is exposed using the URL format `<endpointname>.azureedge.net`.
///
/// !> **Be Aware:** Azure is rolling out a breaking change on Friday 9th April 2021 which may cause issues with the CDN/FrontDoor resources. More information is available in this GitHub issue - however unfortunately this may necessitate a breaking change to the CDN and FrontDoor resources, more information will be posted in the GitHub issue as the necessary changes are identified.
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
///     let exampleEndpoint = endpoint::create(
///         "exampleEndpoint",
///         EndpointArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .origins(
///                 vec![
///                     EndpointOrigin::builder().hostName("www.contoso.com").name("example")
///                     .build_struct(),
///                 ],
///             )
///             .profile_name("${exampleProfile.name}")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleProfile = profile::create(
///         "exampleProfile",
///         ProfileArgs::builder()
///             .location("${example.location}")
///             .name("example-cdn")
///             .resource_group_name("${example.name}")
///             .sku("Standard_Verizon")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// CDN Endpoints can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cdn/endpoint:Endpoint example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Cdn/profiles/myprofile1/endpoints/myendpoint1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EndpointArgs {
        /// An array of strings that indicates a content types on which compression will be applied. The value for the elements should be MIME types.
        #[builder(into, default)]
        pub content_types_to_compresses: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Rules for the rules engine. An endpoint can contain up until 4 of those rules that consist of conditions and actions. A `delivery_rule` blocks as defined below.
        #[builder(into, default)]
        pub delivery_rules: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::cdn::EndpointDeliveryRule>>,
        >,
        /// A set of Geo Filters for this CDN Endpoint. Each `geo_filter` block supports fields documented below.
        #[builder(into, default)]
        pub geo_filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::cdn::EndpointGeoFilter>>,
        >,
        /// Actions that are valid for all resources regardless of any conditions. A `global_delivery_rule` block as defined below.
        #[builder(into, default)]
        pub global_delivery_rule: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cdn::EndpointGlobalDeliveryRule>,
        >,
        /// Indicates whether compression is to be enabled.
        #[builder(into, default)]
        pub is_compression_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies if http allowed. Defaults to `true`.
        #[builder(into, default)]
        pub is_http_allowed: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies if https allowed. Defaults to `true`.
        #[builder(into, default)]
        pub is_https_allowed: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the CDN Endpoint. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// What types of optimization should this CDN Endpoint optimize for? Possible values include `DynamicSiteAcceleration`, `GeneralMediaStreaming`, `GeneralWebDelivery`, `LargeFileDownload` and `VideoOnDemandMediaStreaming`.
        #[builder(into, default)]
        pub optimization_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The host header CDN provider will send along with content requests to origins.
        #[builder(into, default)]
        pub origin_host_header: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The path used at for origin requests.
        #[builder(into, default)]
        pub origin_path: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The set of origins of the CDN endpoint. When multiple origins exist, the first origin will be used as primary and rest will be used as failover options. Each `origin` block supports fields documented below. Changing this forces a new resource to be created.
        #[builder(into)]
        pub origins: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::cdn::EndpointOrigin>,
        >,
        /// the path to a file hosted on the origin which helps accelerate delivery of the dynamic content and calculate the most optimal routes for the CDN. This is relative to the `origin_path`.
        ///
        /// > **NOTE:** `global_delivery_rule` and `delivery_rule` are currently only available for `Microsoft_Standard` CDN profiles.
        #[builder(into, default)]
        pub probe_path: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The CDN Profile to which to attach the CDN Endpoint. Changing this forces a new resource to be created.
        #[builder(into)]
        pub profile_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Sets query string caching behavior. Allowed values are `IgnoreQueryString`, `BypassCaching` and `UseQueryString`. `NotSet` value can be used for `Premium Verizon` CDN profile. Defaults to `IgnoreQueryString`.
        #[builder(into, default)]
        pub querystring_caching_behaviour: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The name of the resource group in which to create the CDN Endpoint. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct EndpointResult {
        /// An array of strings that indicates a content types on which compression will be applied. The value for the elements should be MIME types.
        pub content_types_to_compresses: pulumi_gestalt_rust::Output<
            Option<Vec<String>>,
        >,
        /// Rules for the rules engine. An endpoint can contain up until 4 of those rules that consist of conditions and actions. A `delivery_rule` blocks as defined below.
        pub delivery_rules: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::cdn::EndpointDeliveryRule>>,
        >,
        /// The Fully Qualified Domain Name of the CDN Endpoint.
        pub fqdn: pulumi_gestalt_rust::Output<String>,
        /// A set of Geo Filters for this CDN Endpoint. Each `geo_filter` block supports fields documented below.
        pub geo_filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::cdn::EndpointGeoFilter>>,
        >,
        /// Actions that are valid for all resources regardless of any conditions. A `global_delivery_rule` block as defined below.
        pub global_delivery_rule: pulumi_gestalt_rust::Output<
            Option<super::super::types::cdn::EndpointGlobalDeliveryRule>,
        >,
        /// Indicates whether compression is to be enabled.
        pub is_compression_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies if http allowed. Defaults to `true`.
        pub is_http_allowed: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies if https allowed. Defaults to `true`.
        pub is_https_allowed: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the CDN Endpoint. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// What types of optimization should this CDN Endpoint optimize for? Possible values include `DynamicSiteAcceleration`, `GeneralMediaStreaming`, `GeneralWebDelivery`, `LargeFileDownload` and `VideoOnDemandMediaStreaming`.
        pub optimization_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The host header CDN provider will send along with content requests to origins.
        pub origin_host_header: pulumi_gestalt_rust::Output<Option<String>>,
        /// The path used at for origin requests.
        pub origin_path: pulumi_gestalt_rust::Output<Option<String>>,
        /// The set of origins of the CDN endpoint. When multiple origins exist, the first origin will be used as primary and rest will be used as failover options. Each `origin` block supports fields documented below. Changing this forces a new resource to be created.
        pub origins: pulumi_gestalt_rust::Output<
            Vec<super::super::types::cdn::EndpointOrigin>,
        >,
        /// the path to a file hosted on the origin which helps accelerate delivery of the dynamic content and calculate the most optimal routes for the CDN. This is relative to the `origin_path`.
        ///
        /// > **NOTE:** `global_delivery_rule` and `delivery_rule` are currently only available for `Microsoft_Standard` CDN profiles.
        pub probe_path: pulumi_gestalt_rust::Output<Option<String>>,
        /// The CDN Profile to which to attach the CDN Endpoint. Changing this forces a new resource to be created.
        pub profile_name: pulumi_gestalt_rust::Output<String>,
        /// Sets query string caching behavior. Allowed values are `IgnoreQueryString`, `BypassCaching` and `UseQueryString`. `NotSet` value can be used for `Premium Verizon` CDN profile. Defaults to `IgnoreQueryString`.
        pub querystring_caching_behaviour: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the CDN Endpoint. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EndpointArgs,
    ) -> EndpointResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let content_types_to_compresses_binding = args
            .content_types_to_compresses
            .get_output(context);
        let delivery_rules_binding = args.delivery_rules.get_output(context);
        let geo_filters_binding = args.geo_filters.get_output(context);
        let global_delivery_rule_binding = args.global_delivery_rule.get_output(context);
        let is_compression_enabled_binding = args
            .is_compression_enabled
            .get_output(context);
        let is_http_allowed_binding = args.is_http_allowed.get_output(context);
        let is_https_allowed_binding = args.is_https_allowed.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let optimization_type_binding = args.optimization_type.get_output(context);
        let origin_host_header_binding = args.origin_host_header.get_output(context);
        let origin_path_binding = args.origin_path.get_output(context);
        let origins_binding = args.origins.get_output(context);
        let probe_path_binding = args.probe_path.get_output(context);
        let profile_name_binding = args.profile_name.get_output(context);
        let querystring_caching_behaviour_binding = args
            .querystring_caching_behaviour
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:cdn/endpoint:Endpoint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contentTypesToCompresses".into(),
                    value: content_types_to_compresses_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deliveryRules".into(),
                    value: delivery_rules_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "geoFilters".into(),
                    value: geo_filters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "globalDeliveryRule".into(),
                    value: global_delivery_rule_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "isCompressionEnabled".into(),
                    value: is_compression_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "isHttpAllowed".into(),
                    value: is_http_allowed_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "isHttpsAllowed".into(),
                    value: is_https_allowed_binding.get_id(),
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
                    name: "optimizationType".into(),
                    value: optimization_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "originHostHeader".into(),
                    value: origin_host_header_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "originPath".into(),
                    value: origin_path_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "origins".into(),
                    value: origins_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "probePath".into(),
                    value: probe_path_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "profileName".into(),
                    value: profile_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "querystringCachingBehaviour".into(),
                    value: querystring_caching_behaviour_binding.get_id(),
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
        let o = context.register_resource(request);
        EndpointResult {
            content_types_to_compresses: o.get_field("contentTypesToCompresses"),
            delivery_rules: o.get_field("deliveryRules"),
            fqdn: o.get_field("fqdn"),
            geo_filters: o.get_field("geoFilters"),
            global_delivery_rule: o.get_field("globalDeliveryRule"),
            is_compression_enabled: o.get_field("isCompressionEnabled"),
            is_http_allowed: o.get_field("isHttpAllowed"),
            is_https_allowed: o.get_field("isHttpsAllowed"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            optimization_type: o.get_field("optimizationType"),
            origin_host_header: o.get_field("originHostHeader"),
            origin_path: o.get_field("originPath"),
            origins: o.get_field("origins"),
            probe_path: o.get_field("probePath"),
            profile_name: o.get_field("profileName"),
            querystring_caching_behaviour: o.get_field("querystringCachingBehaviour"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
        }
    }
}
