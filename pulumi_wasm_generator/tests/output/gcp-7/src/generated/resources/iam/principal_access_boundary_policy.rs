/// An IAM Principal Access Boundary Policy resource
///
///
/// To get more information about PrincipalAccessBoundaryPolicy, see:
///
/// * [API documentation](https://cloud.google.com/iam/docs/reference/rest/v3/organizations.locations.principalAccessBoundaryPolicies)
/// * How-to Guides
///     * [Create and apply Principal Access Boundaries](https://cloud.google.com/iam/docs/principal-access-boundary-policies-create)
///
/// ## Example Usage
///
/// ### Iam Principal Access Boundary Policy
///
///
/// ```yaml
/// resources:
///   my-pab-policy:
///     type: gcp:iam:PrincipalAccessBoundaryPolicy
///     properties:
///       organization: '123456789'
///       location: global
///       displayName: test pab policy
///       principalAccessBoundaryPolicyId: test-pab-policy
/// ```
///
/// ## Import
///
/// PrincipalAccessBoundaryPolicy can be imported using any of these accepted formats:
///
/// * `organizations/{{organization}}/locations/{{location}}/principalAccessBoundaryPolicies/{{principal_access_boundary_policy_id}}`
///
/// * `{{organization}}/{{location}}/{{principal_access_boundary_policy_id}}`
///
/// When using the `pulumi import` command, PrincipalAccessBoundaryPolicy can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:iam/principalAccessBoundaryPolicy:PrincipalAccessBoundaryPolicy default organizations/{{organization}}/locations/{{location}}/principalAccessBoundaryPolicies/{{principal_access_boundary_policy_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:iam/principalAccessBoundaryPolicy:PrincipalAccessBoundaryPolicy default {{organization}}/{{location}}/{{principal_access_boundary_policy_id}}
/// ```
///
pub mod principal_access_boundary_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PrincipalAccessBoundaryPolicyArgs {
        /// User defined annotations. See https://google.aip.dev/148#annotations
        /// for more details such as format and size limitations
        ///
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Principal access boundary policy details
        /// Structure is documented below.
        #[builder(into, default)]
        pub details: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::iam::PrincipalAccessBoundaryPolicyDetails>,
        >,
        /// The description of the principal access boundary policy. Must be less than or equal to 63 characters.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The location the principal access boundary policy is in.
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// The parent organization of the principal access boundary policy.
        #[builder(into)]
        pub organization: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID to use to create the principal access boundary policy.
        /// This value must start with a lowercase letter followed by up to 62 lowercase letters, numbers, hyphens, or dots. Pattern, /a-z{2,62}/.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub principal_access_boundary_policy_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PrincipalAccessBoundaryPolicyResult {
        /// User defined annotations. See https://google.aip.dev/148#annotations
        /// for more details such as format and size limitations
        ///
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        pub annotations: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Output only. The time when the principal access boundary policy was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Principal access boundary policy details
        /// Structure is documented below.
        pub details: pulumi_wasm_rust::Output<
            super::super::types::iam::PrincipalAccessBoundaryPolicyDetails,
        >,
        /// The description of the principal access boundary policy. Must be less than or equal to 63 characters.
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        pub effective_annotations: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The etag for the principal access boundary. If this is provided on update, it must match the server's etag.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The location the principal access boundary policy is in.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Identifier. The resource name of the principal access boundary policy.  The following format is supported:
        /// `organizations/{organization_id}/locations/{location}/principalAccessBoundaryPolicies/{policy_id}`
        pub name: pulumi_wasm_rust::Output<String>,
        /// The parent organization of the principal access boundary policy.
        pub organization: pulumi_wasm_rust::Output<String>,
        /// The ID to use to create the principal access boundary policy.
        /// This value must start with a lowercase letter followed by up to 62 lowercase letters, numbers, hyphens, or dots. Pattern, /a-z{2,62}/.
        ///
        ///
        /// - - -
        pub principal_access_boundary_policy_id: pulumi_wasm_rust::Output<String>,
        /// Output only. The globally unique ID of the principal access boundary policy.
        pub uid: pulumi_wasm_rust::Output<String>,
        /// Output only. The time when the principal access boundary policy was most recently updated.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: PrincipalAccessBoundaryPolicyArgs,
    ) -> PrincipalAccessBoundaryPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let annotations_binding = args.annotations.get_output(context).get_inner();
        let details_binding = args.details.get_output(context).get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let organization_binding = args.organization.get_output(context).get_inner();
        let principal_access_boundary_policy_id_binding = args
            .principal_access_boundary_policy_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:iam/principalAccessBoundaryPolicy:PrincipalAccessBoundaryPolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding,
                },
                register_interface::ObjectField {
                    name: "details".into(),
                    value: &details_binding,
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
                    name: "principalAccessBoundaryPolicyId".into(),
                    value: &principal_access_boundary_policy_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "annotations".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "details".into(),
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
                    name: "principalAccessBoundaryPolicyId".into(),
                },
                register_interface::ResultField {
                    name: "uid".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PrincipalAccessBoundaryPolicyResult {
            annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("annotations").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            details: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("details").unwrap(),
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
            principal_access_boundary_policy_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("principalAccessBoundaryPolicyId").unwrap(),
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
