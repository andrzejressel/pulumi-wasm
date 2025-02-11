/// An allocated external IP address and its corresponding internal IP address in a private cloud.
///
///
/// To get more information about ExternalAddress, see:
///
/// * [API documentation](https://cloud.google.com/vmware-engine/docs/reference/rest/v1/projects.locations.privateClouds.externalAddresses)
///
/// ## Example Usage
///
/// ### Vmware Engine External Address Basic
///
///
/// ```yaml
/// resources:
///   external-address-nw:
///     type: gcp:vmwareengine:Network
///     properties:
///       name: pc-nw
///       location: global
///       type: STANDARD
///       description: PC network description.
///   external-address-pc:
///     type: gcp:vmwareengine:PrivateCloud
///     properties:
///       location: -a
///       name: sample-pc
///       description: Sample test PC.
///       networkConfig:
///         managementCidr: 192.168.50.0/24
///         vmwareEngineNetwork: ${["external-address-nw"].id}
///       managementCluster:
///         clusterId: sample-mgmt-cluster
///         nodeTypeConfigs:
///           - nodeTypeId: standard-72
///             nodeCount: 3
///   external-address-np:
///     type: gcp:vmwareengine:NetworkPolicy
///     properties:
///       location: ""
///       name: sample-np
///       edgeServicesCidr: 192.168.30.0/26
///       vmwareEngineNetwork: ${["external-address-nw"].id}
///   vmw-engine-external-address:
///     type: gcp:vmwareengine:ExternalAddress
///     properties:
///       name: sample-external-address
///       parent: ${["external-address-pc"].id}
///       internalIp: 192.168.0.66
///       description: Sample description.
///     options:
///       dependsOn:
///         - ${["external-address-np"]}
/// ```
///
/// ## Import
///
/// ExternalAddress can be imported using any of these accepted formats:
///
/// * `{{parent}}/externalAddresses/{{name}}`
///
/// When using the `pulumi import` command, ExternalAddress can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:vmwareengine/externalAddress:ExternalAddress default {{parent}}/externalAddresses/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod external_address {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExternalAddressArgs {
        /// User-provided description for this resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The internal IP address of a workload VM.
        #[builder(into)]
        pub internal_ip: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the external IP Address.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The resource name of the private cloud to create a new external address in.
        /// Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.
        /// For example: projects/my-project/locations/us-west1-a/privateClouds/my-cloud
        #[builder(into)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ExternalAddressResult {
        /// Creation time of this resource.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and
        /// up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// User-provided description for this resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The external IP address of a workload VM.
        pub external_ip: pulumi_gestalt_rust::Output<String>,
        /// The internal IP address of a workload VM.
        pub internal_ip: pulumi_gestalt_rust::Output<String>,
        /// The ID of the external IP Address.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The resource name of the private cloud to create a new external address in.
        /// Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.
        /// For example: projects/my-project/locations/us-west1-a/privateClouds/my-cloud
        pub parent: pulumi_gestalt_rust::Output<String>,
        /// State of the resource.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// System-generated unique identifier for the resource.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// Last updated time of this resource.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine
        /// fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ExternalAddressArgs,
    ) -> ExternalAddressResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let internal_ip_binding = args.internal_ip.get_output(context);
        let name_binding = args.name.get_output(context);
        let parent_binding = args.parent.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:vmwareengine/externalAddress:ExternalAddress".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "internalIp".into(),
                    value: &internal_ip_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parent".into(),
                    value: &parent_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ExternalAddressResult {
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            external_ip: o.get_field("externalIp"),
            internal_ip: o.get_field("internalIp"),
            name: o.get_field("name"),
            parent: o.get_field("parent"),
            state: o.get_field("state"),
            uid: o.get_field("uid"),
            update_time: o.get_field("updateTime"),
        }
    }
}
