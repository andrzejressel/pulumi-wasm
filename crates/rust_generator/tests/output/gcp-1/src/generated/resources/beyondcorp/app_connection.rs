/// A BeyondCorp AppConnection resource represents a BeyondCorp protected AppConnection to a remote application.
/// It creates all the necessary GCP components needed for creating a BeyondCorp protected AppConnection.
/// Multiple connectors can be authorised for a single AppConnection.
///
///
/// To get more information about AppConnection, see:
///
/// * [API documentation](https://cloud.google.com/beyondcorp/docs/reference/rest#rest-resource:-v1.projects.locations.appconnections)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/beyondcorp-enterprise/docs/enable-app-connector)
///
/// ## Example Usage
///
/// ### Beyondcorp App Connection Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let appConnection = app_connection::create(
///         "appConnection",
///         AppConnectionArgs::builder()
///             .application_endpoint(
///                 AppConnectionApplicationEndpoint::builder()
///                     .host("foo-host")
///                     .port(8080)
///                     .build_struct(),
///             )
///             .connectors(vec!["${appConnector.id}",])
///             .name("my-app-connection")
///             .type_("TCP_PROXY")
///             .build_struct(),
///     );
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
/// ### Beyondcorp App Connection Full
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
///   appGateway:
///     type: gcp:beyondcorp:AppGateway
///     name: app_gateway
///     properties:
///       name: my-app-gateway
///       type: TCP_PROXY
///       hostType: GCP_REGIONAL_MIG
///   appConnector:
///     type: gcp:beyondcorp:AppConnector
///     name: app_connector
///     properties:
///       name: my-app-connector
///       principalInfo:
///         serviceAccount:
///           email: ${serviceAccount.email}
///   appConnection:
///     type: gcp:beyondcorp:AppConnection
///     name: app_connection
///     properties:
///       name: my-app-connection
///       type: TCP_PROXY
///       displayName: some display name
///       applicationEndpoint:
///         host: foo-host
///         port: 8080
///       connectors:
///         - ${appConnector.id}
///       gateway:
///         appGateway: ${appGateway.id}
///       labels:
///         foo: bar
///         bar: baz
/// ```
///
/// ## Import
///
/// AppConnection can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{region}}/appConnections/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, AppConnection can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:beyondcorp/appConnection:AppConnection default projects/{{project}}/locations/{{region}}/appConnections/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:beyondcorp/appConnection:AppConnection default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:beyondcorp/appConnection:AppConnection default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:beyondcorp/appConnection:AppConnection default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod app_connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AppConnectionArgs {
        /// Address of the remote application endpoint for the BeyondCorp AppConnection.
        /// Structure is documented below.
        #[builder(into)]
        pub application_endpoint: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::beyondcorp::AppConnectionApplicationEndpoint,
        >,
        /// List of AppConnectors that are authorised to be associated with this AppConnection
        #[builder(into, default)]
        pub connectors: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// An arbitrary user-provided name for the AppConnection.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Gateway used by the AppConnection.
        #[builder(into, default)]
        pub gateway: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::beyondcorp::AppConnectionGateway>,
        >,
        /// Resource labels to represent user provided metadata. **Note**: This field is non-authoritative, and will only manage the
        /// labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels present on the
        /// resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ID of the AppConnection.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The region of the AppConnection.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The type of network connectivity used by the AppConnection. Refer to
        /// https://cloud.google.com/beyondcorp/docs/reference/rest/v1/projects.locations.appConnections#type for a list of possible
        /// values.
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AppConnectionResult {
        /// Address of the remote application endpoint for the BeyondCorp AppConnection.
        /// Structure is documented below.
        pub application_endpoint: pulumi_gestalt_rust::Output<
            super::super::types::beyondcorp::AppConnectionApplicationEndpoint,
        >,
        /// List of AppConnectors that are authorised to be associated with this AppConnection
        pub connectors: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// An arbitrary user-provided name for the AppConnection.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Gateway used by the AppConnection.
        pub gateway: pulumi_gestalt_rust::Output<
            super::super::types::beyondcorp::AppConnectionGateway,
        >,
        /// Resource labels to represent user provided metadata. **Note**: This field is non-authoritative, and will only manage the
        /// labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels present on the
        /// resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ID of the AppConnection.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The region of the AppConnection.
        pub region: pulumi_gestalt_rust::Output<Option<String>>,
        /// The type of network connectivity used by the AppConnection. Refer to
        /// https://cloud.google.com/beyondcorp/docs/reference/rest/v1/projects.locations.appConnections#type for a list of possible
        /// values.
        pub type_: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AppConnectionArgs,
    ) -> AppConnectionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let application_endpoint_binding = args.application_endpoint.get_output(context);
        let connectors_binding = args.connectors.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let gateway_binding = args.gateway.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:beyondcorp/appConnection:AppConnection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationEndpoint".into(),
                    value: &application_endpoint_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectors".into(),
                    value: &connectors_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gateway".into(),
                    value: &gateway_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: &region_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AppConnectionResult {
            application_endpoint: o.get_field("applicationEndpoint"),
            connectors: o.get_field("connectors"),
            display_name: o.get_field("displayName"),
            effective_labels: o.get_field("effectiveLabels"),
            gateway: o.get_field("gateway"),
            labels: o.get_field("labels"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            region: o.get_field("region"),
            type_: o.get_field("type"),
        }
    }
}
