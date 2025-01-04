/// The Google Compute Engine Instance Group Manager API creates and manages pools
/// of homogeneous Compute Engine virtual machine instances from a common instance
/// template. For more information, see [the official documentation](https://cloud.google.com/compute/docs/instance-groups/manager)
/// and [API](https://cloud.google.com/compute/docs/reference/latest/instanceGroupManagers)
///
/// > **Note:** Use [gcp.compute.RegionInstanceGroupManager](https://www.terraform.io/docs/providers/google/r/compute_region_instance_group_manager.html) to create a regional (multi-zone) instance group manager.
///
/// ## Example Usage
///
/// ### With Top Level Instance Template (`Google` Provider)
///
/// ```yaml
/// resources:
///   autohealing:
///     type: gcp:compute:HealthCheck
///     properties:
///       name: autohealing-health-check
///       checkIntervalSec: 5
///       timeoutSec: 5
///       healthyThreshold: 2
///       unhealthyThreshold: 10 # 50 seconds
///       httpHealthCheck:
///         requestPath: /healthz
///         port: '8080'
///   appserver:
///     type: gcp:compute:InstanceGroupManager
///     properties:
///       name: appserver-igm
///       baseInstanceName: app
///       zone: us-central1-a
///       versions:
///         - instanceTemplate: ${appserverGoogleComputeInstanceTemplate.selfLinkUnique}
///       allInstancesConfig:
///         metadata:
///           metadata_key: metadata_value
///         labels:
///           label_key: label_value
///       targetPools:
///         - ${appserverGoogleComputeTargetPool.id}
///       targetSize: 2
///       namedPorts:
///         - name: customhttp
///           port: 8888
///       autoHealingPolicies:
///         healthCheck: ${autohealing.id}
///         initialDelaySec: 300
/// ```
///
///
/// ### With Multiple Versions (`Google-Beta` Provider)
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let appserver = instance_group_manager::create(
///         "appserver",
///         InstanceGroupManagerArgs::builder()
///             .base_instance_name("app")
///             .name("appserver-igm")
///             .target_size(5)
///             .versions(
///                 vec![
///                     InstanceGroupManagerVersion::builder()
///                     .instanceTemplate("${appserverGoogleComputeInstanceTemplate.selfLinkUnique}")
///                     .name("appserver").build_struct(),
///                     InstanceGroupManagerVersion::builder()
///                     .instanceTemplate("${[\"appserver-canary\"].selfLinkUnique}")
///                     .name("appserver-canary")
///                     .targetSize(InstanceGroupManagerVersionTargetSize::builder().fixed(1)
///                     .build_struct()).build_struct(),
///                 ],
///             )
///             .zone("us-central1-a")
///             .build_struct(),
///     );
/// }
/// ```
///
///
/// ### With Standby Policy (`Google-Beta` Provider)
/// ```yaml
/// resources:
///   igm-sr:
///     type: gcp:compute:InstanceGroupManager
///     properties:
///       name: tf-sr-igm
///       baseInstanceName: tf-sr-igm-instance
///       zone: us-central1-a
///       targetSize: 5
///       versions:
///         - instanceTemplate: ${["sr-igm"].selfLink}
///           name: primary
///       standbyPolicy:
///         initialDelaySec: 30
///         mode: MANUAL
///       targetSuspendedSize: 2
///       targetStoppedSize: 1
/// ```
///
/// ## Import
///
/// Instance group managers can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/zones/{{zone}}/instanceGroupManagers/{{name}}`
///
/// * `{{project}}/{{zone}}/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, instance group managers can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/instanceGroupManager:InstanceGroupManager default projects/{{project}}/zones/{{zone}}/instanceGroupManagers/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/instanceGroupManager:InstanceGroupManager default {{project}}/{{zone}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/instanceGroupManager:InstanceGroupManager default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/instanceGroupManager:InstanceGroupManager default {{name}}
/// ```
///
pub mod instance_group_manager {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceGroupManagerArgs {
        /// Properties to set on all instances in the group. After setting
        /// allInstancesConfig on the group, you must update the group's instances to
        /// apply the configuration.
        #[builder(into, default)]
        pub all_instances_config: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::InstanceGroupManagerAllInstancesConfig>,
        >,
        /// The autohealing policies for this managed instance
        /// group. You can specify only one value. Structure is documented below. For more information, see the [official documentation](https://cloud.google.com/compute/docs/instance-groups/creating-groups-of-managed-instances#monitoring_groups).
        #[builder(into, default)]
        pub auto_healing_policies: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::InstanceGroupManagerAutoHealingPolicies>,
        >,
        /// The base instance name to use for
        /// instances in this group. The value must be a valid
        /// [RFC1035](https://www.ietf.org/rfc/rfc1035.txt) name. Supported characters
        /// are lowercase letters, numbers, and hyphens (-). Instances are named by
        /// appending a hyphen and a random four-character string to the base instance
        /// name.
        #[builder(into)]
        pub base_instance_name: pulumi_wasm_rust::Output<String>,
        /// An optional textual description of the instance
        /// group manager.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The instance lifecycle policy for this managed instance group.
        #[builder(into, default)]
        pub instance_lifecycle_policy: pulumi_wasm_rust::Output<
            Option<
                super::super::types::compute::InstanceGroupManagerInstanceLifecyclePolicy,
            >,
        >,
        /// Pagination behavior of the `listManagedInstances` API
        /// method for this managed instance group. Valid values are: `PAGELESS`, `PAGINATED`.
        /// If `PAGELESS` (default), Pagination is disabled for the group's `listManagedInstances` API method.
        /// `maxResults` and `pageToken` query parameters are ignored and all instances are returned in a single
        /// response. If `PAGINATED`, pagination is enabled, `maxResults` and `pageToken` query parameters are
        /// respected.
        #[builder(into, default)]
        pub list_managed_instances_results: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the instance group manager. Must be 1-63
        /// characters long and comply with
        /// [RFC1035](https://www.ietf.org/rfc/rfc1035.txt). Supported characters
        /// include lowercase letters, numbers, and hyphens.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The named port configuration. See the section below
        /// for details on configuration.
        #[builder(into, default)]
        pub named_ports: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::compute::InstanceGroupManagerNamedPort>>,
        >,
        /// Input only additional params for instance group manager creation. Structure is documented below. For more information, see [API](https://cloud.google.com/compute/docs/reference/rest/beta/instanceGroupManagers/insert).
        ///
        /// - - -
        #[builder(into, default)]
        pub params: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::InstanceGroupManagerParams>,
        >,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The standby policy for stopped and suspended instances. Structure is documented below. For more information, see the [official documentation](https://cloud.google.com/compute/docs/instance-groups/suspended-and-stopped-vms-in-mig) and [API](https://cloud.google.com/compute/docs/reference/rest/beta/regionInstanceGroupManagers/patch)
        #[builder(into, default)]
        pub standby_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::InstanceGroupManagerStandbyPolicy>,
        >,
        /// Disks created on the instances that will be preserved on instance delete, update, etc. Structure is documented below. For more information see the [official documentation](https://cloud.google.com/compute/docs/instance-groups/configuring-stateful-disks-in-migs).
        #[builder(into, default)]
        pub stateful_disks: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::compute::InstanceGroupManagerStatefulDisk>>,
        >,
        /// External network IPs assigned to the instances that will be preserved on instance delete, update, etc. This map is keyed with the network interface name. Structure is documented below.
        #[builder(into, default)]
        pub stateful_external_ips: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::compute::InstanceGroupManagerStatefulExternalIp>,
            >,
        >,
        /// Internal network IPs assigned to the instances that will be preserved on instance delete, update, etc. This map is keyed with the network interface name. Structure is documented below.
        #[builder(into, default)]
        pub stateful_internal_ips: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::compute::InstanceGroupManagerStatefulInternalIp>,
            >,
        >,
        /// The full URL of all target pools to which new
        /// instances in the group are added. Updating the target pools attribute does
        /// not affect existing instances.
        #[builder(into, default)]
        pub target_pools: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The target number of running instances for this managed instance group. This value should always be explicitly set
        /// unless this resource is attached to an autoscaler, in which case it should never be set. Defaults to 0.
        #[builder(into, default)]
        pub target_size: pulumi_wasm_rust::Output<Option<i32>>,
        /// The target number of stopped instances for this managed instance group.
        #[builder(into, default)]
        pub target_stopped_size: pulumi_wasm_rust::Output<Option<i32>>,
        /// The target number of suspended instances for this managed instance group.
        #[builder(into, default)]
        pub target_suspended_size: pulumi_wasm_rust::Output<Option<i32>>,
        /// The update policy for this managed instance group. Structure is documented below. For more information, see the [official documentation](https://cloud.google.com/compute/docs/instance-groups/updating-managed-instance-groups) and [API](https://cloud.google.com/compute/docs/reference/rest/v1/instanceGroupManagers/patch).
        #[builder(into, default)]
        pub update_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::InstanceGroupManagerUpdatePolicy>,
        >,
        /// Application versions managed by this instance group. Each
        /// version deals with a specific instance template, allowing canary release scenarios.
        /// Structure is documented below.
        #[builder(into)]
        pub versions: pulumi_wasm_rust::Output<
            Vec<super::super::types::compute::InstanceGroupManagerVersion>,
        >,
        /// Whether to wait for all instances to be created/updated before
        /// returning. Note that if this is set to true and the operation does not succeed, this provider will
        /// continue trying until it times out.
        #[builder(into, default)]
        pub wait_for_instances: pulumi_wasm_rust::Output<Option<bool>>,
        /// When used with `wait_for_instances` it specifies the status to wait for.
        /// When `STABLE` is specified this resource will wait until the instances are stable before returning. When `UPDATED` is
        /// set, it will wait for the version target to be reached and any per instance configs to be effective as well as all
        /// instances to be stable before returning. The possible values are `STABLE` and `UPDATED`
        #[builder(into, default)]
        pub wait_for_instances_status: pulumi_wasm_rust::Output<Option<String>>,
        /// The zone that instances in this group should be created
        /// in.
        ///
        /// - - -
        #[builder(into, default)]
        pub zone: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct InstanceGroupManagerResult {
        /// Properties to set on all instances in the group. After setting
        /// allInstancesConfig on the group, you must update the group's instances to
        /// apply the configuration.
        pub all_instances_config: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::InstanceGroupManagerAllInstancesConfig>,
        >,
        /// The autohealing policies for this managed instance
        /// group. You can specify only one value. Structure is documented below. For more information, see the [official documentation](https://cloud.google.com/compute/docs/instance-groups/creating-groups-of-managed-instances#monitoring_groups).
        pub auto_healing_policies: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::InstanceGroupManagerAutoHealingPolicies>,
        >,
        /// The base instance name to use for
        /// instances in this group. The value must be a valid
        /// [RFC1035](https://www.ietf.org/rfc/rfc1035.txt) name. Supported characters
        /// are lowercase letters, numbers, and hyphens (-). Instances are named by
        /// appending a hyphen and a random four-character string to the base instance
        /// name.
        pub base_instance_name: pulumi_wasm_rust::Output<String>,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_wasm_rust::Output<String>,
        /// An optional textual description of the instance
        /// group manager.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The fingerprint of the instance group manager.
        pub fingerprint: pulumi_wasm_rust::Output<String>,
        /// The full URL of the instance group created by the manager.
        pub instance_group: pulumi_wasm_rust::Output<String>,
        /// The unique identifier number for the resource. This identifier is defined by the server.
        pub instance_group_manager_id: pulumi_wasm_rust::Output<i32>,
        /// The instance lifecycle policy for this managed instance group.
        pub instance_lifecycle_policy: pulumi_wasm_rust::Output<
            super::super::types::compute::InstanceGroupManagerInstanceLifecyclePolicy,
        >,
        /// Pagination behavior of the `listManagedInstances` API
        /// method for this managed instance group. Valid values are: `PAGELESS`, `PAGINATED`.
        /// If `PAGELESS` (default), Pagination is disabled for the group's `listManagedInstances` API method.
        /// `maxResults` and `pageToken` query parameters are ignored and all instances are returned in a single
        /// response. If `PAGINATED`, pagination is enabled, `maxResults` and `pageToken` query parameters are
        /// respected.
        pub list_managed_instances_results: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the instance group manager. Must be 1-63
        /// characters long and comply with
        /// [RFC1035](https://www.ietf.org/rfc/rfc1035.txt). Supported characters
        /// include lowercase letters, numbers, and hyphens.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The named port configuration. See the section below
        /// for details on configuration.
        pub named_ports: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::compute::InstanceGroupManagerNamedPort>>,
        >,
        pub operation: pulumi_wasm_rust::Output<String>,
        /// Input only additional params for instance group manager creation. Structure is documented below. For more information, see [API](https://cloud.google.com/compute/docs/reference/rest/beta/instanceGroupManagers/insert).
        ///
        /// - - -
        pub params: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::InstanceGroupManagerParams>,
        >,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The URL of the created resource.
        pub self_link: pulumi_wasm_rust::Output<String>,
        /// The standby policy for stopped and suspended instances. Structure is documented below. For more information, see the [official documentation](https://cloud.google.com/compute/docs/instance-groups/suspended-and-stopped-vms-in-mig) and [API](https://cloud.google.com/compute/docs/reference/rest/beta/regionInstanceGroupManagers/patch)
        pub standby_policy: pulumi_wasm_rust::Output<
            super::super::types::compute::InstanceGroupManagerStandbyPolicy,
        >,
        /// Disks created on the instances that will be preserved on instance delete, update, etc. Structure is documented below. For more information see the [official documentation](https://cloud.google.com/compute/docs/instance-groups/configuring-stateful-disks-in-migs).
        pub stateful_disks: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::compute::InstanceGroupManagerStatefulDisk>>,
        >,
        /// External network IPs assigned to the instances that will be preserved on instance delete, update, etc. This map is keyed with the network interface name. Structure is documented below.
        pub stateful_external_ips: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::compute::InstanceGroupManagerStatefulExternalIp>,
            >,
        >,
        /// Internal network IPs assigned to the instances that will be preserved on instance delete, update, etc. This map is keyed with the network interface name. Structure is documented below.
        pub stateful_internal_ips: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::compute::InstanceGroupManagerStatefulInternalIp>,
            >,
        >,
        /// The status of this managed instance group.
        pub statuses: pulumi_wasm_rust::Output<
            Vec<super::super::types::compute::InstanceGroupManagerStatus>,
        >,
        /// The full URL of all target pools to which new
        /// instances in the group are added. Updating the target pools attribute does
        /// not affect existing instances.
        pub target_pools: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The target number of running instances for this managed instance group. This value should always be explicitly set
        /// unless this resource is attached to an autoscaler, in which case it should never be set. Defaults to 0.
        pub target_size: pulumi_wasm_rust::Output<i32>,
        /// The target number of stopped instances for this managed instance group.
        pub target_stopped_size: pulumi_wasm_rust::Output<i32>,
        /// The target number of suspended instances for this managed instance group.
        pub target_suspended_size: pulumi_wasm_rust::Output<i32>,
        /// The update policy for this managed instance group. Structure is documented below. For more information, see the [official documentation](https://cloud.google.com/compute/docs/instance-groups/updating-managed-instance-groups) and [API](https://cloud.google.com/compute/docs/reference/rest/v1/instanceGroupManagers/patch).
        pub update_policy: pulumi_wasm_rust::Output<
            super::super::types::compute::InstanceGroupManagerUpdatePolicy,
        >,
        /// Application versions managed by this instance group. Each
        /// version deals with a specific instance template, allowing canary release scenarios.
        /// Structure is documented below.
        pub versions: pulumi_wasm_rust::Output<
            Vec<super::super::types::compute::InstanceGroupManagerVersion>,
        >,
        /// Whether to wait for all instances to be created/updated before
        /// returning. Note that if this is set to true and the operation does not succeed, this provider will
        /// continue trying until it times out.
        pub wait_for_instances: pulumi_wasm_rust::Output<Option<bool>>,
        /// When used with `wait_for_instances` it specifies the status to wait for.
        /// When `STABLE` is specified this resource will wait until the instances are stable before returning. When `UPDATED` is
        /// set, it will wait for the version target to be reached and any per instance configs to be effective as well as all
        /// instances to be stable before returning. The possible values are `STABLE` and `UPDATED`
        pub wait_for_instances_status: pulumi_wasm_rust::Output<Option<String>>,
        /// The zone that instances in this group should be created
        /// in.
        ///
        /// - - -
        pub zone: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: InstanceGroupManagerArgs,
    ) -> InstanceGroupManagerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let all_instances_config_binding = args.all_instances_config.get_inner();
        let auto_healing_policies_binding = args.auto_healing_policies.get_inner();
        let base_instance_name_binding = args.base_instance_name.get_inner();
        let description_binding = args.description.get_inner();
        let instance_lifecycle_policy_binding = args
            .instance_lifecycle_policy
            .get_inner();
        let list_managed_instances_results_binding = args
            .list_managed_instances_results
            .get_inner();
        let name_binding = args.name.get_inner();
        let named_ports_binding = args.named_ports.get_inner();
        let params_binding = args.params.get_inner();
        let project_binding = args.project.get_inner();
        let standby_policy_binding = args.standby_policy.get_inner();
        let stateful_disks_binding = args.stateful_disks.get_inner();
        let stateful_external_ips_binding = args.stateful_external_ips.get_inner();
        let stateful_internal_ips_binding = args.stateful_internal_ips.get_inner();
        let target_pools_binding = args.target_pools.get_inner();
        let target_size_binding = args.target_size.get_inner();
        let target_stopped_size_binding = args.target_stopped_size.get_inner();
        let target_suspended_size_binding = args.target_suspended_size.get_inner();
        let update_policy_binding = args.update_policy.get_inner();
        let versions_binding = args.versions.get_inner();
        let wait_for_instances_binding = args.wait_for_instances.get_inner();
        let wait_for_instances_status_binding = args
            .wait_for_instances_status
            .get_inner();
        let zone_binding = args.zone.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/instanceGroupManager:InstanceGroupManager".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allInstancesConfig".into(),
                    value: &all_instances_config_binding,
                },
                register_interface::ObjectField {
                    name: "autoHealingPolicies".into(),
                    value: &auto_healing_policies_binding,
                },
                register_interface::ObjectField {
                    name: "baseInstanceName".into(),
                    value: &base_instance_name_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "instanceLifecyclePolicy".into(),
                    value: &instance_lifecycle_policy_binding,
                },
                register_interface::ObjectField {
                    name: "listManagedInstancesResults".into(),
                    value: &list_managed_instances_results_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "namedPorts".into(),
                    value: &named_ports_binding,
                },
                register_interface::ObjectField {
                    name: "params".into(),
                    value: &params_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "standbyPolicy".into(),
                    value: &standby_policy_binding,
                },
                register_interface::ObjectField {
                    name: "statefulDisks".into(),
                    value: &stateful_disks_binding,
                },
                register_interface::ObjectField {
                    name: "statefulExternalIps".into(),
                    value: &stateful_external_ips_binding,
                },
                register_interface::ObjectField {
                    name: "statefulInternalIps".into(),
                    value: &stateful_internal_ips_binding,
                },
                register_interface::ObjectField {
                    name: "targetPools".into(),
                    value: &target_pools_binding,
                },
                register_interface::ObjectField {
                    name: "targetSize".into(),
                    value: &target_size_binding,
                },
                register_interface::ObjectField {
                    name: "targetStoppedSize".into(),
                    value: &target_stopped_size_binding,
                },
                register_interface::ObjectField {
                    name: "targetSuspendedSize".into(),
                    value: &target_suspended_size_binding,
                },
                register_interface::ObjectField {
                    name: "updatePolicy".into(),
                    value: &update_policy_binding,
                },
                register_interface::ObjectField {
                    name: "versions".into(),
                    value: &versions_binding,
                },
                register_interface::ObjectField {
                    name: "waitForInstances".into(),
                    value: &wait_for_instances_binding,
                },
                register_interface::ObjectField {
                    name: "waitForInstancesStatus".into(),
                    value: &wait_for_instances_status_binding,
                },
                register_interface::ObjectField {
                    name: "zone".into(),
                    value: &zone_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "allInstancesConfig".into(),
                },
                register_interface::ResultField {
                    name: "autoHealingPolicies".into(),
                },
                register_interface::ResultField {
                    name: "baseInstanceName".into(),
                },
                register_interface::ResultField {
                    name: "creationTimestamp".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "fingerprint".into(),
                },
                register_interface::ResultField {
                    name: "instanceGroup".into(),
                },
                register_interface::ResultField {
                    name: "instanceGroupManagerId".into(),
                },
                register_interface::ResultField {
                    name: "instanceLifecyclePolicy".into(),
                },
                register_interface::ResultField {
                    name: "listManagedInstancesResults".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "namedPorts".into(),
                },
                register_interface::ResultField {
                    name: "operation".into(),
                },
                register_interface::ResultField {
                    name: "params".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
                register_interface::ResultField {
                    name: "standbyPolicy".into(),
                },
                register_interface::ResultField {
                    name: "statefulDisks".into(),
                },
                register_interface::ResultField {
                    name: "statefulExternalIps".into(),
                },
                register_interface::ResultField {
                    name: "statefulInternalIps".into(),
                },
                register_interface::ResultField {
                    name: "statuses".into(),
                },
                register_interface::ResultField {
                    name: "targetPools".into(),
                },
                register_interface::ResultField {
                    name: "targetSize".into(),
                },
                register_interface::ResultField {
                    name: "targetStoppedSize".into(),
                },
                register_interface::ResultField {
                    name: "targetSuspendedSize".into(),
                },
                register_interface::ResultField {
                    name: "updatePolicy".into(),
                },
                register_interface::ResultField {
                    name: "versions".into(),
                },
                register_interface::ResultField {
                    name: "waitForInstances".into(),
                },
                register_interface::ResultField {
                    name: "waitForInstancesStatus".into(),
                },
                register_interface::ResultField {
                    name: "zone".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        InstanceGroupManagerResult {
            all_instances_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allInstancesConfig").unwrap(),
            ),
            auto_healing_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoHealingPolicies").unwrap(),
            ),
            base_instance_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("baseInstanceName").unwrap(),
            ),
            creation_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTimestamp").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            fingerprint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fingerprint").unwrap(),
            ),
            instance_group: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceGroup").unwrap(),
            ),
            instance_group_manager_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceGroupManagerId").unwrap(),
            ),
            instance_lifecycle_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceLifecyclePolicy").unwrap(),
            ),
            list_managed_instances_results: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("listManagedInstancesResults").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            named_ports: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namedPorts").unwrap(),
            ),
            operation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("operation").unwrap(),
            ),
            params: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("params").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
            standby_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("standbyPolicy").unwrap(),
            ),
            stateful_disks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statefulDisks").unwrap(),
            ),
            stateful_external_ips: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statefulExternalIps").unwrap(),
            ),
            stateful_internal_ips: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statefulInternalIps").unwrap(),
            ),
            statuses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statuses").unwrap(),
            ),
            target_pools: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetPools").unwrap(),
            ),
            target_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetSize").unwrap(),
            ),
            target_stopped_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetStoppedSize").unwrap(),
            ),
            target_suspended_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetSuspendedSize").unwrap(),
            ),
            update_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updatePolicy").unwrap(),
            ),
            versions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versions").unwrap(),
            ),
            wait_for_instances: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("waitForInstances").unwrap(),
            ),
            wait_for_instances_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("waitForInstancesStatus").unwrap(),
            ),
            zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zone").unwrap(),
            ),
        }
    }
}
