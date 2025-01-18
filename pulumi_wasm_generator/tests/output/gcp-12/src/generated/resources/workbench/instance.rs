/// A Workbench instance.
///
///
///
/// ## Example Usage
///
/// ### Workbench Instance Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let instance = instance::create(
///         "instance",
///         InstanceArgs::builder()
///             .location("us-west1-a")
///             .name("workbench-instance")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Workbench Instance Basic Container
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let instance = instance::create(
///         "instance",
///         InstanceArgs::builder()
///             .gce_setup(
///                 InstanceGceSetup::builder()
///                     .containerImage(
///                         InstanceGceSetupContainerImage::builder()
///                             .repository(
///                                 "us-docker.pkg.dev/deeplearning-platform-release/gcr.io/base-cu113.py310",
///                             )
///                             .tag("latest")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .location("us-west1-a")
///             .name("workbench-instance")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Workbench Instance Basic Gpu
///
///
/// ```yaml
/// resources:
///   instance:
///     type: gcp:workbench:Instance
///     properties:
///       name: workbench-instance
///       location: us-central1-a
///       gceSetup:
///         machineType: n1-standard-1
///         acceleratorConfigs:
///           - type: NVIDIA_TESLA_T4
///             coreCount: 1
///         vmImage:
///           project: cloud-notebooks-managed
///           family: workbench-instances
/// ```
/// ### Workbench Instance Labels Stopped
///
///
/// ```yaml
/// resources:
///   instance:
///     type: gcp:workbench:Instance
///     properties:
///       name: workbench-instance
///       location: us-central1-a
///       gceSetup:
///         machineType: e2-standard-4
///         shieldedInstanceConfig:
///           enableSecureBoot: false
///           enableVtpm: false
///           enableIntegrityMonitoring: false
///         serviceAccounts:
///           - email: my@service-account.com
///         metadata:
///           terraform: 'true'
///       labels:
///         k: val
///       desiredState: STOPPED
/// ```
/// ### Workbench Instance Full
///
///
/// ```yaml
/// resources:
///   myNetwork:
///     type: gcp:compute:Network
///     name: my_network
///     properties:
///       name: wbi-test-default
///       autoCreateSubnetworks: false
///   mySubnetwork:
///     type: gcp:compute:Subnetwork
///     name: my_subnetwork
///     properties:
///       name: wbi-test-default
///       network: ${myNetwork.id}
///       region: us-central1
///       ipCidrRange: 10.0.1.0/24
///   static:
///     type: gcp:compute:Address
///     properties:
///       name: wbi-test-default
///   actAsPermission:
///     type: gcp:serviceaccount:IAMBinding
///     name: act_as_permission
///     properties:
///       serviceAccountId: projects/my-project-name/serviceAccounts/my@service-account.com
///       role: roles/iam.serviceAccountUser
///       members:
///         - user:example@example.com
///   instance:
///     type: gcp:workbench:Instance
///     properties:
///       name: workbench-instance
///       location: us-central1-a
///       gceSetup:
///         machineType: n1-standard-4
///         acceleratorConfigs:
///           - type: NVIDIA_TESLA_T4
///             coreCount: 1
///         shieldedInstanceConfig:
///           enableSecureBoot: true
///           enableVtpm: true
///           enableIntegrityMonitoring: true
///         disablePublicIp: false
///         serviceAccounts:
///           - email: my@service-account.com
///         bootDisk:
///           diskSizeGb: 310
///           diskType: PD_SSD
///           diskEncryption: CMEK
///           kmsKey: my-crypto-key
///         dataDisks:
///           diskSizeGb: 330
///           diskType: PD_SSD
///           diskEncryption: CMEK
///           kmsKey: my-crypto-key
///         networkInterfaces:
///           - network: ${myNetwork.id}
///             subnet: ${mySubnetwork.id}
///             nicType: GVNIC
///             accessConfigs:
///               - externalIp: ${static.address}
///         metadata:
///           terraform: 'true'
///         enableIpForwarding: true
///         tags:
///           - abc
///           - def
///       disableProxyAccess: 'true'
///       instanceOwners:
///         - example@example.com
///       labels:
///         k: val
///       desiredState: ACTIVE
/// ```
///
/// ## Import
///
/// Instance can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/instances/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, Instance can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:workbench/instance:Instance default projects/{{project}}/locations/{{location}}/instances/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:workbench/instance:Instance default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:workbench/instance:Instance default {{location}}/{{name}}
/// ```
///
pub mod instance {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceArgs {
        /// Desired state of the Workbench Instance. Set this field to `ACTIVE` to start the Instance, and `STOPPED` to stop the Instance.
        #[builder(into, default)]
        pub desired_state: pulumi_wasm_rust::Output<Option<String>>,
        /// Optional. If true, the workbench instance will not register with the proxy.
        #[builder(into, default)]
        pub disable_proxy_access: pulumi_wasm_rust::Output<Option<bool>>,
        /// The definition of how to configure a VM instance outside of Resources and Identity.
        /// Structure is documented below.
        #[builder(into, default)]
        pub gce_setup: pulumi_wasm_rust::Output<
            Option<super::super::types::workbench::InstanceGceSetup>,
        >,
        /// Required. User-defined unique ID of this instance.
        #[builder(into, default)]
        pub instance_id: pulumi_wasm_rust::Output<Option<String>>,
        /// 'Optional. Input only. The owner of this instance after creation. Format:
        /// `alias@example.com` Currently supports one owner only. If not specified, all of
        /// the service account users of your VM instance''s service account can use the instance.
        /// If specified, sets the access mode to `Single user`. For more details, see
        /// https://cloud.google.com/vertex-ai/docs/workbench/instances/manage-access-jupyterlab'
        #[builder(into, default)]
        pub instance_owners: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Optional. Labels to apply to this instance. These can be later modified
        /// by the UpdateInstance method.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Part of `parent`. See documentation of `projectsId`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of this workbench instance. Format: `projects/{project_id}/locations/{location}/instances/{instance_id}`
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct InstanceResult {
        /// An RFC3339 timestamp in UTC time. This in the format of yyyy-MM-ddTHH:mm:ss.SSSZ.
        /// The milliseconds portion (".SSS") is optional.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Output only. Email address of entity that sent original CreateInstance request.
        pub creator: pulumi_wasm_rust::Output<String>,
        /// Desired state of the Workbench Instance. Set this field to `ACTIVE` to start the Instance, and `STOPPED` to stop the Instance.
        pub desired_state: pulumi_wasm_rust::Output<Option<String>>,
        /// Optional. If true, the workbench instance will not register with the proxy.
        pub disable_proxy_access: pulumi_wasm_rust::Output<Option<bool>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The definition of how to configure a VM instance outside of Resources and Identity.
        /// Structure is documented below.
        pub gce_setup: pulumi_wasm_rust::Output<
            super::super::types::workbench::InstanceGceSetup,
        >,
        /// 'Output only. Additional information about instance health. Example:
        /// healthInfo": { "docker_proxy_agent_status": "1", "docker_status": "1", "jupyterlab_api_status":
        /// "-1", "jupyterlab_status": "-1", "updated": "2020-10-18 09:40:03.573409" }'
        pub health_infos: pulumi_wasm_rust::Output<
            Vec<super::super::types::workbench::InstanceHealthInfo>,
        >,
        /// Output only. Instance health_state.
        pub health_state: pulumi_wasm_rust::Output<String>,
        /// Required. User-defined unique ID of this instance.
        pub instance_id: pulumi_wasm_rust::Output<Option<String>>,
        /// 'Optional. Input only. The owner of this instance after creation. Format:
        /// `alias@example.com` Currently supports one owner only. If not specified, all of
        /// the service account users of your VM instance''s service account can use the instance.
        /// If specified, sets the access mode to `Single user`. For more details, see
        /// https://cloud.google.com/vertex-ai/docs/workbench/instances/manage-access-jupyterlab'
        pub instance_owners: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Optional. Labels to apply to this instance. These can be later modified
        /// by the UpdateInstance method.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Part of `parent`. See documentation of `projectsId`.
        ///
        ///
        /// - - -
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of this workbench instance. Format: `projects/{project_id}/locations/{location}/instances/{instance_id}`
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// Output only. The proxy endpoint that is used to access the Jupyter notebook.
        pub proxy_uri: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// (Output)
        /// Output only. The state of this instance upgrade history entry.
        pub state: pulumi_wasm_rust::Output<String>,
        /// An RFC3339 timestamp in UTC time. This in the format of yyyy-MM-ddTHH:mm:ss.SSSZ.
        /// The milliseconds portion (".SSS") is optional.
        pub update_time: pulumi_wasm_rust::Output<String>,
        /// Output only. The upgrade history of this instance.
        /// Structure is documented below.
        pub upgrade_histories: pulumi_wasm_rust::Output<
            Vec<super::super::types::workbench::InstanceUpgradeHistory>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: InstanceArgs) -> InstanceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let desired_state_binding = args.desired_state.get_inner();
        let disable_proxy_access_binding = args.disable_proxy_access.get_inner();
        let gce_setup_binding = args.gce_setup.get_inner();
        let instance_id_binding = args.instance_id.get_inner();
        let instance_owners_binding = args.instance_owners.get_inner();
        let labels_binding = args.labels.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:workbench/instance:Instance".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "desiredState".into(),
                    value: &desired_state_binding,
                },
                register_interface::ObjectField {
                    name: "disableProxyAccess".into(),
                    value: &disable_proxy_access_binding,
                },
                register_interface::ObjectField {
                    name: "gceSetup".into(),
                    value: &gce_setup_binding,
                },
                register_interface::ObjectField {
                    name: "instanceId".into(),
                    value: &instance_id_binding,
                },
                register_interface::ObjectField {
                    name: "instanceOwners".into(),
                    value: &instance_owners_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "creator".into(),
                },
                register_interface::ResultField {
                    name: "desiredState".into(),
                },
                register_interface::ResultField {
                    name: "disableProxyAccess".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "gceSetup".into(),
                },
                register_interface::ResultField {
                    name: "healthInfos".into(),
                },
                register_interface::ResultField {
                    name: "healthState".into(),
                },
                register_interface::ResultField {
                    name: "instanceId".into(),
                },
                register_interface::ResultField {
                    name: "instanceOwners".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "proxyUri".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
                register_interface::ResultField {
                    name: "upgradeHistories".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        InstanceResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            creator: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creator").unwrap(),
            ),
            desired_state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("desiredState").unwrap(),
            ),
            disable_proxy_access: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disableProxyAccess").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            gce_setup: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gceSetup").unwrap(),
            ),
            health_infos: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("healthInfos").unwrap(),
            ),
            health_state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("healthState").unwrap(),
            ),
            instance_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceId").unwrap(),
            ),
            instance_owners: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceOwners").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            proxy_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("proxyUri").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
            upgrade_histories: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("upgradeHistories").unwrap(),
            ),
        }
    }
}
