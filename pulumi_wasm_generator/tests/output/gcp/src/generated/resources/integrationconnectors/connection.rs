/// An Integration connectors Connection.
///
///
/// To get more information about Connection, see:
///
/// * [API documentation](https://cloud.google.com/integration-connectors/docs/reference/rest/v1/projects.locations.connections)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/integration-connectors/docs/createconnection)
///
/// ## Example Usage
///
/// ### Integration Connectors Connection Basic
///
///
/// ```yaml
/// resources:
///   pubsubconnection:
///     type: gcp:integrationconnectors:Connection
///     properties:
///       name: test-pubsub
///       location: us-central1
///       connectorVersion: projects/${testProject.projectId}/locations/global/providers/gcp/connectors/pubsub/versions/1
///       description: tf created description
///       configVariables:
///         - key: project_id
///           stringValue: connectors-example
///         - key: topic_id
///           stringValue: test
/// variables:
///   testProject:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Integration Connectors Connection Advanced
///
///
/// ```yaml
/// resources:
///   secret-basic:
///     type: gcp:secretmanager:Secret
///     properties:
///       secretId: test-secret
///       replication:
///         userManaged:
///           replicas:
///             - location: us-central1
///   secret-version-basic:
///     type: gcp:secretmanager:SecretVersion
///     properties:
///       secret: ${["secret-basic"].id}
///       secretData: dummypassword
///   secretIam:
///     type: gcp:secretmanager:SecretIamMember
///     name: secret_iam
///     properties:
///       secretId: ${["secret-basic"].id}
///       role: roles/secretmanager.admin
///       member: serviceAccount:${testProject.number}-compute@developer.gserviceaccount.com
///     options:
///       dependsOn:
///         - ${["secret-version-basic"]}
///   zendeskconnection:
///     type: gcp:integrationconnectors:Connection
///     properties:
///       name: test-zendesk
///       description: tf updated description
///       location: us-central1
///       serviceAccount: ${testProject.number}-compute@developer.gserviceaccount.com
///       connectorVersion: projects/${testProject.projectId}/locations/global/providers/zendesk/connectors/zendesk/versions/1
///       configVariables:
///         - key: proxy_enabled
///           booleanValue: false
///         - key: sample_integer_value
///           integerValue: 1
///         - key: sample_encryption_key_value
///           encryptionKeyValue:
///             type: GOOGLE_MANAGED
///             kmsKeyName: sampleKMSKkey
///         - key: sample_secret_value
///           secretValue:
///             secretVersion: ${["secret-version-basic"].name}
///       suspended: false
///       authConfig:
///         additionalVariables:
///           - key: sample_string
///             stringValue: sampleString
///           - key: sample_boolean
///             booleanValue: false
///           - key: sample_integer
///             integerValue: 1
///           - key: sample_secret_value
///             secretValue:
///               secretVersion: ${["secret-version-basic"].name}
///           - key: sample_encryption_key_value
///             encryptionKeyValue:
///               type: GOOGLE_MANAGED
///               kmsKeyName: sampleKMSKkey
///         authType: USER_PASSWORD
///         authKey: sampleAuthKey
///         userPassword:
///           username: user@xyz.com
///           password:
///             secretVersion: ${["secret-version-basic"].name}
///       destinationConfigs:
///         - key: url
///           destinations:
///             - host: https://test.zendesk.com
///               port: 80
///       lockConfig:
///         locked: false
///         reason: Its not locked
///       logConfig:
///         enabled: true
///       nodeConfig:
///         minNodeCount: 2
///         maxNodeCount: 50
///       labels:
///         foo: bar
///       sslConfig:
///         additionalVariables:
///           - key: sample_string
///             stringValue: sampleString
///           - key: sample_boolean
///             booleanValue: false
///           - key: sample_integer
///             integerValue: 1
///           - key: sample_secret_value
///             secretValue:
///               secretVersion: ${["secret-version-basic"].name}
///           - key: sample_encryption_key_value
///             encryptionKeyValue:
///               type: GOOGLE_MANAGED
///               kmsKeyName: sampleKMSKkey
///         clientCertType: PEM
///         clientCertificate:
///           secretVersion: ${["secret-version-basic"].name}
///         clientPrivateKey:
///           secretVersion: ${["secret-version-basic"].name}
///         clientPrivateKeyPass:
///           secretVersion: ${["secret-version-basic"].name}
///         privateServerCertificate:
///           secretVersion: ${["secret-version-basic"].name}
///         serverCertType: PEM
///         trustModel: PRIVATE
///         type: TLS
///         useSsl: true
///       eventingEnablementType: EVENTING_AND_CONNECTION
///       eventingConfig:
///         additionalVariables:
///           - key: sample_string
///             stringValue: sampleString
///           - key: sample_boolean
///             booleanValue: false
///           - key: sample_integer
///             integerValue: 1
///           - key: sample_secret_value
///             secretValue:
///               secretVersion: ${["secret-version-basic"].name}
///           - key: sample_encryption_key_value
///             encryptionKeyValue:
///               type: GOOGLE_MANAGED
///               kmsKeyName: sampleKMSKkey
///         registrationDestinationConfig:
///           key: registration_destination_config
///           destinations:
///             - host: https://test.zendesk.com
///               port: 80
///         authConfig:
///           authType: USER_PASSWORD
///           authKey: sampleAuthKey
///           userPassword:
///             username: user@xyz.com
///             password:
///               secretVersion: ${["secret-version-basic"].name}
///           additionalVariables:
///             - key: sample_string
///               stringValue: sampleString
///             - key: sample_boolean
///               booleanValue: false
///             - key: sample_integer
///               integerValue: 1
///             - key: sample_secret_value
///               secretValue:
///                 secretVersion: ${["secret-version-basic"].name}
///             - key: sample_encryption_key_value
///               encryptionKeyValue:
///                 type: GOOGLE_MANAGED
///                 kmsKeyName: sampleKMSKkey
///         enrichmentEnabled: true
/// variables:
///   testProject:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// Connection can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/connections/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, Connection can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:integrationconnectors/connection:Connection default projects/{{project}}/locations/{{location}}/connections/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:integrationconnectors/connection:Connection default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:integrationconnectors/connection:Connection default {{location}}/{{name}}
/// ```
///
pub mod connection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectionArgs {
        /// authConfig for the connection.
        /// Structure is documented below.
        #[builder(into, default)]
        pub auth_config: pulumi_wasm_rust::Output<
            Option<super::super::types::integrationconnectors::ConnectionAuthConfig>,
        >,
        /// Config Variables for the connection.
        /// Structure is documented below.
        #[builder(into, default)]
        pub config_variables: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::integrationconnectors::ConnectionConfigVariable>,
            >,
        >,
        /// connectorVersion of the Connector.
        #[builder(into)]
        pub connector_version: pulumi_wasm_rust::Output<String>,
        /// An arbitrary description for the Conection.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Define the Connectors target endpoint.
        /// Structure is documented below.
        #[builder(into, default)]
        pub destination_configs: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::integrationconnectors::ConnectionDestinationConfig,
                >,
            >,
        >,
        /// Eventing Configuration of a connection
        /// Structure is documented below.
        #[builder(into, default)]
        pub eventing_config: pulumi_wasm_rust::Output<
            Option<super::super::types::integrationconnectors::ConnectionEventingConfig>,
        >,
        /// Eventing enablement type. Will be nil if eventing is not enabled.
        /// Possible values are: `EVENTING_AND_CONNECTION`, `ONLY_EVENTING`.
        #[builder(into, default)]
        pub eventing_enablement_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Resource labels to represent user provided metadata.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Location in which Connection needs to be created.
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// Determines whether or no a connection is locked. If locked, a reason must be specified.
        /// Structure is documented below.
        #[builder(into, default)]
        pub lock_config: pulumi_wasm_rust::Output<
            Option<super::super::types::integrationconnectors::ConnectionLockConfig>,
        >,
        /// Log configuration for the connection.
        /// Structure is documented below.
        #[builder(into, default)]
        pub log_config: pulumi_wasm_rust::Output<
            Option<super::super::types::integrationconnectors::ConnectionLogConfig>,
        >,
        /// Name of Connection needs to be created.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Node configuration for the connection.
        /// Structure is documented below.
        #[builder(into, default)]
        pub node_config: pulumi_wasm_rust::Output<
            Option<super::super::types::integrationconnectors::ConnectionNodeConfig>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// Service account needed for runtime plane to access Google Cloud resources.
        #[builder(into, default)]
        pub service_account: pulumi_wasm_rust::Output<Option<String>>,
        /// SSL Configuration of a connection
        /// Structure is documented below.
        #[builder(into, default)]
        pub ssl_config: pulumi_wasm_rust::Output<
            Option<super::super::types::integrationconnectors::ConnectionSslConfig>,
        >,
        /// Suspended indicates if a user has suspended a connection or not.
        #[builder(into, default)]
        pub suspended: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct ConnectionResult {
        /// authConfig for the connection.
        /// Structure is documented below.
        pub auth_config: pulumi_wasm_rust::Output<
            Option<super::super::types::integrationconnectors::ConnectionAuthConfig>,
        >,
        /// Config Variables for the connection.
        /// Structure is documented below.
        pub config_variables: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::integrationconnectors::ConnectionConfigVariable>,
            >,
        >,
        /// Connection revision. This field is only updated when the connection is created or updated by User.
        pub connection_revision: pulumi_wasm_rust::Output<String>,
        /// connectorVersion of the Connector.
        pub connector_version: pulumi_wasm_rust::Output<String>,
        /// This configuration provides infra configs like rate limit threshold which need to be configurable for every connector version.
        /// Structure is documented below.
        pub connector_version_infra_configs: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::integrationconnectors::ConnectionConnectorVersionInfraConfig,
            >,
        >,
        /// Flag to mark the version indicating the launch stage.
        pub connector_version_launch_stage: pulumi_wasm_rust::Output<String>,
        /// Time the Namespace was created in UTC.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// An arbitrary description for the Conection.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Define the Connectors target endpoint.
        /// Structure is documented below.
        pub destination_configs: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::integrationconnectors::ConnectionDestinationConfig,
                >,
            >,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Eventing Configuration of a connection
        /// Structure is documented below.
        pub eventing_config: pulumi_wasm_rust::Output<
            Option<super::super::types::integrationconnectors::ConnectionEventingConfig>,
        >,
        /// Eventing enablement type. Will be nil if eventing is not enabled.
        /// Possible values are: `EVENTING_AND_CONNECTION`, `ONLY_EVENTING`.
        pub eventing_enablement_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Eventing Runtime Data.
        /// Structure is documented below.
        pub eventing_runtime_datas: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::integrationconnectors::ConnectionEventingRuntimeData,
            >,
        >,
        /// Resource labels to represent user provided metadata.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Location in which Connection needs to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Determines whether or no a connection is locked. If locked, a reason must be specified.
        /// Structure is documented below.
        pub lock_config: pulumi_wasm_rust::Output<
            Option<super::super::types::integrationconnectors::ConnectionLockConfig>,
        >,
        /// Log configuration for the connection.
        /// Structure is documented below.
        pub log_config: pulumi_wasm_rust::Output<
            Option<super::super::types::integrationconnectors::ConnectionLogConfig>,
        >,
        /// Name of Connection needs to be created.
        ///
        ///
        /// - - -
        pub name: pulumi_wasm_rust::Output<String>,
        /// Node configuration for the connection.
        /// Structure is documented below.
        pub node_config: pulumi_wasm_rust::Output<
            super::super::types::integrationconnectors::ConnectionNodeConfig,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Service account needed for runtime plane to access Google Cloud resources.
        pub service_account: pulumi_wasm_rust::Output<String>,
        /// The name of the Service Directory service name. Used for Private Harpoon to resolve the ILB address.
        /// e.g. "projects/cloud-connectors-e2e-testing/locations/us-central1/namespaces/istio-system/services/istio-ingressgateway-connectors"
        pub service_directory: pulumi_wasm_rust::Output<String>,
        /// SSL Configuration of a connection
        /// Structure is documented below.
        pub ssl_config: pulumi_wasm_rust::Output<
            Option<super::super::types::integrationconnectors::ConnectionSslConfig>,
        >,
        /// (Output)
        /// Current status of eventing.
        /// Structure is documented below.
        pub statuses: pulumi_wasm_rust::Output<
            Vec<super::super::types::integrationconnectors::ConnectionStatus>,
        >,
        /// This subscription type enum states the subscription type of the project.
        pub subscription_type: pulumi_wasm_rust::Output<String>,
        /// Suspended indicates if a user has suspended a connection or not.
        pub suspended: pulumi_wasm_rust::Output<Option<bool>>,
        /// Time the Namespace was updated in UTC.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ConnectionArgs) -> ConnectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let auth_config_binding = args.auth_config.get_inner();
        let config_variables_binding = args.config_variables.get_inner();
        let connector_version_binding = args.connector_version.get_inner();
        let description_binding = args.description.get_inner();
        let destination_configs_binding = args.destination_configs.get_inner();
        let eventing_config_binding = args.eventing_config.get_inner();
        let eventing_enablement_type_binding = args.eventing_enablement_type.get_inner();
        let labels_binding = args.labels.get_inner();
        let location_binding = args.location.get_inner();
        let lock_config_binding = args.lock_config.get_inner();
        let log_config_binding = args.log_config.get_inner();
        let name_binding = args.name.get_inner();
        let node_config_binding = args.node_config.get_inner();
        let project_binding = args.project.get_inner();
        let service_account_binding = args.service_account.get_inner();
        let ssl_config_binding = args.ssl_config.get_inner();
        let suspended_binding = args.suspended.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:integrationconnectors/connection:Connection".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "authConfig".into(),
                    value: &auth_config_binding,
                },
                register_interface::ObjectField {
                    name: "configVariables".into(),
                    value: &config_variables_binding,
                },
                register_interface::ObjectField {
                    name: "connectorVersion".into(),
                    value: &connector_version_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "destinationConfigs".into(),
                    value: &destination_configs_binding,
                },
                register_interface::ObjectField {
                    name: "eventingConfig".into(),
                    value: &eventing_config_binding,
                },
                register_interface::ObjectField {
                    name: "eventingEnablementType".into(),
                    value: &eventing_enablement_type_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "lockConfig".into(),
                    value: &lock_config_binding,
                },
                register_interface::ObjectField {
                    name: "logConfig".into(),
                    value: &log_config_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "nodeConfig".into(),
                    value: &node_config_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "serviceAccount".into(),
                    value: &service_account_binding,
                },
                register_interface::ObjectField {
                    name: "sslConfig".into(),
                    value: &ssl_config_binding,
                },
                register_interface::ObjectField {
                    name: "suspended".into(),
                    value: &suspended_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "authConfig".into(),
                },
                register_interface::ResultField {
                    name: "configVariables".into(),
                },
                register_interface::ResultField {
                    name: "connectionRevision".into(),
                },
                register_interface::ResultField {
                    name: "connectorVersion".into(),
                },
                register_interface::ResultField {
                    name: "connectorVersionInfraConfigs".into(),
                },
                register_interface::ResultField {
                    name: "connectorVersionLaunchStage".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "destinationConfigs".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "eventingConfig".into(),
                },
                register_interface::ResultField {
                    name: "eventingEnablementType".into(),
                },
                register_interface::ResultField {
                    name: "eventingRuntimeDatas".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "lockConfig".into(),
                },
                register_interface::ResultField {
                    name: "logConfig".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "nodeConfig".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "serviceAccount".into(),
                },
                register_interface::ResultField {
                    name: "serviceDirectory".into(),
                },
                register_interface::ResultField {
                    name: "sslConfig".into(),
                },
                register_interface::ResultField {
                    name: "statuses".into(),
                },
                register_interface::ResultField {
                    name: "subscriptionType".into(),
                },
                register_interface::ResultField {
                    name: "suspended".into(),
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
        ConnectionResult {
            auth_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authConfig").unwrap(),
            ),
            config_variables: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configVariables").unwrap(),
            ),
            connection_revision: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionRevision").unwrap(),
            ),
            connector_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectorVersion").unwrap(),
            ),
            connector_version_infra_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectorVersionInfraConfigs").unwrap(),
            ),
            connector_version_launch_stage: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectorVersionLaunchStage").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            destination_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationConfigs").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            eventing_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventingConfig").unwrap(),
            ),
            eventing_enablement_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventingEnablementType").unwrap(),
            ),
            eventing_runtime_datas: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventingRuntimeDatas").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            lock_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lockConfig").unwrap(),
            ),
            log_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logConfig").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            node_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodeConfig").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            service_account: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceAccount").unwrap(),
            ),
            service_directory: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceDirectory").unwrap(),
            ),
            ssl_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sslConfig").unwrap(),
            ),
            statuses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statuses").unwrap(),
            ),
            subscription_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subscriptionType").unwrap(),
            ),
            suspended: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("suspended").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
