/// Application Integration Client.
///
///
/// To get more information about Client, see:
///
/// * [API documentation](https://cloud.google.com/application-integration/docs/reference/rest/v1/projects.locations.clients)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/application-integration/docs/overview)
///     * [Set up Application Integration](https://cloud.google.com/application-integration/docs/setup-application-integration)
///
/// ## Example Usage
///
/// ### Integrations Client Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = client::create(
///         "example",
///         ClientArgs::builder().location("us-central1").build_struct(),
///     );
/// }
/// ```
/// ### Integrations Client Full
///
///
/// ```yaml
/// resources:
///   keyring:
///     type: gcp:kms:KeyRing
///     properties:
///       name: my-keyring
///       location: us-east1
///   cryptokey:
///     type: gcp:kms:CryptoKey
///     properties:
///       name: crypto-key-example
///       keyRing: ${keyring.id}
///       rotationPeriod: 7776000s
///   testKey:
///     type: gcp:kms:CryptoKeyVersion
///     name: test_key
///     properties:
///       cryptoKey: ${cryptokey.id}
///   serviceAccount:
///     type: gcp:serviceaccount:Account
///     name: service_account
///     properties:
///       accountId: service-acc
///       displayName: Service Account
///   example:
///     type: gcp:applicationintegration:Client
///     properties:
///       location: us-east1
///       createSampleIntegrations: true
///       runAsServiceAccount: ${serviceAccount.email}
///       cloudKmsConfig:
///         kmsLocation: us-east1
///         kmsRing:
///           fn::invoke:
///             function: std:basename
///             arguments:
///               input: ${keyring.id}
///             return: result
///         key:
///           fn::invoke:
///             function: std:basename
///             arguments:
///               input: ${cryptokey.id}
///             return: result
///         keyVersion:
///           fn::invoke:
///             function: std:basename
///             arguments:
///               input: ${testKey.id}
///             return: result
///         kmsProjectId: ${testProject.projectId}
/// variables:
///   testProject:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// Client can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/clients`
///
/// * `{{project}}/{{location}}`
///
/// * `{{location}}`
///
/// When using the `pulumi import` command, Client can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:applicationintegration/client:Client default projects/{{project}}/locations/{{location}}/clients
/// ```
///
/// ```sh
/// $ pulumi import gcp:applicationintegration/client:Client default {{project}}/{{location}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:applicationintegration/client:Client default {{location}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod client {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClientArgs {
        /// Cloud KMS config for AuthModule to encrypt/decrypt credentials.
        /// Structure is documented below.
        #[builder(into, default)]
        pub cloud_kms_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::applicationintegration::ClientCloudKmsConfig>,
        >,
        /// Indicates if sample integrations should be created along with provisioning.
        #[builder(into, default)]
        pub create_sample_integrations: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Location in which client needs to be provisioned.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// User input run-as service account, if empty, will bring up a new default service account.
        #[builder(into, default)]
        pub run_as_service_account: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ClientResult {
        /// Cloud KMS config for AuthModule to encrypt/decrypt credentials.
        /// Structure is documented below.
        pub cloud_kms_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::applicationintegration::ClientCloudKmsConfig>,
        >,
        /// Indicates if sample integrations should be created along with provisioning.
        pub create_sample_integrations: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Location in which client needs to be provisioned.
        ///
        ///
        /// - - -
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// User input run-as service account, if empty, will bring up a new default service account.
        pub run_as_service_account: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ClientArgs,
    ) -> ClientResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cloud_kms_config_binding = args.cloud_kms_config.get_output(context);
        let create_sample_integrations_binding = args
            .create_sample_integrations
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let project_binding = args.project.get_output(context);
        let run_as_service_account_binding = args
            .run_as_service_account
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:applicationintegration/client:Client".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cloudKmsConfig".into(),
                    value: cloud_kms_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "createSampleIntegrations".into(),
                    value: create_sample_integrations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "runAsServiceAccount".into(),
                    value: run_as_service_account_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ClientResult {
            cloud_kms_config: o.get_field("cloudKmsConfig"),
            create_sample_integrations: o.get_field("createSampleIntegrations"),
            location: o.get_field("location"),
            project: o.get_field("project"),
            run_as_service_account: o.get_field("runAsServiceAccount"),
        }
    }
}
