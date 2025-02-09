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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod workforce_pool {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkforcePoolArgs {
        /// Configure access restrictions on the workforce pool users. This is an optional field. If specified web
        /// sign-in can be restricted to given set of services or programmatic sign-in can be disabled for pool users.
        /// Structure is documented below.
        #[builder(into, default)]
        pub access_restrictions: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::iam::WorkforcePoolAccessRestrictions>,
        >,
        /// A user-specified description of the pool. Cannot exceed 256 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether the pool is disabled. You cannot use a disabled pool to exchange tokens,
        /// or use existing tokens to access resources. If the pool is re-enabled, existing tokens grant access again.
        #[builder(into, default)]
        pub disabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A user-specified display name of the pool in Google Cloud Console. Cannot exceed 32 characters.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The location for the resource.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Immutable. The resource name of the parent. Format: `organizations/{org-id}`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Duration that the Google Cloud access tokens, console sign-in sessions,
        /// and `gcloud` sign-in sessions from this pool are valid.
        /// Must be greater than 15 minutes (900s) and less than 12 hours (43200s).
        /// If `sessionDuration` is not configured, minted credentials have a default duration of one hour (3600s).
        /// A duration in seconds with up to nine fractional digits, ending with '`s`'. Example: "`3.5s`".
        #[builder(into, default)]
        pub session_duration: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the pool. The ID must be a globally unique string of 6 to 63 lowercase letters,
        /// digits, or hyphens. It must start with a letter, and cannot have a trailing hyphen.
        /// The prefix `gcp-` is reserved for use by Google, and may not be specified.
        #[builder(into)]
        pub workforce_pool_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WorkforcePoolResult {
        /// Configure access restrictions on the workforce pool users. This is an optional field. If specified web
        /// sign-in can be restricted to given set of services or programmatic sign-in can be disabled for pool users.
        /// Structure is documented below.
        pub access_restrictions: pulumi_gestalt_rust::Output<
            Option<super::super::types::iam::WorkforcePoolAccessRestrictions>,
        >,
        /// A user-specified description of the pool. Cannot exceed 256 characters.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether the pool is disabled. You cannot use a disabled pool to exchange tokens,
        /// or use existing tokens to access resources. If the pool is re-enabled, existing tokens grant access again.
        pub disabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A user-specified display name of the pool in Google Cloud Console. Cannot exceed 32 characters.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The location for the resource.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Output only. The resource name of the pool.
        /// Format: `locations/{location}/workforcePools/{workforcePoolId}`
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Immutable. The resource name of the parent. Format: `organizations/{org-id}`.
        ///
        ///
        /// - - -
        pub parent: pulumi_gestalt_rust::Output<String>,
        /// Duration that the Google Cloud access tokens, console sign-in sessions,
        /// and `gcloud` sign-in sessions from this pool are valid.
        /// Must be greater than 15 minutes (900s) and less than 12 hours (43200s).
        /// If `sessionDuration` is not configured, minted credentials have a default duration of one hour (3600s).
        /// A duration in seconds with up to nine fractional digits, ending with '`s`'. Example: "`3.5s`".
        pub session_duration: pulumi_gestalt_rust::Output<Option<String>>,
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
        pub state: pulumi_gestalt_rust::Output<String>,
        /// The name of the pool. The ID must be a globally unique string of 6 to 63 lowercase letters,
        /// digits, or hyphens. It must start with a letter, and cannot have a trailing hyphen.
        /// The prefix `gcp-` is reserved for use by Google, and may not be specified.
        pub workforce_pool_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WorkforcePoolArgs,
    ) -> WorkforcePoolResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let access_restrictions_binding = args.access_restrictions.get_output(context);
        let description_binding = args.description.get_output(context);
        let disabled_binding = args.disabled.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let location_binding = args.location.get_output(context);
        let parent_binding = args.parent.get_output(context);
        let session_duration_binding = args.session_duration.get_output(context);
        let workforce_pool_id_binding = args.workforce_pool_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:iam/workforcePool:WorkforcePool".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accessRestrictions".into(),
                    value: access_restrictions_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "disabled".into(),
                    value: disabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parent".into(),
                    value: parent_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sessionDuration".into(),
                    value: session_duration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workforcePoolId".into(),
                    value: workforce_pool_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        WorkforcePoolResult {
            access_restrictions: o.get_field("accessRestrictions"),
            description: o.get_field("description"),
            disabled: o.get_field("disabled"),
            display_name: o.get_field("displayName"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            parent: o.get_field("parent"),
            session_duration: o.get_field("sessionDuration"),
            state: o.get_field("state"),
            workforce_pool_id: o.get_field("workforcePoolId"),
        }
    }
}
