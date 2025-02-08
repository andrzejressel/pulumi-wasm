/// Authorize the Synchronizer to download environment data from the control plane.
///
///
/// To get more information about SyncAuthorization, see:
///
/// * [API documentation](https://cloud.google.com/apigee/docs/reference/apis/apigee/rest/v1/organizations#getsyncauthorization)
/// * How-to Guides
///     * [Enable Synchronizer access](https://cloud.google.com/apigee/docs/hybrid/v1.8/synchronizer-access#enable-synchronizer-access)
///
/// ## Example Usage
///
/// ### Apigee Sync Authorization Basic Test
///
///
/// ```yaml
/// resources:
///   project:
///     type: gcp:organizations:Project
///     properties:
///       projectId: my-project
///       name: my-project
///       orgId: '123456789'
///       billingAccount: 000000-0000000-0000000-000000
///       deletionPolicy: DELETE
///   apigee:
///     type: gcp:projects:Service
///     properties:
///       project: ${project.projectId}
///       service: apigee.googleapis.com
///   apigeeOrg:
///     type: gcp:apigee:Organization
///     name: apigee_org
///     properties:
///       analyticsRegion: us-central1
///       projectId: ${project.projectId}
///       runtimeType: HYBRID
///     options:
///       dependsOn:
///         - ${apigee}
///   serviceAccount:
///     type: gcp:serviceaccount:Account
///     name: service_account
///     properties:
///       accountId: my-account
///       displayName: Service Account
///   synchronizer-iam:
///     type: gcp:projects:IAMMember
///     properties:
///       project: ${project.projectId}
///       role: roles/apigee.synchronizerManager
///       member: serviceAccount:${serviceAccount.email}
///   apigeeSyncAuthorization:
///     type: gcp:apigee:SyncAuthorization
///     name: apigee_sync_authorization
///     properties:
///       name: ${apigeeOrg.name}
///       identities:
///         - serviceAccount:${serviceAccount.email}
///     options:
///       dependsOn:
///         - ${["synchronizer-iam"]}
/// ```
///
/// ## Import
///
/// SyncAuthorization can be imported using any of these accepted formats:
///
/// * `organizations/{{name}}/syncAuthorization`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, SyncAuthorization can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:apigee/syncAuthorization:SyncAuthorization default organizations/{{name}}/syncAuthorization
/// ```
///
/// ```sh
/// $ pulumi import gcp:apigee/syncAuthorization:SyncAuthorization default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod sync_authorization {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SyncAuthorizationArgs {
        /// Array of service accounts to grant access to control plane resources, each specified using the following format: `serviceAccount:service-account-name`.
        /// The `service-account-name` is formatted like an email address. For example: my-synchronizer-manager-serviceAccount@my_project_id.iam.gserviceaccount.com
        /// You might specify multiple service accounts, for example, if you have multiple environments and wish to assign a unique service account to each one.
        /// The service accounts must have **Apigee Synchronizer Manager** role. See also [Create service accounts](https://cloud.google.com/apigee/docs/hybrid/v1.8/sa-about#create-the-service-accounts).
        #[builder(into)]
        pub identities: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Name of the Apigee organization.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SyncAuthorizationResult {
        /// Entity tag (ETag) used for optimistic concurrency control as a way to help prevent simultaneous updates from overwriting each other.
        /// Used internally during updates.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// Array of service accounts to grant access to control plane resources, each specified using the following format: `serviceAccount:service-account-name`.
        /// The `service-account-name` is formatted like an email address. For example: my-synchronizer-manager-serviceAccount@my_project_id.iam.gserviceaccount.com
        /// You might specify multiple service accounts, for example, if you have multiple environments and wish to assign a unique service account to each one.
        /// The service accounts must have **Apigee Synchronizer Manager** role. See also [Create service accounts](https://cloud.google.com/apigee/docs/hybrid/v1.8/sa-about#create-the-service-accounts).
        pub identities: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Name of the Apigee organization.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SyncAuthorizationArgs,
    ) -> SyncAuthorizationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let identities_binding = args.identities.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:apigee/syncAuthorization:SyncAuthorization".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "identities".into(),
                    value: &identities_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SyncAuthorizationResult {
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            identities: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identities"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
        }
    }
}
