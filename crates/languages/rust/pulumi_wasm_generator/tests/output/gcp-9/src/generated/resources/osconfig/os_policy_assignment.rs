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
pub mod os_policy_assignment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OsPolicyAssignmentArgs {
        /// OS policy assignment description. Length of the description is limited to 1024 characters.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Filter to select VMs. Structure is
        /// documented below.
        #[builder(into)]
        pub instance_filter: pulumi_wasm_rust::InputOrOutput<
            super::super::types::osconfig::OsPolicyAssignmentInstanceFilter,
        >,
        /// The location for the resource
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// Resource name.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// List of OS policies to be applied to the VMs.
        /// Structure is documented below.
        #[builder(into)]
        pub os_policies: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::types::osconfig::OsPolicyAssignmentOsPolicy>,
        >,
        /// The project for the resource
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Rollout to deploy the OS policy assignment. A rollout
        /// is triggered in the following situations: 1) OSPolicyAssignment is created.
        /// 2) OSPolicyAssignment is updated and the update contains changes to one of
        /// the following fields: - instance_filter - os_policies 3) OSPolicyAssignment
        /// is deleted. Structure is documented below.
        #[builder(into)]
        pub rollout: pulumi_wasm_rust::InputOrOutput<
            super::super::types::osconfig::OsPolicyAssignmentRollout,
        >,
        /// Set to true to skip awaiting rollout during resource creation and update.
        #[builder(into, default)]
        pub skip_await_rollout: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct OsPolicyAssignmentResult {
        /// Output only. Indicates that this revision has been successfully
        /// rolled out in this zone and new VMs will be assigned OS policies from this
        /// revision. For a given OS policy assignment, there is only one revision with
        /// a value of `true` for this field.
        pub baseline: pulumi_wasm_rust::Output<bool>,
        /// Output only. Indicates that this revision deletes the OS policy
        /// assignment.
        pub deleted: pulumi_wasm_rust::Output<bool>,
        /// OS policy assignment description. Length of the description is limited to 1024 characters.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The etag for this OS policy assignment. If this is provided on
        /// update, it must match the server's etag.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// Filter to select VMs. Structure is
        /// documented below.
        pub instance_filter: pulumi_wasm_rust::Output<
            super::super::types::osconfig::OsPolicyAssignmentInstanceFilter,
        >,
        /// The location for the resource
        pub location: pulumi_wasm_rust::Output<String>,
        /// Resource name.
        pub name: pulumi_wasm_rust::Output<String>,
        /// List of OS policies to be applied to the VMs.
        /// Structure is documented below.
        pub os_policies: pulumi_wasm_rust::Output<
            Vec<super::super::types::osconfig::OsPolicyAssignmentOsPolicy>,
        >,
        /// The project for the resource
        pub project: pulumi_wasm_rust::Output<String>,
        /// Output only. Indicates that reconciliation is in progress
        /// for the revision. This value is `true` when the `rollout_state` is one of:
        pub reconciling: pulumi_wasm_rust::Output<bool>,
        /// Output only. The timestamp that the revision was
        /// created.
        pub revision_create_time: pulumi_wasm_rust::Output<String>,
        /// Output only. The assignment revision ID A new revision is
        /// committed whenever a rollout is triggered for a OS policy assignment
        pub revision_id: pulumi_wasm_rust::Output<String>,
        /// Rollout to deploy the OS policy assignment. A rollout
        /// is triggered in the following situations: 1) OSPolicyAssignment is created.
        /// 2) OSPolicyAssignment is updated and the update contains changes to one of
        /// the following fields: - instance_filter - os_policies 3) OSPolicyAssignment
        /// is deleted. Structure is documented below.
        pub rollout: pulumi_wasm_rust::Output<
            super::super::types::osconfig::OsPolicyAssignmentRollout,
        >,
        /// Output only. OS policy assignment rollout state
        pub rollout_state: pulumi_wasm_rust::Output<String>,
        /// Set to true to skip awaiting rollout during resource creation and update.
        pub skip_await_rollout: pulumi_wasm_rust::Output<Option<bool>>,
        /// Output only. Server generated unique id for the OS policy assignment
        /// resource.
        pub uid: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: OsPolicyAssignmentArgs,
    ) -> OsPolicyAssignmentResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let instance_filter_binding = args
            .instance_filter
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let os_policies_binding = args.os_policies.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let rollout_binding = args.rollout.get_output(context).get_inner();
        let skip_await_rollout_binding = args
            .skip_await_rollout
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:osconfig/osPolicyAssignment:OsPolicyAssignment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "instanceFilter".into(),
                    value: &instance_filter_binding,
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
                    name: "osPolicies".into(),
                    value: &os_policies_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "rollout".into(),
                    value: &rollout_binding,
                },
                register_interface::ObjectField {
                    name: "skipAwaitRollout".into(),
                    value: &skip_await_rollout_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        OsPolicyAssignmentResult {
            baseline: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("baseline"),
            ),
            deleted: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deleted"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(o.extract_field("etag")),
            instance_filter: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instanceFilter"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            os_policies: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("osPolicies"),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            reconciling: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("reconciling"),
            ),
            revision_create_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("revisionCreateTime"),
            ),
            revision_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("revisionId"),
            ),
            rollout: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("rollout"),
            ),
            rollout_state: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("rolloutState"),
            ),
            skip_await_rollout: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("skipAwaitRollout"),
            ),
            uid: pulumi_wasm_rust::__private::into_domain(o.extract_field("uid")),
        }
    }
}
