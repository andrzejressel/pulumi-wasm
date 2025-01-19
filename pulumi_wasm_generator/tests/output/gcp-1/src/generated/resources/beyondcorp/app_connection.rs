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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod app_connection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AppConnectionArgs {
        /// Address of the remote application endpoint for the BeyondCorp AppConnection.
        /// Structure is documented below.
        #[builder(into)]
        pub application_endpoint: pulumi_wasm_rust::Output<
            super::super::types::beyondcorp::AppConnectionApplicationEndpoint,
        >,
        /// List of AppConnectors that are authorised to be associated with this AppConnection
        #[builder(into, default)]
        pub connectors: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// An arbitrary user-provided name for the AppConnection.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Gateway used by the AppConnection.
        #[builder(into, default)]
        pub gateway: pulumi_wasm_rust::Output<
            Option<super::super::types::beyondcorp::AppConnectionGateway>,
        >,
        /// Resource labels to represent user provided metadata. **Note**: This field is non-authoritative, and will only manage the
        /// labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels present on the
        /// resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ID of the AppConnection.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The region of the AppConnection.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::Output<Option<String>>,
        /// The type of network connectivity used by the AppConnection. Refer to
        /// https://cloud.google.com/beyondcorp/docs/reference/rest/v1/projects.locations.appConnections#type for a list of possible
        /// values.
        #[builder(into, default)]
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AppConnectionResult {
        /// Address of the remote application endpoint for the BeyondCorp AppConnection.
        /// Structure is documented below.
        pub application_endpoint: pulumi_wasm_rust::Output<
            super::super::types::beyondcorp::AppConnectionApplicationEndpoint,
        >,
        /// List of AppConnectors that are authorised to be associated with this AppConnection
        pub connectors: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// An arbitrary user-provided name for the AppConnection.
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Gateway used by the AppConnection.
        pub gateway: pulumi_wasm_rust::Output<
            super::super::types::beyondcorp::AppConnectionGateway,
        >,
        /// Resource labels to represent user provided metadata. **Note**: This field is non-authoritative, and will only manage the
        /// labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels present on the
        /// resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ID of the AppConnection.
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The region of the AppConnection.
        pub region: pulumi_wasm_rust::Output<Option<String>>,
        /// The type of network connectivity used by the AppConnection. Refer to
        /// https://cloud.google.com/beyondcorp/docs/reference/rest/v1/projects.locations.appConnections#type for a list of possible
        /// values.
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AppConnectionArgs) -> AppConnectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_endpoint_binding = args.application_endpoint.get_inner();
        let connectors_binding = args.connectors.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let gateway_binding = args.gateway.get_inner();
        let labels_binding = args.labels.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let region_binding = args.region.get_inner();
        let type__binding = args.type_.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:beyondcorp/appConnection:AppConnection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationEndpoint".into(),
                    value: &application_endpoint_binding,
                },
                register_interface::ObjectField {
                    name: "connectors".into(),
                    value: &connectors_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "gateway".into(),
                    value: &gateway_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "applicationEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "connectors".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "gateway".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AppConnectionResult {
            application_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationEndpoint").unwrap(),
            ),
            connectors: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectors").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            gateway: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gateway").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
