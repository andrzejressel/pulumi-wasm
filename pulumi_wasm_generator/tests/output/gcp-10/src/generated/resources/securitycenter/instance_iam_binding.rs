/// Represents a Data Fusion instance.
///
///
/// To get more information about Instance, see:
///
/// * [API documentation](https://cloud.google.com/data-fusion/docs/reference/rest/v1beta1/projects.locations.instances)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/data-fusion/docs/)
///
/// ## Example Usage
///
/// ### Data Fusion Instance Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basicInstance = instance::create(
///         "basicInstance",
///         InstanceArgs::builder()
///             .name("my-instance")
///             .region("us-central1")
///             .type_("BASIC")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Data Fusion Instance Full
///
///
/// ```yaml
/// resources:
///   extendedInstance:
///     type: gcp:datafusion:Instance
///     name: extended_instance
///     properties:
///       name: my-instance
///       description: My Data Fusion instance
///       displayName: My Data Fusion instance
///       region: us-central1
///       type: BASIC
///       enableStackdriverLogging: true
///       enableStackdriverMonitoring: true
///       privateInstance: true
///       dataprocServiceAccount: ${default.email}
///       labels:
///         example_key: example_value
///       networkConfig:
///         network: default
///         ipAllocation: ${privateIpAlloc.address}/${privateIpAlloc.prefixLength}
///       accelerators:
///         - acceleratorType: CDC
///           state: ENABLED
///   network:
///     type: gcp:compute:Network
///     properties:
///       name: datafusion-full-network
///   privateIpAlloc:
///     type: gcp:compute:GlobalAddress
///     name: private_ip_alloc
///     properties:
///       name: datafusion-ip-alloc
///       addressType: INTERNAL
///       purpose: VPC_PEERING
///       prefixLength: 22
///       network: ${network.id}
/// variables:
///   default:
///     fn::invoke:
///       function: gcp:appengine:getDefaultServiceAccount
///       arguments: {}
/// ```
/// ### Data Fusion Instance Psc
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let psc = network::create(
///         "psc",
///         NetworkArgs::builder()
///             .auto_create_subnetworks(false)
///             .name("datafusion-psc-network")
///             .build_struct(),
///     );
///     let pscInstance = instance::create(
///         "pscInstance",
///         InstanceArgs::builder()
///             .name("psc-instance")
///             .network_config(
///                 InstanceNetworkConfig::builder()
///                     .connectionType("PRIVATE_SERVICE_CONNECT_INTERFACES")
///                     .privateServiceConnectConfig(
///                         InstanceNetworkConfigPrivateServiceConnectConfig::builder()
///                             .networkAttachment("${pscNetworkAttachment.id}")
///                             .unreachableCidrBlock("192.168.0.0/25")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .private_instance(true)
///             .region("us-central1")
///             .type_("BASIC")
///             .build_struct(),
///     );
///     let pscNetworkAttachment = network_attachment::create(
///         "pscNetworkAttachment",
///         NetworkAttachmentArgs::builder()
///             .connection_preference("ACCEPT_AUTOMATIC")
///             .name("datafusion-psc-attachment")
///             .region("us-central1")
///             .subnetworks(vec!["${pscSubnetwork.selfLink}",])
///             .build_struct(),
///     );
///     let pscSubnetwork = subnetwork::create(
///         "pscSubnetwork",
///         SubnetworkArgs::builder()
///             .ip_cidr_range("10.0.0.0/16")
///             .name("datafusion-psc-subnet")
///             .network("${psc.id}")
///             .region("us-central1")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Data Fusion Instance Cmek
///
///
/// ```yaml
/// resources:
///   cmek:
///     type: gcp:datafusion:Instance
///     properties:
///       name: my-instance
///       region: us-central1
///       type: BASIC
///       cryptoKeyConfig:
///         keyReference: ${cryptoKey.id}
///     options:
///       dependsOn:
///         - ${cryptoKeyMember}
///   cryptoKey:
///     type: gcp:kms:CryptoKey
///     name: crypto_key
///     properties:
///       name: my-instance
///       keyRing: ${keyRing.id}
///   keyRing:
///     type: gcp:kms:KeyRing
///     name: key_ring
///     properties:
///       name: my-instance
///       location: us-central1
///   cryptoKeyMember:
///     type: gcp:kms:CryptoKeyIAMMember
///     name: crypto_key_member
///     properties:
///       cryptoKeyId: ${cryptoKey.id}
///       role: roles/cloudkms.cryptoKeyEncrypterDecrypter
///       member: serviceAccount:service-${project.number}@gcp-sa-datafusion.iam.gserviceaccount.com
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Data Fusion Instance Enterprise
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let enterpriseInstance = instance::create(
///         "enterpriseInstance",
///         InstanceArgs::builder()
///             .enable_rbac(true)
///             .name("my-instance")
///             .region("us-central1")
///             .type_("ENTERPRISE")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Data Fusion Instance Event
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let event = instance::create(
///         "event",
///         InstanceArgs::builder()
///             .event_publish_config(
///                 InstanceEventPublishConfig::builder()
///                     .enabled(true)
///                     .topic("${eventTopic.id}")
///                     .build_struct(),
///             )
///             .name("my-instance")
///             .region("us-central1")
///             .type_("BASIC")
///             .build_struct(),
///     );
///     let eventTopic = topic::create(
///         "eventTopic",
///         TopicArgs::builder().name("my-instance").build_struct(),
///     );
/// }
/// ```
/// ### Data Fusion Instance Zone
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let zone = instance::create(
///         "zone",
///         InstanceArgs::builder()
///             .name("my-instance")
///             .region("us-central1")
///             .type_("DEVELOPER")
///             .zone("us-central1-a")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Instance can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{region}}/instances/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Instance can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:securitycenter/instanceIamBinding:InstanceIamBinding default projects/{{project}}/locations/{{region}}/instances/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:securitycenter/instanceIamBinding:InstanceIamBinding default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:securitycenter/instanceIamBinding:InstanceIamBinding default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:securitycenter/instanceIamBinding:InstanceIamBinding default {{name}}
/// ```
///
pub mod instance_iam_binding {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceIamBindingArgs {
        #[builder(into, default)]
        pub condition: pulumi_wasm_rust::Output<
            Option<super::super::types::securitycenter::InstanceIamBindingCondition>,
        >,
        #[builder(into)]
        pub members: pulumi_wasm_rust::Output<Vec<String>>,
        /// The ID of the instance or a fully qualified identifier for the instance.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The region of the Data Fusion instance.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into)]
        pub role: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct InstanceIamBindingResult {
        pub condition: pulumi_wasm_rust::Output<
            Option<super::super::types::securitycenter::InstanceIamBindingCondition>,
        >,
        pub etag: pulumi_wasm_rust::Output<String>,
        pub members: pulumi_wasm_rust::Output<Vec<String>>,
        /// The ID of the instance or a fully qualified identifier for the instance.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The region of the Data Fusion instance.
        pub region: pulumi_wasm_rust::Output<String>,
        pub role: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: InstanceIamBindingArgs) -> InstanceIamBindingResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let condition_binding = args.condition.get_inner();
        let members_binding = args.members.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let region_binding = args.region.get_inner();
        let role_binding = args.role.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:securitycenter/instanceIamBinding:InstanceIamBinding".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "condition".into(),
                    value: &condition_binding,
                },
                register_interface::ObjectField {
                    name: "members".into(),
                    value: &members_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "role".into(),
                    value: &role_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "condition".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "members".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
                register_interface::ResultField {
                    name: "role".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        InstanceIamBindingResult {
            condition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("condition").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            members: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("members").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
            role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("role").unwrap(),
            ),
        }
    }
}
