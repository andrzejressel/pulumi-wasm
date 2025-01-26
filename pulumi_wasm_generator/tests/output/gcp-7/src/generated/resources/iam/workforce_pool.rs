/// Represents a collection of external workforces. Provides namespaces for
/// federated users that can be referenced in IAM policies.
///
///
/// To get more information about WorkforcePool, see:
///
/// * [API documentation](https://cloud.google.com/iam/docs/reference/rest/v1/locations.workforcePools)
/// * How-to Guides
///     * [Manage pools](https://cloud.google.com/iam/docs/manage-workforce-identity-pools-providers#manage_pools)
///
/// > **Note:** Ask your Google Cloud account team to request access to workforce identity federation for
/// your billing/quota project. The account team notifies you when the project is granted access.
///
/// ## Example Usage
///
/// ### Iam Workforce Pool Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = workforce_pool::create(
///         "example",
///         WorkforcePoolArgs::builder()
///             .location("global")
///             .parent("organizations/123456789")
///             .workforce_pool_id("example-pool")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Iam Workforce Pool Full
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = workforce_pool::create(
///         "example",
///         WorkforcePoolArgs::builder()
///             .access_restrictions(
///                 WorkforcePoolAccessRestrictions::builder()
///                     .allowedServices(
///                         vec![
///                             WorkforcePoolAccessRestrictionsAllowedService::builder()
///                             .domain("backstory.chronicle.security").build_struct(),
///                         ],
///                     )
///                     .disableProgrammaticSignin(false)
///                     .build_struct(),
///             )
///             .description("A sample workforce pool.")
///             .disabled(false)
///             .display_name("Display name")
///             .location("global")
///             .parent("organizations/123456789")
///             .session_duration("7200s")
///             .workforce_pool_id("example-pool")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// WorkforcePool can be imported using any of these accepted formats:
///
/// * `locations/{{location}}/workforcePools/{{workforce_pool_id}}`
///
/// * `{{location}}/{{workforce_pool_id}}`
///
/// When using the `pulumi import` command, WorkforcePool can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:iam/workforcePool:WorkforcePool default locations/{{location}}/workforcePools/{{workforce_pool_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:iam/workforcePool:WorkforcePool default {{location}}/{{workforce_pool_id}}
/// ```
///
pub mod workforce_pool {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkforcePoolArgs {
        /// Configure access restrictions on the workforce pool users. This is an optional field. If specified web
        /// sign-in can be restricted to given set of services or programmatic sign-in can be disabled for pool users.
        /// Structure is documented below.
        #[builder(into, default)]
        pub access_restrictions: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::iam::WorkforcePoolAccessRestrictions>,
        >,
        /// A user-specified description of the pool. Cannot exceed 256 characters.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Whether the pool is disabled. You cannot use a disabled pool to exchange tokens,
        /// or use existing tokens to access resources. If the pool is re-enabled, existing tokens grant access again.
        #[builder(into, default)]
        pub disabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// A user-specified display name of the pool in Google Cloud Console. Cannot exceed 32 characters.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The location for the resource.
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// Immutable. The resource name of the parent. Format: `organizations/{org-id}`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub parent: pulumi_wasm_rust::InputOrOutput<String>,
        /// Duration that the Google Cloud access tokens, console sign-in sessions,
        /// and `gcloud` sign-in sessions from this pool are valid.
        /// Must be greater than 15 minutes (900s) and less than 12 hours (43200s).
        /// If `sessionDuration` is not configured, minted credentials have a default duration of one hour (3600s).
        /// A duration in seconds with up to nine fractional digits, ending with '`s`'. Example: "`3.5s`".
        #[builder(into, default)]
        pub session_duration: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the pool. The ID must be a globally unique string of 6 to 63 lowercase letters,
        /// digits, or hyphens. It must start with a letter, and cannot have a trailing hyphen.
        /// The prefix `gcp-` is reserved for use by Google, and may not be specified.
        #[builder(into)]
        pub workforce_pool_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WorkforcePoolResult {
        /// Configure access restrictions on the workforce pool users. This is an optional field. If specified web
        /// sign-in can be restricted to given set of services or programmatic sign-in can be disabled for pool users.
        /// Structure is documented below.
        pub access_restrictions: pulumi_wasm_rust::Output<
            Option<super::super::types::iam::WorkforcePoolAccessRestrictions>,
        >,
        /// A user-specified description of the pool. Cannot exceed 256 characters.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether the pool is disabled. You cannot use a disabled pool to exchange tokens,
        /// or use existing tokens to access resources. If the pool is re-enabled, existing tokens grant access again.
        pub disabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A user-specified display name of the pool in Google Cloud Console. Cannot exceed 32 characters.
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The location for the resource.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Output only. The resource name of the pool.
        /// Format: `locations/{location}/workforcePools/{workforcePoolId}`
        pub name: pulumi_wasm_rust::Output<String>,
        /// Immutable. The resource name of the parent. Format: `organizations/{org-id}`.
        ///
        ///
        /// - - -
        pub parent: pulumi_wasm_rust::Output<String>,
        /// Duration that the Google Cloud access tokens, console sign-in sessions,
        /// and `gcloud` sign-in sessions from this pool are valid.
        /// Must be greater than 15 minutes (900s) and less than 12 hours (43200s).
        /// If `sessionDuration` is not configured, minted credentials have a default duration of one hour (3600s).
        /// A duration in seconds with up to nine fractional digits, ending with '`s`'. Example: "`3.5s`".
        pub session_duration: pulumi_wasm_rust::Output<Option<String>>,
        /// Output only. The state of the pool.
        /// * STATE_UNSPECIFIED: State unspecified.
        /// * ACTIVE: The pool is active, and may be used in Google Cloud policies.
        /// * DELETED: The pool is soft-deleted. Soft-deleted pools are permanently deleted
        /// after approximately 30 days. You can restore a soft-deleted pool using
        /// [workforcePools.undelete](https://cloud.google.com/iam/docs/reference/rest/v1/locations.workforcePools/undelete#google.iam.admin.v1.WorkforcePools.UndeleteWorkforcePool).
        /// You cannot reuse the ID of a soft-deleted pool until it is permanently deleted.
        /// While a pool is deleted, you cannot use it to exchange tokens, or use
        /// existing tokens to access resources. If the pool is undeleted, existing
        /// tokens grant access again.
        pub state: pulumi_wasm_rust::Output<String>,
        /// The name of the pool. The ID must be a globally unique string of 6 to 63 lowercase letters,
        /// digits, or hyphens. It must start with a letter, and cannot have a trailing hyphen.
        /// The prefix `gcp-` is reserved for use by Google, and may not be specified.
        pub workforce_pool_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: WorkforcePoolArgs,
    ) -> WorkforcePoolResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_restrictions_binding = args
            .access_restrictions
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let disabled_binding = args.disabled.get_output(context).get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let parent_binding = args.parent.get_output(context).get_inner();
        let session_duration_binding = args
            .session_duration
            .get_output(context)
            .get_inner();
        let workforce_pool_id_binding = args
            .workforce_pool_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:iam/workforcePool:WorkforcePool".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessRestrictions".into(),
                    value: &access_restrictions_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "disabled".into(),
                    value: &disabled_binding,
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
                    name: "parent".into(),
                    value: &parent_binding,
                },
                register_interface::ObjectField {
                    name: "sessionDuration".into(),
                    value: &session_duration_binding,
                },
                register_interface::ObjectField {
                    name: "workforcePoolId".into(),
                    value: &workforce_pool_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessRestrictions".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "disabled".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "parent".into(),
                },
                register_interface::ResultField {
                    name: "sessionDuration".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "workforcePoolId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        WorkforcePoolResult {
            access_restrictions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessRestrictions").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            disabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disabled").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            parent: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parent").unwrap(),
            ),
            session_duration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sessionDuration").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            workforce_pool_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workforcePoolId").unwrap(),
            ),
        }
    }
}
