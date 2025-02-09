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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ExternalAddressArgs,
    ) -> ExternalAddressResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let internal_ip_binding_1 = args.internal_ip.get_output(context);
        let internal_ip_binding = internal_ip_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let parent_binding_1 = args.parent.get_output(context);
        let parent_binding = parent_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:vmwareengine/externalAddress:ExternalAddress".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "internalIp".into(),
                    value: &internal_ip_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ExternalAddressResult {
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            external_ip: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("externalIp"),
            ),
            internal_ip: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("internalIp"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            parent: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parent"),
            ),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            uid: pulumi_gestalt_rust::__private::into_domain(o.extract_field("uid")),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
