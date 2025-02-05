/// Represents a private cloud resource. Private clouds are zonal resources.
///
///
/// To get more information about PrivateCloud, see:
///
/// * [API documentation](https://cloud.google.com/vmware-engine/docs/reference/rest/v1/projects.locations.privateClouds)
///
/// ## Example Usage
///
/// ### Vmware Engine Private Cloud Basic
///
///
/// ```yaml
/// resources:
///   vmw-engine-pc:
///     type: gcp:vmwareengine:PrivateCloud
///     properties:
///       location: us-west1-a
///       name: sample-pc
///       description: Sample test PC.
///       networkConfig:
///         managementCidr: 192.168.30.0/24
///         vmwareEngineNetwork: ${["pc-nw"].id}
///       managementCluster:
///         clusterId: sample-mgmt-cluster
///         nodeTypeConfigs:
///           - nodeTypeId: standard-72
///             nodeCount: 3
///   pc-nw:
///     type: gcp:vmwareengine:Network
///     properties:
///       name: pc-nw
///       location: global
///       type: STANDARD
///       description: PC network description.
/// ```
/// ### Vmware Engine Private Cloud Full
///
///
/// ```yaml
/// resources:
///   vmw-engine-pc:
///     type: gcp:vmwareengine:PrivateCloud
///     properties:
///       location: us-west1-a
///       name: sample-pc
///       description: Sample test PC.
///       type: TIME_LIMITED
///       networkConfig:
///         managementCidr: 192.168.30.0/24
///         vmwareEngineNetwork: ${["pc-nw"].id}
///       managementCluster:
///         clusterId: sample-mgmt-cluster
///         nodeTypeConfigs:
///           - nodeTypeId: standard-72
///             nodeCount: 1
///             customCoreCount: 32
///         autoscalingSettings:
///           autoscalingPolicies:
///             - autoscalePolicyId: autoscaling-policy
///               nodeTypeId: standard-72
///               scaleOutSize: 1
///               cpuThresholds:
///                 scaleOut: 80
///                 scaleIn: 15
///               consumedMemoryThresholds:
///                 scaleOut: 75
///                 scaleIn: 20
///               storageThresholds:
///                 scaleOut: 80
///                 scaleIn: 20
///           minClusterNodeCount: 3
///           maxClusterNodeCount: 8
///           coolDownPeriod: 1800s
///       deletionDelayHours: 0
///       sendDeletionDelayHoursIfZero: true
///   pc-nw:
///     type: gcp:vmwareengine:Network
///     properties:
///       name: pc-nw
///       location: global
///       type: STANDARD
///       description: PC network description.
/// ```
///
/// ## Import
///
/// PrivateCloud can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/privateClouds/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, PrivateCloud can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:vmwareengine/privateCloud:PrivateCloud default projects/{{project}}/locations/{{location}}/privateClouds/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vmwareengine/privateCloud:PrivateCloud default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vmwareengine/privateCloud:PrivateCloud default {{location}}/{{name}}
/// ```
///
pub mod private_cloud {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PrivateCloudArgs {
        /// The number of hours to delay this request. You can set this value to an hour between 0 to 8, where setting it to 0
        /// starts the deletion request immediately. If no value is set, a default value is set at the API Level.
        #[builder(into, default)]
        pub deletion_delay_hours: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// User-provided description for this private cloud.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The location where the PrivateCloud should reside.
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// The management cluster for this private cloud. This used for creating and managing the default cluster.
        /// Structure is documented below.
        #[builder(into)]
        pub management_cluster: pulumi_wasm_rust::InputOrOutput<
            super::super::types::vmwareengine::PrivateCloudManagementCluster,
        >,
        /// The ID of the PrivateCloud.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Network configuration in the consumer project with which the peering has to be done.
        /// Structure is documented below.
        #[builder(into)]
        pub network_config: pulumi_wasm_rust::InputOrOutput<
            super::super::types::vmwareengine::PrivateCloudNetworkConfig,
        >,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// While set true, deletion_delay_hours value will be sent in the request even for zero value of the field. This field is
        /// only useful for setting 0 value to the deletion_delay_hours field. It can be used both alone and together with
        /// deletion_delay_hours.
        #[builder(into, default)]
        pub send_deletion_delay_hours_if_zero: pulumi_wasm_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Initial type of the private cloud. Possible values: ["STANDARD", "TIME_LIMITED", "STRETCHED"]
        #[builder(into, default)]
        pub type_: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct PrivateCloudResult {
        /// The number of hours to delay this request. You can set this value to an hour between 0 to 8, where setting it to 0
        /// starts the deletion request immediately. If no value is set, a default value is set at the API Level.
        pub deletion_delay_hours: pulumi_wasm_rust::Output<Option<i32>>,
        /// User-provided description for this private cloud.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Details about a HCX Cloud Manager appliance.
        /// Structure is documented below.
        pub hcxes: pulumi_wasm_rust::Output<
            Vec<super::super::types::vmwareengine::PrivateCloudHcx>,
        >,
        /// The location where the PrivateCloud should reside.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The management cluster for this private cloud. This used for creating and managing the default cluster.
        /// Structure is documented below.
        pub management_cluster: pulumi_wasm_rust::Output<
            super::super::types::vmwareengine::PrivateCloudManagementCluster,
        >,
        /// The ID of the PrivateCloud.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Network configuration in the consumer project with which the peering has to be done.
        /// Structure is documented below.
        pub network_config: pulumi_wasm_rust::Output<
            super::super::types::vmwareengine::PrivateCloudNetworkConfig,
        >,
        /// Details about a NSX Manager appliance.
        /// Structure is documented below.
        pub nsxes: pulumi_wasm_rust::Output<
            Vec<super::super::types::vmwareengine::PrivateCloudNsx>,
        >,
        pub project: pulumi_wasm_rust::Output<String>,
        /// While set true, deletion_delay_hours value will be sent in the request even for zero value of the field. This field is
        /// only useful for setting 0 value to the deletion_delay_hours field. It can be used both alone and together with
        /// deletion_delay_hours.
        pub send_deletion_delay_hours_if_zero: pulumi_wasm_rust::Output<Option<bool>>,
        /// State of the appliance.
        /// Possible values are: `ACTIVE`, `CREATING`.
        pub state: pulumi_wasm_rust::Output<String>,
        /// Initial type of the private cloud. Possible values: ["STANDARD", "TIME_LIMITED", "STRETCHED"]
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
        /// System-generated unique identifier for the resource.
        pub uid: pulumi_wasm_rust::Output<String>,
        /// Details about a vCenter Server management appliance.
        /// Structure is documented below.
        pub vcenters: pulumi_wasm_rust::Output<
            Vec<super::super::types::vmwareengine::PrivateCloudVcenter>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: PrivateCloudArgs,
    ) -> PrivateCloudResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let deletion_delay_hours_binding = args
            .deletion_delay_hours
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let management_cluster_binding = args
            .management_cluster
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let network_config_binding = args.network_config.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let send_deletion_delay_hours_if_zero_binding = args
            .send_deletion_delay_hours_if_zero
            .get_output(context)
            .get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:vmwareengine/privateCloud:PrivateCloud".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "deletionDelayHours".into(),
                    value: &deletion_delay_hours_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "managementCluster".into(),
                    value: &management_cluster_binding,
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
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "sendDeletionDelayHoursIfZero".into(),
                    value: &send_deletion_delay_hours_if_zero_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        PrivateCloudResult {
            deletion_delay_hours: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deletionDelayHours"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            hcxes: pulumi_wasm_rust::__private::into_domain(o.extract_field("hcxes")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            management_cluster: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("managementCluster"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            network_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("networkConfig"),
            ),
            nsxes: pulumi_wasm_rust::__private::into_domain(o.extract_field("nsxes")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            send_deletion_delay_hours_if_zero: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sendDeletionDelayHoursIfZero"),
            ),
            state: pulumi_wasm_rust::__private::into_domain(o.extract_field("state")),
            type_: pulumi_wasm_rust::__private::into_domain(o.extract_field("type")),
            uid: pulumi_wasm_rust::__private::into_domain(o.extract_field("uid")),
            vcenters: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("vcenters"),
            ),
        }
    }
}
