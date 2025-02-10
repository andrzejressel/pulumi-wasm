/// A BeyondCorp AppConnector resource represents an application facing component deployed proximal to
/// and with direct access to the application instances. It is used to establish connectivity between the
/// remote enterprise environment and GCP. It initiates connections to the applications and can proxy the
/// data from users over the connection.
///
///
/// To get more information about AppConnector, see:
///
/// * [API documentation](https://cloud.google.com/beyondcorp/docs/reference/rest#rest-resource:-v1.projects.locations.appconnectors)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/beyondcorp-enterprise/docs/enable-app-connector)
///
/// ## Example Usage
///
/// ### Beyondcorp App Connector Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let appConnector = app_connector::create(
///         "appConnector",
///         AppConnectorArgs::builder()
///             .name("my-app-connector")
///             .principal_info(
///                 AppConnectorPrincipalInfo::builder()
///                     .serviceAccount(
///                         AppConnectorPrincipalInfoServiceAccount::builder()
///                             .email("${serviceAccount.email}")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let serviceAccount = account::create(
///         "serviceAccount",
///         AccountArgs::builder()
///             .account_id("my-account")
///             .display_name("Test Service Account")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Beyondcorp App Connector Full
///
///
/// ```yaml
/// resources:
///   serviceAccount:
///     type: gcp:serviceaccount:Account
///     name: service_account
///     properties:
///       accountId: my-account
///       displayName: Test Service Account
///   appConnector:
///     type: gcp:beyondcorp:AppConnector
///     name: app_connector
///     properties:
///       name: my-app-connector
///       region: us-central1
///       displayName: some display name
///       principalInfo:
///         serviceAccount:
///           email: ${serviceAccount.email}
///       labels:
///         foo: bar
///         bar: baz
/// ```
///
/// ## Import
///
/// AppConnector can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{region}}/appConnectors/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, AppConnector can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:beyondcorp/appConnector:AppConnector default projects/{{project}}/locations/{{region}}/appConnectors/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:beyondcorp/appConnector:AppConnector default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:beyondcorp/appConnector:AppConnector default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:beyondcorp/appConnector:AppConnector default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod app_connector {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AppConnectorArgs {
        /// An arbitrary user-provided name for the AppConnector.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Resource labels to represent user provided metadata. **Note**: This field is non-authoritative, and will only manage the
        /// labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels present on the
        /// resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ID of the AppConnector.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Principal information about the Identity of the AppConnector.
        /// Structure is documented below.
        #[builder(into)]
        pub principal_info: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::beyondcorp::AppConnectorPrincipalInfo,
        >,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The region of the AppConnector.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AppConnectorResult {
        /// An arbitrary user-provided name for the AppConnector.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Resource labels to represent user provided metadata. **Note**: This field is non-authoritative, and will only manage the
        /// labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels present on the
        /// resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ID of the AppConnector.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Principal information about the Identity of the AppConnector.
        /// Structure is documented below.
        pub principal_info: pulumi_gestalt_rust::Output<
            super::super::types::beyondcorp::AppConnectorPrincipalInfo,
        >,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The region of the AppConnector.
        pub region: pulumi_gestalt_rust::Output<Option<String>>,
        /// Represents the different states of a AppConnector.
        pub state: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AppConnectorArgs,
    ) -> AppConnectorResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let display_name_binding = args.display_name.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let name_binding = args.name.get_output(context);
        let principal_info_binding = args.principal_info.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:beyondcorp/appConnector:AppConnector".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "principalInfo".into(),
                    value: principal_info_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: region_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AppConnectorResult {
            display_name: o.get_field("displayName"),
            effective_labels: o.get_field("effectiveLabels"),
            labels: o.get_field("labels"),
            name: o.get_field("name"),
            principal_info: o.get_field("principalInfo"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            region: o.get_field("region"),
            state: o.get_field("state"),
        }
    }
}
