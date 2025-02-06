/// Represents a TargetInstance resource which defines an endpoint instance
/// that terminates traffic of certain protocols. In particular, they are used
/// in Protocol Forwarding, where forwarding rules can send packets to a
/// non-NAT'ed target instance. Each target instance contains a single
/// virtual machine instance that receives and handles traffic from the
/// corresponding forwarding rules.
///
///
/// To get more information about TargetInstance, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/v1/targetInstances)
/// * How-to Guides
///     * [Using Protocol Forwarding](https://cloud.google.com/compute/docs/protocol-forwarding)
///
/// ## Example Usage
///
/// ### Target Instance Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:TargetInstance
///     properties:
///       name: target
///       instance: ${["target-vm"].id}
///   target-vm:
///     type: gcp:compute:Instance
///     properties:
///       name: target-vm
///       machineType: e2-medium
///       zone: us-central1-a
///       bootDisk:
///         initializeParams:
///           image: ${vmimage.selfLink}
///       networkInterfaces:
///         - network: default
/// variables:
///   vmimage:
///     fn::invoke:
///       function: gcp:compute:getImage
///       arguments:
///         family: debian-11
///         project: debian-cloud
/// ```
/// ### Target Instance Custom Network
///
///
/// ```yaml
/// resources:
///   customNetwork:
///     type: gcp:compute:TargetInstance
///     name: custom_network
///     properties:
///       name: custom-network
///       instance: ${["target-vmInstance"].id}
///       network: ${["target-vm"].selfLink}
///   target-vmInstance:
///     type: gcp:compute:Instance
///     name: target-vm
///     properties:
///       name: custom-network-target-vm
///       machineType: e2-medium
///       zone: us-central1-a
///       bootDisk:
///         initializeParams:
///           image: ${vmimage.selfLink}
///       networkInterfaces:
///         - network: default
/// variables:
///   target-vm:
///     fn::invoke:
///       function: gcp:compute:getNetwork
///       arguments:
///         name: default
///   vmimage:
///     fn::invoke:
///       function: gcp:compute:getImage
///       arguments:
///         family: debian-12
///         project: debian-cloud
/// ```
/// ### Target Instance With Security Policy
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:Network
///     properties:
///       name: custom-default-network
///       autoCreateSubnetworks: false
///       routingMode: REGIONAL
///   defaultSubnetwork:
///     type: gcp:compute:Subnetwork
///     name: default
///     properties:
///       name: custom-default-subnet
///       ipCidrRange: 10.1.2.0/24
///       network: ${default.id}
///       privateIpv6GoogleAccess: DISABLE_GOOGLE_ACCESS
///       purpose: PRIVATE
///       region: southamerica-west1
///       stackType: IPV4_ONLY
///   target-vm:
///     type: gcp:compute:Instance
///     properties:
///       networkInterfaces:
///         - accessConfigs:
///             - {}
///           network: ${default.selfLink}
///           subnetwork: ${defaultSubnetwork.selfLink}
///       name: target-vm
///       machineType: e2-medium
///       zone: southamerica-west1-a
///       bootDisk:
///         initializeParams:
///           image: ${vmimage.selfLink}
///   policyddosprotection:
///     type: gcp:compute:RegionSecurityPolicy
///     properties:
///       region: southamerica-west1
///       name: tf-test-policyddos_85794
///       description: ddos protection security policy to set target instance
///       type: CLOUD_ARMOR_NETWORK
///       ddosProtectionConfig:
///         ddosProtection: ADVANCED_PREVIEW
///   edgeSecService:
///     type: gcp:compute:NetworkEdgeSecurityService
///     name: edge_sec_service
///     properties:
///       region: southamerica-west1
///       name: tf-test-edgesec_21197
///       securityPolicy: ${policyddosprotection.selfLink}
///   regionsecuritypolicy:
///     type: gcp:compute:RegionSecurityPolicy
///     properties:
///       name: region-secpolicy
///       region: southamerica-west1
///       description: basic security policy for target instance
///       type: CLOUD_ARMOR_NETWORK
///     options:
///       dependsOn:
///         - ${edgeSecService}
///   defaultTargetInstance:
///     type: gcp:compute:TargetInstance
///     name: default
///     properties:
///       name: target-instance
///       zone: southamerica-west1-a
///       instance: ${["target-vm"].id}
///       securityPolicy: ${regionsecuritypolicy.selfLink}
/// variables:
///   vmimage:
///     fn::invoke:
///       function: gcp:compute:getImage
///       arguments:
///         family: debian-11
///         project: debian-cloud
/// ```
///
/// ## Import
///
/// TargetInstance can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/zones/{{zone}}/targetInstances/{{name}}`
///
/// * `{{project}}/{{zone}}/{{name}}`
///
/// * `{{zone}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, TargetInstance can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/targetInstance:TargetInstance default projects/{{project}}/zones/{{zone}}/targetInstances/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/targetInstance:TargetInstance default {{project}}/{{zone}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/targetInstance:TargetInstance default {{zone}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/targetInstance:TargetInstance default {{name}}
/// ```
///
pub mod target_instance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TargetInstanceArgs {
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Compute instance VM handling traffic for this target instance.
        /// Accepts the instance self-link, relative path
        /// (e.g. `projects/project/zones/zone/instances/instance`) or name. If
        /// name is given, the zone will default to the given zone or
        /// the provider-default zone and the project will default to the
        /// provider-level project.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub instance: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// NAT option controlling how IPs are NAT'ed to the instance.
        /// Currently only NO_NAT (default value) is supported.
        /// Default value is `NO_NAT`.
        /// Possible values are: `NO_NAT`.
        #[builder(into, default)]
        pub nat_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The URL of the network this target instance uses to forward traffic. If not specified, the traffic will be forwarded to the network that the default network interface belongs to.
        #[builder(into, default)]
        pub network: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The resource URL for the security policy associated with this target instance.
        #[builder(into, default)]
        pub security_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// URL of the zone where the target instance resides.
        #[builder(into, default)]
        pub zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct TargetInstanceResult {
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// An optional description of this resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Compute instance VM handling traffic for this target instance.
        /// Accepts the instance self-link, relative path
        /// (e.g. `projects/project/zones/zone/instances/instance`) or name. If
        /// name is given, the zone will default to the given zone or
        /// the provider-default zone and the project will default to the
        /// provider-level project.
        ///
        ///
        /// - - -
        pub instance: pulumi_gestalt_rust::Output<String>,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// NAT option controlling how IPs are NAT'ed to the instance.
        /// Currently only NO_NAT (default value) is supported.
        /// Default value is `NO_NAT`.
        /// Possible values are: `NO_NAT`.
        pub nat_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// The URL of the network this target instance uses to forward traffic. If not specified, the traffic will be forwarded to the network that the default network interface belongs to.
        pub network: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The resource URL for the security policy associated with this target instance.
        pub security_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// URL of the zone where the target instance resides.
        pub zone: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: TargetInstanceArgs,
    ) -> TargetInstanceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let instance_binding = args.instance.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let nat_policy_binding = args.nat_policy.get_output(context).get_inner();
        let network_binding = args.network.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let security_policy_binding = args
            .security_policy
            .get_output(context)
            .get_inner();
        let zone_binding = args.zone.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/targetInstance:TargetInstance".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "instance".into(),
                    value: &instance_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "natPolicy".into(),
                    value: &nat_policy_binding,
                },
                register_interface::ObjectField {
                    name: "network".into(),
                    value: &network_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "securityPolicy".into(),
                    value: &security_policy_binding,
                },
                register_interface::ObjectField {
                    name: "zone".into(),
                    value: &zone_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        TargetInstanceResult {
            creation_timestamp: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("creationTimestamp"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            instance: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instance"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            nat_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("natPolicy"),
            ),
            network: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("network"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            security_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("securityPolicy"),
            ),
            self_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            zone: pulumi_gestalt_rust::__private::into_domain(o.extract_field("zone")),
        }
    }
}
