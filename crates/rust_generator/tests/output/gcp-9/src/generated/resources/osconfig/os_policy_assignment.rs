/// OS policy assignment is an API resource that is used to apply a set of OS
/// policies to a dynamically targeted group of Compute Engine VM instances. An OS
/// policy is used to define the desired state configuration for a Compute Engine VM
/// instance through a set of configuration resources that provide capabilities such
/// as installing or removing software packages, or executing a script. For more
/// information about the OS policy resource definitions and examples, see
/// [OS policy and OS policy assignment](https://cloud.google.com/compute/docs/os-configuration-management/working-with-os-policies).
///
/// To get more information about OSPolicyAssignment, see:
///
/// *   [API documentation](https://cloud.google.com/compute/docs/osconfig/rest/v1/projects.locations.osPolicyAssignments)
/// *   How-to Guides
///     *   [Official Documentation](https://cloud.google.com/compute/docs/os-configuration-management/create-os-policy-assignment)
///
/// ## Example Usage
///
/// ### Os Config Os Policy Assignment Basic
///
/// ```yaml
/// resources:
///   primary:
///     type: gcp:osconfig:OsPolicyAssignment
///     properties:
///       instanceFilter:
///         all: false
///         exclusionLabels:
///           - labels:
///               label-two: value-two
///         inclusionLabels:
///           - labels:
///               label-one: value-one
///         inventories:
///           - osShortName: centos
///             osVersion: 8.*
///       location: us-central1-a
///       name: policy-assignment
///       osPolicies:
///         - id: policy
///           mode: VALIDATION
///           resourceGroups:
///             - resources:
///                 - id: apt-to-yum
///                   repository:
///                     apt:
///                       archiveType: DEB
///                       components:
///                         - doc
///                       distribution: debian
///                       uri: https://atl.mirrors.clouvider.net/debian
///                       gpgKey: .gnupg/pubring.kbx
///                 - id: exec1
///                   exec:
///                     validate:
///                       interpreter: SHELL
///                       args:
///                         - arg1
///                       file:
///                         localPath: $HOME/script.sh
///                       outputFilePath: $HOME/out
///                     enforce:
///                       interpreter: SHELL
///                       args:
///                         - arg1
///                       file:
///                         allowInsecure: true
///                         remote:
///                           uri: https://www.example.com/script.sh
///                           sha256Checksum: c7938fed83afdccbb0e86a2a2e4cad7d5035012ca3214b4a61268393635c3063
///                       outputFilePath: $HOME/out
///               inventoryFilters:
///                 - osShortName: centos
///                   osVersion: 8.*
///           allowNoResourceGroupMatch: false
///           description: A test os policy
///       rollout:
///         disruptionBudget:
///           percent: 100
///         minWaitDuration: 3s
///       description: A test os policy assignment
/// ```
///
/// ## Import
///
/// OSPolicyAssignment can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/osPolicyAssignments/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, OSPolicyAssignment can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:osconfig/osPolicyAssignment:OsPolicyAssignment default projects/{{project}}/locations/{{location}}/osPolicyAssignments/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:osconfig/osPolicyAssignment:OsPolicyAssignment default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:osconfig/osPolicyAssignment:OsPolicyAssignment default {{location}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod os_policy_assignment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OsPolicyAssignmentArgs {
        /// OS policy assignment description. Length of the description is limited to 1024 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Filter to select VMs. Structure is
        /// documented below.
        #[builder(into)]
        pub instance_filter: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::osconfig::OsPolicyAssignmentInstanceFilter,
        >,
        /// The location for the resource
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Resource name.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of OS policies to be applied to the VMs.
        /// Structure is documented below.
        #[builder(into)]
        pub os_policies: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::osconfig::OsPolicyAssignmentOsPolicy>,
        >,
        /// The project for the resource
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Rollout to deploy the OS policy assignment. A rollout
        /// is triggered in the following situations: 1) OSPolicyAssignment is created.
        /// 2) OSPolicyAssignment is updated and the update contains changes to one of
        /// the following fields: - instance_filter - os_policies 3) OSPolicyAssignment
        /// is deleted. Structure is documented below.
        #[builder(into)]
        pub rollout: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::osconfig::OsPolicyAssignmentRollout,
        >,
        /// Set to true to skip awaiting rollout during resource creation and update.
        #[builder(into, default)]
        pub skip_await_rollout: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct OsPolicyAssignmentResult {
        /// Output only. Indicates that this revision has been successfully
        /// rolled out in this zone and new VMs will be assigned OS policies from this
        /// revision. For a given OS policy assignment, there is only one revision with
        /// a value of `true` for this field.
        pub baseline: pulumi_gestalt_rust::Output<bool>,
        /// Output only. Indicates that this revision deletes the OS policy
        /// assignment.
        pub deleted: pulumi_gestalt_rust::Output<bool>,
        /// OS policy assignment description. Length of the description is limited to 1024 characters.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The etag for this OS policy assignment. If this is provided on
        /// update, it must match the server's etag.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// Filter to select VMs. Structure is
        /// documented below.
        pub instance_filter: pulumi_gestalt_rust::Output<
            super::super::types::osconfig::OsPolicyAssignmentInstanceFilter,
        >,
        /// The location for the resource
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Resource name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// List of OS policies to be applied to the VMs.
        /// Structure is documented below.
        pub os_policies: pulumi_gestalt_rust::Output<
            Vec<super::super::types::osconfig::OsPolicyAssignmentOsPolicy>,
        >,
        /// The project for the resource
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Output only. Indicates that reconciliation is in progress
        /// for the revision. This value is `true` when the `rollout_state` is one of:
        pub reconciling: pulumi_gestalt_rust::Output<bool>,
        /// Output only. The timestamp that the revision was
        /// created.
        pub revision_create_time: pulumi_gestalt_rust::Output<String>,
        /// Output only. The assignment revision ID A new revision is
        /// committed whenever a rollout is triggered for a OS policy assignment
        pub revision_id: pulumi_gestalt_rust::Output<String>,
        /// Rollout to deploy the OS policy assignment. A rollout
        /// is triggered in the following situations: 1) OSPolicyAssignment is created.
        /// 2) OSPolicyAssignment is updated and the update contains changes to one of
        /// the following fields: - instance_filter - os_policies 3) OSPolicyAssignment
        /// is deleted. Structure is documented below.
        pub rollout: pulumi_gestalt_rust::Output<
            super::super::types::osconfig::OsPolicyAssignmentRollout,
        >,
        /// Output only. OS policy assignment rollout state
        pub rollout_state: pulumi_gestalt_rust::Output<String>,
        /// Set to true to skip awaiting rollout during resource creation and update.
        pub skip_await_rollout: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Output only. Server generated unique id for the OS policy assignment
        /// resource.
        pub uid: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: OsPolicyAssignmentArgs,
    ) -> OsPolicyAssignmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let instance_filter_binding = args.instance_filter.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let os_policies_binding = args.os_policies.get_output(context);
        let project_binding = args.project.get_output(context);
        let rollout_binding = args.rollout.get_output(context);
        let skip_await_rollout_binding = args.skip_await_rollout.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:osconfig/osPolicyAssignment:OsPolicyAssignment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceFilter".into(),
                    value: instance_filter_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "osPolicies".into(),
                    value: os_policies_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rollout".into(),
                    value: rollout_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skipAwaitRollout".into(),
                    value: skip_await_rollout_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        OsPolicyAssignmentResult {
            baseline: o.get_field("baseline"),
            deleted: o.get_field("deleted"),
            description: o.get_field("description"),
            etag: o.get_field("etag"),
            instance_filter: o.get_field("instanceFilter"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            os_policies: o.get_field("osPolicies"),
            project: o.get_field("project"),
            reconciling: o.get_field("reconciling"),
            revision_create_time: o.get_field("revisionCreateTime"),
            revision_id: o.get_field("revisionId"),
            rollout: o.get_field("rollout"),
            rollout_state: o.get_field("rolloutState"),
            skip_await_rollout: o.get_field("skipAwaitRollout"),
            uid: o.get_field("uid"),
        }
    }
}
