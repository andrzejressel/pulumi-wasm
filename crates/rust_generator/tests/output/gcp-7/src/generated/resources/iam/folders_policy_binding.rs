/// A policy binding to a folder
///
///
/// To get more information about FoldersPolicyBinding, see:
///
/// * [API documentation](https://cloud.google.com/iam/docs/reference/rest/v3/folders.locations.policyBindings)
/// * How-to Guides
///     * [Apply a policy binding](https://cloud.google.com/iam/docs/principal-access-boundary-policies-create#create_binding)
///
/// ## Example Usage
///
/// ### Iam Folders Policy Binding
///
///
/// ```yaml
/// resources:
///   pabPolicy:
///     type: gcp:iam:PrincipalAccessBoundaryPolicy
///     name: pab_policy
///     properties:
///       organization: '123456789'
///       location: global
///       displayName: test folder binding
///       principalAccessBoundaryPolicyId: my-pab-policy
///   folder:
///     type: gcp:organizations:Folder
///     properties:
///       displayName: test folder
///       parent: organizations/123456789
///       deletionProtection: false
///   wait120s:
///     type: time:sleep
///     name: wait_120s
///     properties:
///       createDuration: 120s
///     options:
///       dependsOn:
///         - ${folder}
///   my-folder-binding:
///     type: gcp:iam:FoldersPolicyBinding
///     properties:
///       folder: ${folder.folderId}
///       location: global
///       displayName: test folder binding
///       policyKind: PRINCIPAL_ACCESS_BOUNDARY
///       policyBindingId: test-folder-binding
///       policy: organizations/123456789/locations/global/principalAccessBoundaryPolicies/${pabPolicy.principalAccessBoundaryPolicyId}
///       target:
///         principalSet: //cloudresourcemanager.googleapis.com/folders/${folder.folderId}
///     options:
///       dependsOn:
///         - ${wait120s}
/// ```
///
/// ## Import
///
/// FoldersPolicyBinding can be imported using any of these accepted formats:
///
/// * `folders/{{folder}}/locations/{{location}}/policyBindings/{{policy_binding_id}}`
///
/// * `{{folder}}/{{location}}/{{policy_binding_id}}`
///
/// When using the `pulumi import` command, FoldersPolicyBinding can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:iam/foldersPolicyBinding:FoldersPolicyBinding default folders/{{folder}}/locations/{{location}}/policyBindings/{{policy_binding_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:iam/foldersPolicyBinding:FoldersPolicyBinding default {{folder}}/{{location}}/{{policy_binding_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod folders_policy_binding {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FoldersPolicyBindingArgs {
        /// Optional. User defined annotations. See https://google.aip.dev/148#annotations for more details such as format and size
        /// limitations **Note**: This field is non-authoritative, and will only manage the annotations present in your
        /// configuration. Please refer to the field 'effective_annotations' for all of the annotations present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Represents a textual expression in the Common Expression Language (CEL) syntax. CEL is a C-like expression language. The
        /// syntax and semantics of CEL are documented at https://github.com/google/cel-spec. Example (Comparison): title: \"Summary
        /// size limit\" description: \"Determines if a summary is less than 100 chars\" expression: \"document.summary.size() <
        /// 100\" Example (Equality): title: \"Requestor is owner\" description: \"Determines if requestor is the document owner\"
        /// expression: \"document.owner == request.auth.claims.email\" Example (Logic): title: \"Public documents\" description:
        /// \"Determine whether the document should be publicly visible\" expression: \"document.type != 'private' && document.type
        /// != 'internal'\" Example (Data Manipulation): title: \"Notification string\" description: \"Create a notification string
        /// with a timestamp.\" expression: \"'New message received at ' + string(document.create_time)\" The exact variables and
        /// functions that may be referenced within an expression are determined by the service that evaluates it. See the service
        /// documentation for additional information.
        #[builder(into, default)]
        pub condition: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::iam::FoldersPolicyBindingCondition>,
        >,
        /// Optional. The description of the policy binding. Must be less than or equal to 63 characters.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The parent folder for the PolicyBinding.
        #[builder(into)]
        pub folder: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The location of the PolicyBinding.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Required. Immutable. The resource name of the policy to be bound. The binding parent and policy must belong to the same Organization (or Project).
        #[builder(into)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Policy Binding ID.
        #[builder(into)]
        pub policy_binding_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Immutable. The kind of the policy to attach in this binding. This field must be one of the following: - Left empty (will
        /// be automatically set to the policy kind) - The input policy kind Possible values: POLICY_KIND_UNSPECIFIED
        /// PRINCIPAL_ACCESS_BOUNDARY ACCESS
        #[builder(into, default)]
        pub policy_kind: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Target is the full resource name of the resource to which the policy will be bound. Immutable once set.
        /// Structure is documented below.
        #[builder(into)]
        pub target: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::iam::FoldersPolicyBindingTarget,
        >,
    }
    #[allow(dead_code)]
    pub struct FoldersPolicyBindingResult {
        /// Optional. User defined annotations. See https://google.aip.dev/148#annotations for more details such as format and size
        /// limitations **Note**: This field is non-authoritative, and will only manage the annotations present in your
        /// configuration. Please refer to the field 'effective_annotations' for all of the annotations present on the resource.
        pub annotations: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Represents a textual expression in the Common Expression Language (CEL) syntax. CEL is a C-like expression language. The
        /// syntax and semantics of CEL are documented at https://github.com/google/cel-spec. Example (Comparison): title: \"Summary
        /// size limit\" description: \"Determines if a summary is less than 100 chars\" expression: \"document.summary.size() <
        /// 100\" Example (Equality): title: \"Requestor is owner\" description: \"Determines if requestor is the document owner\"
        /// expression: \"document.owner == request.auth.claims.email\" Example (Logic): title: \"Public documents\" description:
        /// \"Determine whether the document should be publicly visible\" expression: \"document.type != 'private' && document.type
        /// != 'internal'\" Example (Data Manipulation): title: \"Notification string\" description: \"Create a notification string
        /// with a timestamp.\" expression: \"'New message received at ' + string(document.create_time)\" The exact variables and
        /// functions that may be referenced within an expression are determined by the service that evaluates it. See the service
        /// documentation for additional information.
        pub condition: pulumi_gestalt_rust::Output<
            Option<super::super::types::iam::FoldersPolicyBindingCondition>,
        >,
        /// Output only. The time when the policy binding was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Optional. The description of the policy binding. Must be less than or equal to 63 characters.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        pub effective_annotations: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Optional. The etag for the policy binding. If this is provided on update, it must match the server's etag.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The parent folder for the PolicyBinding.
        pub folder: pulumi_gestalt_rust::Output<String>,
        /// The location of the PolicyBinding.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the policy binding in the format `{binding_parent/locations/{location}/policyBindings/{policy_binding_id}`
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Required. Immutable. The resource name of the policy to be bound. The binding parent and policy must belong to the same Organization (or Project).
        pub policy: pulumi_gestalt_rust::Output<String>,
        /// The Policy Binding ID.
        pub policy_binding_id: pulumi_gestalt_rust::Output<String>,
        /// Immutable. The kind of the policy to attach in this binding. This field must be one of the following: - Left empty (will
        /// be automatically set to the policy kind) - The input policy kind Possible values: POLICY_KIND_UNSPECIFIED
        /// PRINCIPAL_ACCESS_BOUNDARY ACCESS
        pub policy_kind: pulumi_gestalt_rust::Output<Option<String>>,
        /// Output only. The globally unique ID of the policy to be bound.
        pub policy_uid: pulumi_gestalt_rust::Output<String>,
        /// Target is the full resource name of the resource to which the policy will be bound. Immutable once set.
        /// Structure is documented below.
        pub target: pulumi_gestalt_rust::Output<
            super::super::types::iam::FoldersPolicyBindingTarget,
        >,
        /// Output only. The globally unique ID of the policy binding. Assigned when the policy binding is created.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// Output only. The time when the policy binding was most recently updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FoldersPolicyBindingArgs,
    ) -> FoldersPolicyBindingResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let annotations_binding = args.annotations.get_output(context);
        let condition_binding = args.condition.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let folder_binding = args.folder.get_output(context);
        let location_binding = args.location.get_output(context);
        let policy_binding = args.policy.get_output(context);
        let policy_binding_id_binding = args.policy_binding_id.get_output(context);
        let policy_kind_binding = args.policy_kind.get_output(context);
        let target_binding = args.target.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:iam/foldersPolicyBinding:FoldersPolicyBinding".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "condition".into(),
                    value: &condition_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "folder".into(),
                    value: &folder_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policy".into(),
                    value: &policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyBindingId".into(),
                    value: &policy_binding_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyKind".into(),
                    value: &policy_kind_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "target".into(),
                    value: &target_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        FoldersPolicyBindingResult {
            annotations: o.get_field("annotations"),
            condition: o.get_field("condition"),
            create_time: o.get_field("createTime"),
            display_name: o.get_field("displayName"),
            effective_annotations: o.get_field("effectiveAnnotations"),
            etag: o.get_field("etag"),
            folder: o.get_field("folder"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            policy: o.get_field("policy"),
            policy_binding_id: o.get_field("policyBindingId"),
            policy_kind: o.get_field("policyKind"),
            policy_uid: o.get_field("policyUid"),
            target: o.get_field("target"),
            uid: o.get_field("uid"),
            update_time: o.get_field("updateTime"),
        }
    }
}
