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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectionArgs {
        /// authConfig for the connection.
        /// Structure is documented below.
        #[builder(into, default)]
        pub auth_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::integrationconnectors::ConnectionAuthConfig>,
        >,
        /// Config Variables for the connection.
        /// Structure is documented below.
        #[builder(into, default)]
        pub config_variables: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::integrationconnectors::ConnectionConfigVariable>,
            >,
        >,
        /// connectorVersion of the Connector.
        #[builder(into)]
        pub connector_version: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An arbitrary description for the Conection.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Define the Connectors target endpoint.
        /// Structure is documented below.
        #[builder(into, default)]
        pub destination_configs: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::integrationconnectors::ConnectionDestinationConfig,
                >,
            >,
        >,
        /// Eventing Configuration of a connection
        /// Structure is documented below.
        #[builder(into, default)]
        pub eventing_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::integrationconnectors::ConnectionEventingConfig>,
        >,
        /// Eventing enablement type. Will be nil if eventing is not enabled.
        /// Possible values are: `EVENTING_AND_CONNECTION`, `ONLY_EVENTING`.
        #[builder(into, default)]
        pub eventing_enablement_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Resource labels to represent user provided metadata.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Location in which Connection needs to be created.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Determines whether or no a connection is locked. If locked, a reason must be specified.
        /// Structure is documented below.
        #[builder(into, default)]
        pub lock_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::integrationconnectors::ConnectionLockConfig>,
        >,
        /// Log configuration for the connection.
        /// Structure is documented below.
        #[builder(into, default)]
        pub log_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::integrationconnectors::ConnectionLogConfig>,
        >,
        /// Name of Connection needs to be created.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Node configuration for the connection.
        /// Structure is documented below.
        #[builder(into, default)]
        pub node_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::integrationconnectors::ConnectionNodeConfig>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Service account needed for runtime plane to access Google Cloud resources.
        #[builder(into, default)]
        pub service_account: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// SSL Configuration of a connection
        /// Structure is documented below.
        #[builder(into, default)]
        pub ssl_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::integrationconnectors::ConnectionSslConfig>,
        >,
        /// Suspended indicates if a user has suspended a connection or not.
        #[builder(into, default)]
        pub suspended: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct ConnectionResult {
        /// authConfig for the connection.
        /// Structure is documented below.
        pub auth_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::integrationconnectors::ConnectionAuthConfig>,
        >,
        /// Config Variables for the connection.
        /// Structure is documented below.
        pub config_variables: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::types::integrationconnectors::ConnectionConfigVariable>,
            >,
        >,
        /// Connection revision. This field is only updated when the connection is created or updated by User.
        pub connection_revision: pulumi_gestalt_rust::Output<String>,
        /// connectorVersion of the Connector.
        pub connector_version: pulumi_gestalt_rust::Output<String>,
        /// This configuration provides infra configs like rate limit threshold which need to be configurable for every connector version.
        /// Structure is documented below.
        pub connector_version_infra_configs: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::integrationconnectors::ConnectionConnectorVersionInfraConfig,
            >,
        >,
        /// Flag to mark the version indicating the launch stage.
        pub connector_version_launch_stage: pulumi_gestalt_rust::Output<String>,
        /// Time the Namespace was created in UTC.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// An arbitrary description for the Conection.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Define the Connectors target endpoint.
        /// Structure is documented below.
        pub destination_configs: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::integrationconnectors::ConnectionDestinationConfig,
                >,
            >,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Eventing Configuration of a connection
        /// Structure is documented below.
        pub eventing_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::integrationconnectors::ConnectionEventingConfig>,
        >,
        /// Eventing enablement type. Will be nil if eventing is not enabled.
        /// Possible values are: `EVENTING_AND_CONNECTION`, `ONLY_EVENTING`.
        pub eventing_enablement_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Eventing Runtime Data.
        /// Structure is documented below.
        pub eventing_runtime_datas: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::integrationconnectors::ConnectionEventingRuntimeData,
            >,
        >,
        /// Resource labels to represent user provided metadata.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Location in which Connection needs to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Determines whether or no a connection is locked. If locked, a reason must be specified.
        /// Structure is documented below.
        pub lock_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::integrationconnectors::ConnectionLockConfig>,
        >,
        /// Log configuration for the connection.
        /// Structure is documented below.
        pub log_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::integrationconnectors::ConnectionLogConfig>,
        >,
        /// Name of Connection needs to be created.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Node configuration for the connection.
        /// Structure is documented below.
        pub node_config: pulumi_gestalt_rust::Output<
            super::super::types::integrationconnectors::ConnectionNodeConfig,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Service account needed for runtime plane to access Google Cloud resources.
        pub service_account: pulumi_gestalt_rust::Output<String>,
        /// The name of the Service Directory service name. Used for Private Harpoon to resolve the ILB address.
        /// e.g. "projects/cloud-connectors-e2e-testing/locations/us-central1/namespaces/istio-system/services/istio-ingressgateway-connectors"
        pub service_directory: pulumi_gestalt_rust::Output<String>,
        /// SSL Configuration of a connection
        /// Structure is documented below.
        pub ssl_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::integrationconnectors::ConnectionSslConfig>,
        >,
        /// (Output)
        /// Current status of eventing.
        /// Structure is documented below.
        pub statuses: pulumi_gestalt_rust::Output<
            Vec<super::super::types::integrationconnectors::ConnectionStatus>,
        >,
        /// This subscription type enum states the subscription type of the project.
        pub subscription_type: pulumi_gestalt_rust::Output<String>,
        /// Suspended indicates if a user has suspended a connection or not.
        pub suspended: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Time the Namespace was updated in UTC.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ConnectionArgs,
    ) -> ConnectionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let auth_config_binding = args.auth_config.get_output(context);
        let config_variables_binding = args.config_variables.get_output(context);
        let connector_version_binding = args.connector_version.get_output(context);
        let description_binding = args.description.get_output(context);
        let destination_configs_binding = args.destination_configs.get_output(context);
        let eventing_config_binding = args.eventing_config.get_output(context);
        let eventing_enablement_type_binding = args
            .eventing_enablement_type
            .get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let lock_config_binding = args.lock_config.get_output(context);
        let log_config_binding = args.log_config.get_output(context);
        let name_binding = args.name.get_output(context);
        let node_config_binding = args.node_config.get_output(context);
        let project_binding = args.project.get_output(context);
        let service_account_binding = args.service_account.get_output(context);
        let ssl_config_binding = args.ssl_config.get_output(context);
        let suspended_binding = args.suspended.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:integrationconnectors/connection:Connection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authConfig".into(),
                    value: auth_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configVariables".into(),
                    value: config_variables_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectorVersion".into(),
                    value: connector_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destinationConfigs".into(),
                    value: destination_configs_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventingConfig".into(),
                    value: eventing_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventingEnablementType".into(),
                    value: eventing_enablement_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lockConfig".into(),
                    value: lock_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logConfig".into(),
                    value: log_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nodeConfig".into(),
                    value: node_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceAccount".into(),
                    value: service_account_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sslConfig".into(),
                    value: ssl_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "suspended".into(),
                    value: suspended_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ConnectionResult {
            auth_config: o.get_field("authConfig"),
            config_variables: o.get_field("configVariables"),
            connection_revision: o.get_field("connectionRevision"),
            connector_version: o.get_field("connectorVersion"),
            connector_version_infra_configs: o.get_field("connectorVersionInfraConfigs"),
            connector_version_launch_stage: o.get_field("connectorVersionLaunchStage"),
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            destination_configs: o.get_field("destinationConfigs"),
            effective_labels: o.get_field("effectiveLabels"),
            eventing_config: o.get_field("eventingConfig"),
            eventing_enablement_type: o.get_field("eventingEnablementType"),
            eventing_runtime_datas: o.get_field("eventingRuntimeDatas"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            lock_config: o.get_field("lockConfig"),
            log_config: o.get_field("logConfig"),
            name: o.get_field("name"),
            node_config: o.get_field("nodeConfig"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            service_account: o.get_field("serviceAccount"),
            service_directory: o.get_field("serviceDirectory"),
            ssl_config: o.get_field("sslConfig"),
            statuses: o.get_field("statuses"),
            subscription_type: o.get_field("subscriptionType"),
            suspended: o.get_field("suspended"),
            update_time: o.get_field("updateTime"),
        }
    }
}
