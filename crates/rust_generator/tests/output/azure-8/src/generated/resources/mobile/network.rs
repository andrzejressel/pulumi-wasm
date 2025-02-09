/// Manages a Mobile Network.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: east us
///   exampleNetwork:
///     type: azure:mobile:Network
///     name: example
///     properties:
///       name: example-mn
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       mobileCountryCode: '001'
///       mobileNetworkCode: '01'
///       tags:
///         key: value
/// ```
///
/// ## Import
///
/// Mobile Network can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:mobile/network:Network example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.MobileNetwork/mobileNetworks/mobileNetwork1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod network {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkArgs {
        /// Specifies the Azure Region where the Mobile Network should exist. Changing this forces a new Mobile Network to be created. The possible values are `eastus` and `northeurope`.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Mobile country code (MCC), defined in https://www.itu.int/rec/T-REC-E.212 . Changing this forces a new resource to be created.
        #[builder(into)]
        pub mobile_country_code: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Mobile network code (MNC), defined in https://www.itu.int/rec/T-REC-E.212 . Changing this forces a new resource to be created.
        #[builder(into)]
        pub mobile_network_code: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name which should be used for this Mobile Network. Changing this forces a new Mobile Network to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Resource Group where the Mobile Network should exist. Changing this forces a new Mobile Network to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Mobile Network.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct NetworkResult {
        /// Specifies the Azure Region where the Mobile Network should exist. Changing this forces a new Mobile Network to be created. The possible values are `eastus` and `northeurope`.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Mobile country code (MCC), defined in https://www.itu.int/rec/T-REC-E.212 . Changing this forces a new resource to be created.
        pub mobile_country_code: pulumi_gestalt_rust::Output<String>,
        /// Mobile network code (MNC), defined in https://www.itu.int/rec/T-REC-E.212 . Changing this forces a new resource to be created.
        pub mobile_network_code: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name which should be used for this Mobile Network. Changing this forces a new Mobile Network to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Resource Group where the Mobile Network should exist. Changing this forces a new Mobile Network to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The mobile network resource identifier.
        pub service_key: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Mobile Network.
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
        args: NetworkArgs,
    ) -> NetworkResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let location_binding = args.location.get_output(context);
        let mobile_country_code_binding = args.mobile_country_code.get_output(context);
        let mobile_network_code_binding = args.mobile_network_code.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:mobile/network:Network".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mobileCountryCode".into(),
                    value: mobile_country_code_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mobileNetworkCode".into(),
                    value: mobile_network_code_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
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
        NetworkResult {
            location: o.get_field("location"),
            mobile_country_code: o.get_field("mobileCountryCode"),
            mobile_network_code: o.get_field("mobileNetworkCode"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            service_key: o.get_field("serviceKey"),
            tags: o.get_field("tags"),
        }
    }
}
