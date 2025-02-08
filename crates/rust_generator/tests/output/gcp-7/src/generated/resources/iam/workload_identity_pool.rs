/// Represents a collection of external workload identities. You can define IAM policies to
/// grant these identities access to Google Cloud resources.
///
///
/// To get more information about WorkloadIdentityPool, see:
///
/// * [API documentation](https://cloud.google.com/iam/docs/reference/rest/v1/projects.locations.workloadIdentityPools)
/// * How-to Guides
///     * [Managing workload identity pools](https://cloud.google.com/iam/docs/manage-workload-identity-pools-providers#pools)
///
/// ## Example Usage
///
/// ### Iam Workload Identity Pool Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = workload_identity_pool::create(
///         "example",
///         WorkloadIdentityPoolArgs::builder()
///             .workload_identity_pool_id("example-pool")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Iam Workload Identity Pool Full
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = workload_identity_pool::create(
///         "example",
///         WorkloadIdentityPoolArgs::builder()
///             .description("Identity pool for automated test")
///             .disabled(true)
///             .display_name("Name of pool")
///             .workload_identity_pool_id("example-pool")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// WorkloadIdentityPool can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/global/workloadIdentityPools/{{workload_identity_pool_id}}`
///
/// * `{{project}}/{{workload_identity_pool_id}}`
///
/// * `{{workload_identity_pool_id}}`
///
/// When using the `pulumi import` command, WorkloadIdentityPool can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:iam/workloadIdentityPool:WorkloadIdentityPool default projects/{{project}}/locations/global/workloadIdentityPools/{{workload_identity_pool_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:iam/workloadIdentityPool:WorkloadIdentityPool default {{project}}/{{workload_identity_pool_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:iam/workloadIdentityPool:WorkloadIdentityPool default {{workload_identity_pool_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod workload_identity_pool {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkloadIdentityPoolArgs {
        /// A description of the pool. Cannot exceed 256 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether the pool is disabled. You cannot use a disabled pool to exchange tokens, or use
        /// existing tokens to access resources. If the pool is re-enabled, existing tokens grant
        /// access again.
        #[builder(into, default)]
        pub disabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A display name for the pool. Cannot exceed 32 characters.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID to use for the pool, which becomes the final component of the resource name. This
        /// value should be 4-32 characters, and may contain the characters [a-z0-9-]. The prefix
        /// `gcp-` is reserved for use by Google, and may not be specified.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub workload_identity_pool_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WorkloadIdentityPoolResult {
        /// A description of the pool. Cannot exceed 256 characters.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether the pool is disabled. You cannot use a disabled pool to exchange tokens, or use
        /// existing tokens to access resources. If the pool is re-enabled, existing tokens grant
        /// access again.
        pub disabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A display name for the pool. Cannot exceed 32 characters.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The resource name of the pool as
        /// `projects/{project_number}/locations/global/workloadIdentityPools/{workload_identity_pool_id}`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The state of the pool.
        /// * STATE_UNSPECIFIED: State unspecified.
        /// * ACTIVE: The pool is active, and may be used in Google Cloud policies.
        /// * DELETED: The pool is soft-deleted. Soft-deleted pools are permanently deleted after
        /// approximately 30 days. You can restore a soft-deleted pool using
        /// UndeleteWorkloadIdentityPool. You cannot reuse the ID of a soft-deleted pool until it is
        /// permanently deleted. While a pool is deleted, you cannot use it to exchange tokens, or
        /// use existing tokens to access resources. If the pool is undeleted, existing tokens grant
        /// access again.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// The ID to use for the pool, which becomes the final component of the resource name. This
        /// value should be 4-32 characters, and may contain the characters [a-z0-9-]. The prefix
        /// `gcp-` is reserved for use by Google, and may not be specified.
        ///
        ///
        /// - - -
        pub workload_identity_pool_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: WorkloadIdentityPoolArgs,
    ) -> WorkloadIdentityPoolResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let disabled_binding = args.disabled.get_output(context).get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let workload_identity_pool_id_binding = args
            .workload_identity_pool_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:iam/workloadIdentityPool:WorkloadIdentityPool".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
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
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "workloadIdentityPoolId".into(),
                    value: &workload_identity_pool_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        WorkloadIdentityPoolResult {
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            disabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("disabled"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            workload_identity_pool_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workloadIdentityPoolId"),
            ),
        }
    }
}
