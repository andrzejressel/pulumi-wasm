/// ## Example Usage
///
/// ### Compute Rmig Resize Request
///
///
/// ```yaml
/// resources:
///   a3Dws:
///     type: gcp:compute:RegionInstanceTemplate
///     name: a3_dws
///     properties:
///       name: a3-dws
///       region: us-central1
///       description: This template is used to create a mig instance that is compatible with DWS resize requests.
///       instanceDescription: A3 GPU
///       machineType: a3-highgpu-8g
///       canIpForward: false
///       scheduling:
///         automaticRestart: false
///         onHostMaintenance: TERMINATE
///       disks:
///         - sourceImage: cos-cloud/cos-105-lts
///           autoDelete: true
///           boot: true
///           diskType: pd-ssd
///           diskSizeGb: '960'
///           mode: READ_WRITE
///       guestAccelerators:
///         - type: nvidia-h100-80gb
///           count: 8
///       reservationAffinity:
///         type: NO_RESERVATION
///       shieldedInstanceConfig:
///         enableVtpm: true
///         enableIntegrityMonitoring: true
///       networkInterfaces:
///         - network: default
///   a3DwsRegionInstanceGroupManager:
///     type: gcp:compute:RegionInstanceGroupManager
///     name: a3_dws
///     properties:
///       name: a3-dws
///       baseInstanceName: a3-dws
///       region: us-central1
///       versions:
///         - instanceTemplate: ${a3Dws.selfLink}
///       instanceLifecyclePolicy:
///         defaultActionOnFailure: DO_NOTHING
///       distributionPolicyTargetShape: ANY_SINGLE_ZONE
///       distributionPolicyZones:
///         - us-central1-a
///         - us-central1-b
///         - us-central1-c
///         - us-central1-f
///       updatePolicy:
///         instanceRedistributionType: NONE
///         type: OPPORTUNISTIC
///         minimalAction: REPLACE
///         maxSurgeFixed: 0
///         maxUnavailableFixed: 6
///       waitForInstances: false
///   a3ResizeRequest:
///     type: gcp:compute:RegionResizeRequest
///     name: a3_resize_request
///     properties:
///       name: a3-dws
///       instanceGroupManager: ${a3DwsRegionInstanceGroupManager.name}
///       region: us-central1
///       description: Test resize request resource
///       resizeBy: 2
///       requestedRunDuration:
///         seconds: 14400
///         nanos: 0
/// ```
///
/// ## Import
///
/// RegionResizeRequest can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/regions/{{region}}/instanceGroupManagers/{{instance_group_manager}}/resizeRequests/{{name}}`
///
/// * `{{project}}/{{region}}/{{instance_group_manager}}/{{name}}`
///
/// * `{{region}}/{{instance_group_manager}}/{{name}}`
///
/// * `{{instance_group_manager}}/{{name}}`
///
/// When using the `pulumi import` command, RegionResizeRequest can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/regionResizeRequest:RegionResizeRequest default projects/{{project}}/regions/{{region}}/instanceGroupManagers/{{instance_group_manager}}/resizeRequests/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionResizeRequest:RegionResizeRequest default {{project}}/{{region}}/{{instance_group_manager}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionResizeRequest:RegionResizeRequest default {{region}}/{{instance_group_manager}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionResizeRequest:RegionResizeRequest default {{instance_group_manager}}/{{name}}
/// ```
///
pub mod region_resize_request {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegionResizeRequestArgs {
        /// An optional description of this resize-request.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The reference of the regional instance group manager this ResizeRequest is a part of.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub instance_group_manager: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of this resize request. The name must be 1-63 characters long, and comply with RFC1035.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The reference of the compute region scoping this request.
        #[builder(into)]
        pub region: pulumi_wasm_rust::InputOrOutput<String>,
        /// Requested run duration for instances that will be created by this request. At the end of the run duration instances will be deleted.
        /// Structure is documented below.
        #[builder(into, default)]
        pub requested_run_duration: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::compute::RegionResizeRequestRequestedRunDuration>,
        >,
        /// The number of instances to be created by this resize request. The group's target size will be increased by this number.
        #[builder(into)]
        pub resize_by: pulumi_wasm_rust::InputOrOutput<i32>,
    }
    #[allow(dead_code)]
    pub struct RegionResizeRequestResult {
        /// The creation timestamp for this resize request in RFC3339 text format.
        pub creation_timestamp: pulumi_wasm_rust::Output<String>,
        /// An optional description of this resize-request.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The reference of the regional instance group manager this ResizeRequest is a part of.
        ///
        ///
        /// - - -
        pub instance_group_manager: pulumi_wasm_rust::Output<String>,
        /// The name of this resize request. The name must be 1-63 characters long, and comply with RFC1035.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The reference of the compute region scoping this request.
        pub region: pulumi_wasm_rust::Output<String>,
        /// Requested run duration for instances that will be created by this request. At the end of the run duration instances will be deleted.
        /// Structure is documented below.
        pub requested_run_duration: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::RegionResizeRequestRequestedRunDuration>,
        >,
        /// The number of instances to be created by this resize request. The group's target size will be increased by this number.
        pub resize_by: pulumi_wasm_rust::Output<i32>,
        /// Current state of the request.
        pub state: pulumi_wasm_rust::Output<String>,
        /// Status of the request.
        /// Structure is documented below.
        pub statuses: pulumi_wasm_rust::Output<
            Vec<super::super::types::compute::RegionResizeRequestStatus>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: RegionResizeRequestArgs,
    ) -> RegionResizeRequestResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let instance_group_manager_binding = args
            .instance_group_manager
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let requested_run_duration_binding = args
            .requested_run_duration
            .get_output(context)
            .get_inner();
        let resize_by_binding = args.resize_by.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/regionResizeRequest:RegionResizeRequest".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "instanceGroupManager".into(),
                    value: &instance_group_manager_binding,
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
                    name: "requestedRunDuration".into(),
                    value: &requested_run_duration_binding,
                },
                register_interface::ObjectField {
                    name: "resizeBy".into(),
                    value: &resize_by_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RegionResizeRequestResult {
            creation_timestamp: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("creationTimestamp"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            instance_group_manager: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instanceGroupManager"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            region: pulumi_wasm_rust::__private::into_domain(o.extract_field("region")),
            requested_run_duration: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("requestedRunDuration"),
            ),
            resize_by: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resizeBy"),
            ),
            state: pulumi_wasm_rust::__private::into_domain(o.extract_field("state")),
            statuses: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("statuses"),
            ),
        }
    }
}
