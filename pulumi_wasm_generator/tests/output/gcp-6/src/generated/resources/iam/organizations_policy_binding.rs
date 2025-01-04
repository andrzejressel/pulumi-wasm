/// A policy binding to an organizations
///
///
/// To get more information about OrganizationsPolicyBinding, see:
///
/// * [API documentation](https://cloud.google.com/iam/docs/reference/rest/v3/organizations.locations.policyBindings)
/// * How-to Guides
///     * [Apply a policy binding](https://cloud.google.com/iam/docs/principal-access-boundary-policies-create#create_binding)
///
/// ## Example Usage
///
/// ### Iam Organizations Policy Binding
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
///       displayName: test org binding
///       principalAccessBoundaryPolicyId: my-pab-policy
///   my-org-binding:
///     type: gcp:iam:OrganizationsPolicyBinding
///     properties:
///       organization: '123456789'
///       location: global
///       displayName: test org binding
///       policyKind: PRINCIPAL_ACCESS_BOUNDARY
///       policyBindingId: test-org-binding
///       policy: organizations/123456789/locations/global/principalAccessBoundaryPolicies/${pabPolicy.principalAccessBoundaryPolicyId}
///       target:
///         principalSet: //cloudresourcemanager.googleapis.com/organizations/123456789
/// ```
///
/// ## Import
///
/// OrganizationsPolicyBinding can be imported using any of these accepted formats:
///
/// * `organizations/{{organization}}/locations/{{location}}/policyBindings/{{policy_binding_id}}`
///
/// * `{{organization}}/{{location}}/{{policy_binding_id}}`
///
/// When using the `pulumi import` command, OrganizationsPolicyBinding can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:iam/organizationsPolicyBinding:OrganizationsPolicyBinding default organizations/{{organization}}/locations/{{location}}/policyBindings/{{policy_binding_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:iam/organizationsPolicyBinding:OrganizationsPolicyBinding default {{organization}}/{{location}}/{{policy_binding_id}}
/// ```
///
pub mod organizations_policy_binding {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OrganizationsPolicyBindingArgs {
        /// Optional. User defined annotations. See https://google.aip.dev/148#annotations for more details such as format and size
        /// limitations **Note**: This field is non-authoritative, and will only manage the annotations present in your
        /// configuration. Please refer to the field 'effective_annotations' for all of the annotations present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_wasm_rust::Output<
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
        pub condition: pulumi_wasm_rust::Output<
            Option<super::super::types::iam::OrganizationsPolicyBindingCondition>,
        >,
        /// Optional. The description of the policy binding. Must be less than or equal to 63 characters.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The location of the Policy Binding
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// The parent organization of the Policy Binding.
        #[builder(into)]
        pub organization: pulumi_wasm_rust::Output<String>,
        /// Required. Immutable. The resource name of the policy to be bound. The binding parent and policy must belong to the same Organization (or Project).
        #[builder(into)]
        pub policy: pulumi_wasm_rust::Output<String>,
        /// The Policy Binding ID.
        #[builder(into)]
        pub policy_binding_id: pulumi_wasm_rust::Output<String>,
        /// Immutable. The kind of the policy to attach in this binding. This field must be one of the following: - Left empty (will
        /// be automatically set to the policy kind) - The input policy kind Possible values: POLICY_KIND_UNSPECIFIED
        /// PRINCIPAL_ACCESS_BOUNDARY ACCESS
        #[builder(into, default)]
        pub policy_kind: pulumi_wasm_rust::Output<Option<String>>,
        /// Target is the full resource name of the resource to which the policy will be bound. Immutable once set.
        /// Structure is documented below.
        #[builder(into)]
        pub target: pulumi_wasm_rust::Output<
            super::super::types::iam::OrganizationsPolicyBindingTarget,
        >,
    }
    #[allow(dead_code)]
    pub struct OrganizationsPolicyBindingResult {
        /// Optional. User defined annotations. See https://google.aip.dev/148#annotations for more details such as format and size
        /// limitations **Note**: This field is non-authoritative, and will only manage the annotations present in your
        /// configuration. Please refer to the field 'effective_annotations' for all of the annotations present on the resource.
        pub annotations: pulumi_wasm_rust::Output<
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
        pub condition: pulumi_wasm_rust::Output<
            Option<super::super::types::iam::OrganizationsPolicyBindingCondition>,
        >,
        /// Output only. The time when the policy binding was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Optional. The description of the policy binding. Must be less than or equal to 63 characters.
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        pub effective_annotations: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Optional. The etag for the policy binding. If this is provided on update, it must match the server's etag.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The location of the Policy Binding
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the policy binding in the format `{binding_parent/locations/{location}/policyBindings/{policy_binding_id}`
        pub name: pulumi_wasm_rust::Output<String>,
        /// The parent organization of the Policy Binding.
        pub organization: pulumi_wasm_rust::Output<String>,
        /// Required. Immutable. The resource name of the policy to be bound. The binding parent and policy must belong to the same Organization (or Project).
        pub policy: pulumi_wasm_rust::Output<String>,
        /// The Policy Binding ID.
        pub policy_binding_id: pulumi_wasm_rust::Output<String>,
        /// Immutable. The kind of the policy to attach in this binding. This field must be one of the following: - Left empty (will
        /// be automatically set to the policy kind) - The input policy kind Possible values: POLICY_KIND_UNSPECIFIED
        /// PRINCIPAL_ACCESS_BOUNDARY ACCESS
        pub policy_kind: pulumi_wasm_rust::Output<Option<String>>,
        /// Output only. The globally unique ID of the policy to be bound.
        pub policy_uid: pulumi_wasm_rust::Output<String>,
        /// Target is the full resource name of the resource to which the policy will be bound. Immutable once set.
        /// Structure is documented below.
        pub target: pulumi_wasm_rust::Output<
            super::super::types::iam::OrganizationsPolicyBindingTarget,
        >,
        /// Output only. The globally unique ID of the policy binding. Assigned when the policy binding is created.
        pub uid: pulumi_wasm_rust::Output<String>,
        /// Output only. The time when the policy binding was most recently updated.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: OrganizationsPolicyBindingArgs,
    ) -> OrganizationsPolicyBindingResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let annotations_binding = args.annotations.get_inner();
        let condition_binding = args.condition.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let location_binding = args.location.get_inner();
        let organization_binding = args.organization.get_inner();
        let policy_binding = args.policy.get_inner();
        let policy_binding_id_binding = args.policy_binding_id.get_inner();
        let policy_kind_binding = args.policy_kind.get_inner();
        let target_binding = args.target.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:iam/organizationsPolicyBinding:OrganizationsPolicyBinding"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding,
                },
                register_interface::ObjectField {
                    name: "condition".into(),
                    value: &condition_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "organization".into(),
                    value: &organization_binding,
                },
                register_interface::ObjectField {
                    name: "policy".into(),
                    value: &policy_binding,
                },
                register_interface::ObjectField {
                    name: "policyBindingId".into(),
                    value: &policy_binding_id_binding,
                },
                register_interface::ObjectField {
                    name: "policyKind".into(),
                    value: &policy_kind_binding,
                },
                register_interface::ObjectField {
                    name: "target".into(),
                    value: &target_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "annotations".into(),
                },
                register_interface::ResultField {
                    name: "condition".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "effectiveAnnotations".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "organization".into(),
                },
                register_interface::ResultField {
                    name: "policy".into(),
                },
                register_interface::ResultField {
                    name: "policyBindingId".into(),
                },
                register_interface::ResultField {
                    name: "policyKind".into(),
                },
                register_interface::ResultField {
                    name: "policyUid".into(),
                },
                register_interface::ResultField {
                    name: "target".into(),
                },
                register_interface::ResultField {
                    name: "uid".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        OrganizationsPolicyBindingResult {
            annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("annotations").unwrap(),
            ),
            condition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("condition").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            effective_annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveAnnotations").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            organization: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("organization").unwrap(),
            ),
            policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policy").unwrap(),
            ),
            policy_binding_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyBindingId").unwrap(),
            ),
            policy_kind: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyKind").unwrap(),
            ),
            policy_uid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyUid").unwrap(),
            ),
            target: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("target").unwrap(),
            ),
            uid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uid").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
