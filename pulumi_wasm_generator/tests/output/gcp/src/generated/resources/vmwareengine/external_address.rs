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
pub mod external_address {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExternalAddressArgs {
        /// User-provided description for this resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The internal IP address of a workload VM.
        #[builder(into)]
        pub internal_ip: pulumi_wasm_rust::Output<String>,
        /// The ID of the external IP Address.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The resource name of the private cloud to create a new external address in.
        /// Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.
        /// For example: projects/my-project/locations/us-west1-a/privateClouds/my-cloud
        #[builder(into)]
        pub parent: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ExternalAddressResult {
        /// Creation time of this resource.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and
        /// up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// User-provided description for this resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The external IP address of a workload VM.
        pub external_ip: pulumi_wasm_rust::Output<String>,
        /// The internal IP address of a workload VM.
        pub internal_ip: pulumi_wasm_rust::Output<String>,
        /// The ID of the external IP Address.
        ///
        ///
        /// - - -
        pub name: pulumi_wasm_rust::Output<String>,
        /// The resource name of the private cloud to create a new external address in.
        /// Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.
        /// For example: projects/my-project/locations/us-west1-a/privateClouds/my-cloud
        pub parent: pulumi_wasm_rust::Output<String>,
        /// State of the resource.
        pub state: pulumi_wasm_rust::Output<String>,
        /// System-generated unique identifier for the resource.
        pub uid: pulumi_wasm_rust::Output<String>,
        /// Last updated time of this resource.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine
        /// fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ExternalAddressArgs) -> ExternalAddressResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let internal_ip_binding = args.internal_ip.get_inner();
        let name_binding = args.name.get_inner();
        let parent_binding = args.parent.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:vmwareengine/externalAddress:ExternalAddress".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "externalIp".into(),
                },
                register_interface::ResultField {
                    name: "internalIp".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "parent".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "uid".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ExternalAddressResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            external_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("externalIp").unwrap(),
            ),
            internal_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("internalIp").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            parent: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parent").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            uid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uid").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
