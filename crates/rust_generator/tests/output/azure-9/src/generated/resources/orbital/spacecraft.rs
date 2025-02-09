/// Manages a Spacecraft.
///
/// > **Note:** The `azure.orbital.Spacecraft` resource has been deprecated and will be removed in v5.0 of the AzureRM Provider.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: rg-example
///       location: West Europe
///   exampleSpacecraft:
///     type: azure:orbital:Spacecraft
///     name: example
///     properties:
///       name: example-spacecraft
///       resourceGroupName: ${example.name}
///       location: westeurope
///       noradId: '12345'
///       links:
///         - bandwidthMhz: 30
///           centerFrequencyMhz: 2050
///           direction: Uplink
///           polarization: LHCP
///           name: examplename
///       twoLineElements:
///         - 1 23455U 94089A   97320.90946019  .00000140  00000-0  10191-3 0  2621
///         - 2 23455  99.0090 272.6745 0008546 223.1686 136.8816 14.11711747148495
///       titleLine: AQUA
///       tags:
///         aks-managed-cluster-name: 9a57225d-a405-4d40-aa46-f13d2342abef
/// ```
///
/// ## Import
///
/// Spacecraft can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:orbital/spacecraft:Spacecraft example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Orbital/spacecrafts/spacecraft1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod spacecraft {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpacecraftArgs {
        /// A `links` block as defined below. Changing this forces a new resource to be created.
        #[builder(into)]
        pub links: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::orbital::SpacecraftLink>,
        >,
        /// The location where the Spacecraft exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Spacecraft. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// NORAD ID of the Spacecraft.
        #[builder(into)]
        pub norad_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Spacecraft exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Title of the two line elements (TLE).
        #[builder(into)]
        pub title_line: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A list of the two line elements (TLE), the first string being the first of the TLE, the second string being the second line of the TLE. Changing this forces a new resource to be created.
        #[builder(into)]
        pub two_line_elements: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
    }
    #[allow(dead_code)]
    pub struct SpacecraftResult {
        /// A `links` block as defined below. Changing this forces a new resource to be created.
        pub links: pulumi_gestalt_rust::Output<
            Vec<super::super::types::orbital::SpacecraftLink>,
        >,
        /// The location where the Spacecraft exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the Spacecraft. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// NORAD ID of the Spacecraft.
        pub norad_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Spacecraft exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Title of the two line elements (TLE).
        pub title_line: pulumi_gestalt_rust::Output<String>,
        /// A list of the two line elements (TLE), the first string being the first of the TLE, the second string being the second line of the TLE. Changing this forces a new resource to be created.
        pub two_line_elements: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SpacecraftArgs,
    ) -> SpacecraftResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let links_binding = args.links.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let norad_id_binding = args.norad_id.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let title_line_binding = args.title_line.get_output(context);
        let two_line_elements_binding = args.two_line_elements.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:orbital/spacecraft:Spacecraft".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "links".into(),
                    value: links_binding.get_id(),
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
                    name: "noradId".into(),
                    value: norad_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "titleLine".into(),
                    value: title_line_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "twoLineElements".into(),
                    value: two_line_elements_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SpacecraftResult {
            links: o.get_field("links"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            norad_id: o.get_field("noradId"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
            title_line: o.get_field("titleLine"),
            two_line_elements: o.get_field("twoLineElements"),
        }
    }
}
