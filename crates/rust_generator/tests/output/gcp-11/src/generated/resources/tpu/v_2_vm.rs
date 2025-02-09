/// ## Example Usage
///
/// ### Tpu V2 Vm Basic
///
///
/// ```yaml
/// resources:
///   tpu:
///     type: gcp:tpu:V2Vm
///     properties:
///       name: test-tpu
///       zone: us-central1-c
///       runtimeVersion: tpu-vm-tf-2.13.0
/// variables:
///   available:
///     fn::invoke:
///       function: gcp:tpu:getV2RuntimeVersions
///       arguments: {}
/// ```
/// ### Tpu V2 Vm Full
///
///
/// ```yaml
/// resources:
///   tpu:
///     type: gcp:tpu:V2Vm
///     properties:
///       name: test-tpu
///       zone: us-central1-c
///       description: Text description of the TPU.
///       runtimeVersion: tpu-vm-tf-2.13.0
///       acceleratorConfig:
///         type: V2
///         topology: 2x2
///       cidrBlock: 10.0.0.0/29
///       networkConfig:
///         canIpForward: true
///         enableExternalIps: true
///         network: ${network.id}
///         subnetwork: ${subnet.id}
///         queueCount: 32
///       schedulingConfig:
///         preemptible: true
///       shieldedInstanceConfig:
///         enableSecureBoot: true
///       serviceAccount:
///         email: ${sa.email}
///         scopes:
///           - https://www.googleapis.com/auth/cloud-platform
///       dataDisks:
///         - sourceDisk: ${disk.id}
///           mode: READ_ONLY
///       labels:
///         foo: bar
///       metadata:
///         foo: bar
///       tags:
///         - foo
///     options:
///       dependsOn:
///         - ${wait60Seconds}
///   subnet:
///     type: gcp:compute:Subnetwork
///     properties:
///       name: tpu-subnet
///       ipCidrRange: 10.0.0.0/16
///       region: us-central1
///       network: ${network.id}
///   network:
///     type: gcp:compute:Network
///     properties:
///       name: tpu-net
///       autoCreateSubnetworks: false
///   sa:
///     type: gcp:serviceaccount:Account
///     properties:
///       accountId: tpu-sa
///       displayName: Test TPU VM
///   disk:
///     type: gcp:compute:Disk
///     properties:
///       name: tpu-disk
///       image: debian-cloud/debian-11
///       size: 10
///       type: pd-ssd
///       zone: us-central1-c
///   # Wait after service account creation to limit eventual consistency errors.
///   wait60Seconds:
///     type: time:sleep
///     name: wait_60_seconds
///     properties:
///       createDuration: 60s
///     options:
///       dependsOn:
///         - ${sa}
/// variables:
///   available:
///     fn::invoke:
///       function: gcp:tpu:getV2RuntimeVersions
///       arguments: {}
///   availableGetV2AcceleratorTypes:
///     fn::invoke:
///       function: gcp:tpu:getV2AcceleratorTypes
///       arguments: {}
/// ```
///
/// ## Import
///
/// Vm can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{zone}}/nodes/{{name}}`
///
/// * `{{project}}/{{zone}}/{{name}}`
///
/// * `{{zone}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Vm can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:tpu/v2Vm:V2Vm default projects/{{project}}/locations/{{zone}}/nodes/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:tpu/v2Vm:V2Vm default {{project}}/{{zone}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:tpu/v2Vm:V2Vm default {{zone}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:tpu/v2Vm:V2Vm default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod v_2_vm {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct V2VmArgs {
        /// The AccleratorConfig for the TPU Node. `accelerator_config` cannot be used at the same time
        /// as `accelerator_type`. If neither is specified, `accelerator_type` defaults to 'v2-8'.
        /// Structure is documented below.
        #[builder(into, default)]
        pub accelerator_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::tpu::V2VmAcceleratorConfig>,
        >,
        /// TPU accelerator type for the TPU. `accelerator_type` cannot be used at the same time as
        /// `accelerator_config`. If neither is specified, `accelerator_type` defaults to 'v2-8'.
        #[builder(into, default)]
        pub accelerator_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The CIDR block that the TPU node will use when selecting an IP address. This CIDR block must
        /// be a /29 block; the Compute Engine networks API forbids a smaller block, and using a larger
        /// block would be wasteful (a node can only consume one IP address). Errors will occur if the
        /// CIDR block has already been used for a currently existing TPU node, the CIDR block conflicts
        /// with any subnetworks in the user's provided network, or the provided network is peered with
        /// another network that is using that CIDR block.
        #[builder(into, default)]
        pub cidr_block: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The additional data disks for the Node.
        /// Structure is documented below.
        #[builder(into, default)]
        pub data_disks: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::tpu::V2VmDataDisk>>,
        >,
        /// Text description of the TPU.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Resource labels to represent user-provided metadata.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Custom metadata to apply to the TPU Node. Can set startup-script and shutdown-script.
        #[builder(into, default)]
        pub metadata: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The immutable name of the TPU.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Network configurations for the TPU node.
        /// Structure is documented below.
        #[builder(into, default)]
        pub network_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::tpu::V2VmNetworkConfig>,
        >,
        /// Repeated network configurations for the TPU node. This field is used to specify multiple
        /// network configs for the TPU node.
        /// Structure is documented below.
        #[builder(into, default)]
        pub network_configs: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::tpu::V2VmNetworkConfig>>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Runtime version for the TPU.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub runtime_version: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The scheduling options for this node.
        /// Structure is documented below.
        #[builder(into, default)]
        pub scheduling_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::tpu::V2VmSchedulingConfig>,
        >,
        /// The Google Cloud Platform Service Account to be used by the TPU node VMs. If None is
        /// specified, the default compute service account will be used.
        /// Structure is documented below.
        #[builder(into, default)]
        pub service_account: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::tpu::V2VmServiceAccount>,
        >,
        /// Shielded Instance options.
        /// Structure is documented below.
        #[builder(into, default)]
        pub shielded_instance_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::tpu::V2VmShieldedInstanceConfig>,
        >,
        /// Tags to apply to the TPU Node. Tags are used to identify valid sources or targets for network firewalls.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The GCP location for the TPU. If it is not provided, the provider zone is used.
        #[builder(into, default)]
        pub zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct V2VmResult {
        /// The AccleratorConfig for the TPU Node. `accelerator_config` cannot be used at the same time
        /// as `accelerator_type`. If neither is specified, `accelerator_type` defaults to 'v2-8'.
        /// Structure is documented below.
        pub accelerator_config: pulumi_gestalt_rust::Output<
            super::super::types::tpu::V2VmAcceleratorConfig,
        >,
        /// TPU accelerator type for the TPU. `accelerator_type` cannot be used at the same time as
        /// `accelerator_config`. If neither is specified, `accelerator_type` defaults to 'v2-8'.
        pub accelerator_type: pulumi_gestalt_rust::Output<String>,
        /// The API version that created this Node.
        pub api_version: pulumi_gestalt_rust::Output<String>,
        /// The CIDR block that the TPU node will use when selecting an IP address. This CIDR block must
        /// be a /29 block; the Compute Engine networks API forbids a smaller block, and using a larger
        /// block would be wasteful (a node can only consume one IP address). Errors will occur if the
        /// CIDR block has already been used for a currently existing TPU node, the CIDR block conflicts
        /// with any subnetworks in the user's provided network, or the provided network is peered with
        /// another network that is using that CIDR block.
        pub cidr_block: pulumi_gestalt_rust::Output<String>,
        /// The additional data disks for the Node.
        /// Structure is documented below.
        pub data_disks: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::tpu::V2VmDataDisk>>,
        >,
        /// Text description of the TPU.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The health status of the TPU node.
        pub health: pulumi_gestalt_rust::Output<String>,
        /// If this field is populated, it contains a description of why the TPU Node is unhealthy.
        pub health_description: pulumi_gestalt_rust::Output<String>,
        /// Resource labels to represent user-provided metadata.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Custom metadata to apply to the TPU Node. Can set startup-script and shutdown-script.
        pub metadata: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Whether the Node belongs to a Multislice group.
        pub multislice_node: pulumi_gestalt_rust::Output<bool>,
        /// The immutable name of the TPU.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Network configurations for the TPU node.
        /// Structure is documented below.
        pub network_config: pulumi_gestalt_rust::Output<
            super::super::types::tpu::V2VmNetworkConfig,
        >,
        /// Repeated network configurations for the TPU node. This field is used to specify multiple
        /// network configs for the TPU node.
        /// Structure is documented below.
        pub network_configs: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::tpu::V2VmNetworkConfig>>,
        >,
        /// The network endpoints where TPU workers can be accessed and sent work. It is recommended that
        /// runtime clients of the node reach out to the 0th entry in this map first.
        /// Structure is documented below.
        pub network_endpoints: pulumi_gestalt_rust::Output<
            Vec<super::super::types::tpu::V2VmNetworkEndpoint>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The qualified name of the QueuedResource that requested this Node.
        pub queued_resource: pulumi_gestalt_rust::Output<String>,
        /// Runtime version for the TPU.
        ///
        ///
        /// - - -
        pub runtime_version: pulumi_gestalt_rust::Output<String>,
        /// The scheduling options for this node.
        /// Structure is documented below.
        pub scheduling_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::tpu::V2VmSchedulingConfig>,
        >,
        /// The Google Cloud Platform Service Account to be used by the TPU node VMs. If None is
        /// specified, the default compute service account will be used.
        /// Structure is documented below.
        pub service_account: pulumi_gestalt_rust::Output<
            super::super::types::tpu::V2VmServiceAccount,
        >,
        /// Shielded Instance options.
        /// Structure is documented below.
        pub shielded_instance_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::tpu::V2VmShieldedInstanceConfig>,
        >,
        /// The current state for the TPU Node.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// The Symptoms that have occurred to the TPU Node.
        /// Structure is documented below.
        pub symptoms: pulumi_gestalt_rust::Output<
            Vec<super::super::types::tpu::V2VmSymptom>,
        >,
        /// Tags to apply to the TPU Node. Tags are used to identify valid sources or targets for network firewalls.
        pub tags: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The GCP location for the TPU. If it is not provided, the provider zone is used.
        pub zone: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: V2VmArgs,
    ) -> V2VmResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let accelerator_config_binding_1 = args.accelerator_config.get_output(context);
        let accelerator_config_binding = accelerator_config_binding_1.get_inner();
        let accelerator_type_binding_1 = args.accelerator_type.get_output(context);
        let accelerator_type_binding = accelerator_type_binding_1.get_inner();
        let cidr_block_binding_1 = args.cidr_block.get_output(context);
        let cidr_block_binding = cidr_block_binding_1.get_inner();
        let data_disks_binding_1 = args.data_disks.get_output(context);
        let data_disks_binding = data_disks_binding_1.get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let labels_binding_1 = args.labels.get_output(context);
        let labels_binding = labels_binding_1.get_inner();
        let metadata_binding_1 = args.metadata.get_output(context);
        let metadata_binding = metadata_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let network_config_binding_1 = args.network_config.get_output(context);
        let network_config_binding = network_config_binding_1.get_inner();
        let network_configs_binding_1 = args.network_configs.get_output(context);
        let network_configs_binding = network_configs_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let runtime_version_binding_1 = args.runtime_version.get_output(context);
        let runtime_version_binding = runtime_version_binding_1.get_inner();
        let scheduling_config_binding_1 = args.scheduling_config.get_output(context);
        let scheduling_config_binding = scheduling_config_binding_1.get_inner();
        let service_account_binding_1 = args.service_account.get_output(context);
        let service_account_binding = service_account_binding_1.get_inner();
        let shielded_instance_config_binding_1 = args
            .shielded_instance_config
            .get_output(context);
        let shielded_instance_config_binding = shielded_instance_config_binding_1
            .get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let zone_binding_1 = args.zone.get_output(context);
        let zone_binding = zone_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:tpu/v2Vm:V2Vm".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "acceleratorConfig".into(),
                    value: &accelerator_config_binding,
                },
                register_interface::ObjectField {
                    name: "acceleratorType".into(),
                    value: &accelerator_type_binding,
                },
                register_interface::ObjectField {
                    name: "cidrBlock".into(),
                    value: &cidr_block_binding,
                },
                register_interface::ObjectField {
                    name: "dataDisks".into(),
                    value: &data_disks_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "metadata".into(),
                    value: &metadata_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "networkConfig".into(),
                    value: &network_config_binding,
                },
                register_interface::ObjectField {
                    name: "networkConfigs".into(),
                    value: &network_configs_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "runtimeVersion".into(),
                    value: &runtime_version_binding,
                },
                register_interface::ObjectField {
                    name: "schedulingConfig".into(),
                    value: &scheduling_config_binding,
                },
                register_interface::ObjectField {
                    name: "serviceAccount".into(),
                    value: &service_account_binding,
                },
                register_interface::ObjectField {
                    name: "shieldedInstanceConfig".into(),
                    value: &shielded_instance_config_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "zone".into(),
                    value: &zone_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        V2VmResult {
            accelerator_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("acceleratorConfig"),
            ),
            accelerator_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("acceleratorType"),
            ),
            api_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("apiVersion"),
            ),
            cidr_block: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cidrBlock"),
            ),
            data_disks: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dataDisks"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            health: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("health"),
            ),
            health_description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("healthDescription"),
            ),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            metadata: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("metadata"),
            ),
            multislice_node: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("multisliceNode"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            network_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkConfig"),
            ),
            network_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkConfigs"),
            ),
            network_endpoints: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkEndpoints"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            queued_resource: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("queuedResource"),
            ),
            runtime_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("runtimeVersion"),
            ),
            scheduling_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("schedulingConfig"),
            ),
            service_account: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceAccount"),
            ),
            shielded_instance_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("shieldedInstanceConfig"),
            ),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            symptoms: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("symptoms"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            zone: pulumi_gestalt_rust::__private::into_domain(o.extract_field("zone")),
        }
    }
}
