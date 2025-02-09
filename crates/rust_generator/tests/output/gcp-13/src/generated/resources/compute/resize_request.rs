/// Represents a Managed Instance Group Resize Request
///
/// Resize Requests are the Managed Instance Group implementation of Dynamic Workload Scheduler Flex Start.
///
/// With Dynamic Workload Scheduler in Flex Start mode, you submit a GPU capacity request for your AI/ML jobs by indicating how many you need, a duration, and your preferred region. Dynamic Workload Scheduler intelligently persists the request; once the capacity becomes available, it automatically provisions your VMs enabling your workloads to run continuously for the entire duration of the capacity allocation.
///
///
/// To get more information about ResizeRequest, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/instanceGroupManagerResizeRequests)
/// * How-to Guides
///     * [QUICKSTART_TITLE](https://cloud.google.com/compute/docs/instance-groups/create-resize-requests-mig)
///
/// ## Example Usage
///
/// ### Compute Mig Resize Request
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
///   a3DwsInstanceGroupManager:
///     type: gcp:compute:InstanceGroupManager
///     name: a3_dws
///     properties:
///       name: a3-dws
///       baseInstanceName: a3-dws
///       zone: us-central1-a
///       versions:
///         - instanceTemplate: ${a3Dws.selfLink}
///       instanceLifecyclePolicy:
///         defaultActionOnFailure: DO_NOTHING
///       waitForInstances: false
///   a3ResizeRequest:
///     type: gcp:compute:ResizeRequest
///     name: a3_resize_request
///     properties:
///       name: a3-dws
///       instanceGroupManager: ${a3DwsInstanceGroupManager.name}
///       zone: us-central1-a
///       description: Test resize request resource
///       resizeBy: 2
///       requestedRunDuration:
///         seconds: 14400
///         nanos: 0
/// ```
///
/// ## Import
///
/// ResizeRequest can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/zones/{{zone}}/instanceGroupManagers/{{instance_group_manager}}/resizeRequests/{{name}}`
///
/// * `{{project}}/{{zone}}/{{instance_group_manager}}/{{name}}`
///
/// * `{{zone}}/{{instance_group_manager}}/{{name}}`
///
/// * `{{instance_group_manager}}/{{name}}`
///
/// When using the `pulumi import` command, ResizeRequest can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/resizeRequest:ResizeRequest default projects/{{project}}/zones/{{zone}}/instanceGroupManagers/{{instance_group_manager}}/resizeRequests/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/resizeRequest:ResizeRequest default {{project}}/{{zone}}/{{instance_group_manager}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/resizeRequest:ResizeRequest default {{zone}}/{{instance_group_manager}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/resizeRequest:ResizeRequest default {{instance_group_manager}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod resize_request {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResizeRequestArgs {
        /// An optional description of this resize-request.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the managed instance group. The name should conform to RFC1035 or be a resource ID.
        /// Authorization requires the following IAM permission on the specified resource instanceGroupManager:
        /// *compute.instanceGroupManagers.update
        ///
        ///
        /// - - -
        #[builder(into)]
        pub instance_group_manager: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of this resize request. The name must be 1-63 characters long, and comply with RFC1035.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Requested run duration for instances that will be created by this request. At the end of the run duration instance will be deleted.
        /// Structure is documented below.
        #[builder(into, default)]
        pub requested_run_duration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::ResizeRequestRequestedRunDuration>,
        >,
        /// The number of instances to be created by this resize request. The group's target size will be increased by this number.
        #[builder(into)]
        pub resize_by: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Name of the compute zone scoping this request. Name should conform to RFC1035.
        #[builder(into)]
        pub zone: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ResizeRequestResult {
        /// The creation timestamp for this resize request in RFC3339 text format.
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// An optional description of this resize-request.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the managed instance group. The name should conform to RFC1035 or be a resource ID.
        /// Authorization requires the following IAM permission on the specified resource instanceGroupManager:
        /// *compute.instanceGroupManagers.update
        ///
        ///
        /// - - -
        pub instance_group_manager: pulumi_gestalt_rust::Output<String>,
        /// The name of this resize request. The name must be 1-63 characters long, and comply with RFC1035.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Requested run duration for instances that will be created by this request. At the end of the run duration instance will be deleted.
        /// Structure is documented below.
        pub requested_run_duration: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::ResizeRequestRequestedRunDuration>,
        >,
        /// The number of instances to be created by this resize request. The group's target size will be increased by this number.
        pub resize_by: pulumi_gestalt_rust::Output<i32>,
        /// [Output only] Current state of the request.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// [Output only] Status of the request.
        /// Structure is documented below.
        pub statuses: pulumi_gestalt_rust::Output<
            Vec<super::super::types::compute::ResizeRequestStatus>,
        >,
        /// Name of the compute zone scoping this request. Name should conform to RFC1035.
        pub zone: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResizeRequestArgs,
    ) -> ResizeRequestResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let instance_group_manager_binding = args
            .instance_group_manager
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let requested_run_duration_binding = args
            .requested_run_duration
            .get_output(context);
        let resize_by_binding = args.resize_by.get_output(context);
        let zone_binding = args.zone.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/resizeRequest:ResizeRequest".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceGroupManager".into(),
                    value: instance_group_manager_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "requestedRunDuration".into(),
                    value: requested_run_duration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resizeBy".into(),
                    value: resize_by_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zone".into(),
                    value: zone_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ResizeRequestResult {
            creation_timestamp: o.get_field("creationTimestamp"),
            description: o.get_field("description"),
            instance_group_manager: o.get_field("instanceGroupManager"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            requested_run_duration: o.get_field("requestedRunDuration"),
            resize_by: o.get_field("resizeBy"),
            state: o.get_field("state"),
            statuses: o.get_field("statuses"),
            zone: o.get_field("zone"),
        }
    }
}
