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
/// $ pulumi import gcp:securitycenter/instanceIamMember:InstanceIamMember default projects/{{project}}/locations/{{region}}/instances/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:securitycenter/instanceIamMember:InstanceIamMember default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:securitycenter/instanceIamMember:InstanceIamMember default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:securitycenter/instanceIamMember:InstanceIamMember default {{name}}
/// ```
///
pub mod instance_iam_member {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceIamMemberArgs {
        #[builder(into, default)]
        pub condition: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::securitycenter::InstanceIamMemberCondition>,
        >,
        #[builder(into)]
        pub member: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the instance or a fully qualified identifier for the instance.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The region of the Data Fusion instance.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into)]
        pub role: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct InstanceIamMemberResult {
        pub condition: pulumi_wasm_rust::Output<
            Option<super::super::types::securitycenter::InstanceIamMemberCondition>,
        >,
        pub etag: pulumi_wasm_rust::Output<String>,
        pub member: pulumi_wasm_rust::Output<String>,
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
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: InstanceIamMemberArgs,
    ) -> InstanceIamMemberResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let condition_binding = args.condition.get_output(context).get_inner();
        let member_binding = args.member.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let role_binding = args.role.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:securitycenter/instanceIamMember:InstanceIamMember".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "condition".into(),
                    value: &condition_binding,
                },
                register_interface::ObjectField {
                    name: "member".into(),
                    value: &member_binding,
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        InstanceIamMemberResult {
            condition: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("condition"),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(o.extract_field("etag")),
            member: pulumi_wasm_rust::__private::into_domain(o.extract_field("member")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            region: pulumi_wasm_rust::__private::into_domain(o.extract_field("region")),
            role: pulumi_wasm_rust::__private::into_domain(o.extract_field("role")),
        }
    }
}
