/// Manages a Front Door (standard/premium) Endpoint.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-cdn-frontdoor
///       location: West Europe
///   exampleFrontdoorProfile:
///     type: azure:cdn:FrontdoorProfile
///     name: example
///     properties:
///       name: example-profile
///       resourceGroupName: ${example.name}
///       skuName: Standard_AzureFrontDoor
///   exampleFrontdoorEndpoint:
///     type: azure:cdn:FrontdoorEndpoint
///     name: example
///     properties:
///       name: example-endpoint
///       cdnFrontdoorProfileId: ${exampleFrontdoorProfile.id}
///       tags:
///         ENV: example
/// ```
///
/// ## Import
///
/// Front Door Endpoints can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cdn/frontdoorEndpoint:FrontdoorEndpoint example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.Cdn/profiles/profile1/afdEndpoints/endpoint1
/// ```
///
pub mod frontdoor_endpoint {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FrontdoorEndpointArgs {
        /// The ID of the Front Door Profile within which this Front Door Endpoint should exist. Changing this forces a new Front Door Endpoint to be created.
        #[builder(into)]
        pub cdn_frontdoor_profile_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies if this Front Door Endpoint is enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The name which should be used for this Front Door Endpoint. Changing this forces a new Front Door Endpoint to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies a mapping of tags which should be assigned to the Front Door Endpoint.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct FrontdoorEndpointResult {
        /// The ID of the Front Door Profile within which this Front Door Endpoint should exist. Changing this forces a new Front Door Endpoint to be created.
        pub cdn_frontdoor_profile_id: pulumi_wasm_rust::Output<String>,
        /// Specifies if this Front Door Endpoint is enabled? Defaults to `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The host name of the Front Door Endpoint, in the format `{endpointName}.{dnsZone}` (for example, `contoso.azureedge.net`).
        pub host_name: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Front Door Endpoint. Changing this forces a new Front Door Endpoint to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies a mapping of tags which should be assigned to the Front Door Endpoint.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: FrontdoorEndpointArgs,
    ) -> FrontdoorEndpointResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cdn_frontdoor_profile_id_binding = args
            .cdn_frontdoor_profile_id
            .get_output(context)
            .get_inner();
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:cdn/frontdoorEndpoint:FrontdoorEndpoint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cdnFrontdoorProfileId".into(),
                    value: &cdn_frontdoor_profile_id_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "cdnFrontdoorProfileId".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "hostName".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FrontdoorEndpointResult {
            cdn_frontdoor_profile_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cdnFrontdoorProfileId").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            host_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostName").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
