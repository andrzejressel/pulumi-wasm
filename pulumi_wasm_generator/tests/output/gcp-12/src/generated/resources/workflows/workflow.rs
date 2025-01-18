/// Workflow program to be executed by Workflows.
///
///
/// To get more information about Workflow, see:
///
/// * [API documentation](https://cloud.google.com/workflows/docs/reference/rest/v1/projects.locations.workflows)
/// * How-to Guides
///     * [Managing Workflows](https://cloud.google.com/workflows/docs/creating-updating-workflow)
///
/// ## Example Usage
///
/// ### Workflow Basic
///
///
/// ```yaml
/// resources:
///   testAccount:
///     type: gcp:serviceaccount:Account
///     name: test_account
///     properties:
///       accountId: my-account
///       displayName: Test Service Account
///   example:
///     type: gcp:workflows:Workflow
///     properties:
///       name: workflow
///       region: us-central1
///       description: Magic
///       serviceAccount: ${testAccount.id}
///       callLogLevel: LOG_ERRORS_ONLY
///       labels:
///         env: test
///       userEnvVars:
///         url: https://timeapi.io/api/Time/current/zone?timeZone=Europe/Amsterdam
///       deletionProtection: false
///       sourceContents: |
///         # This is a sample workflow. You can replace it with your source code.
///         #
///         # This workflow does the following:
///         # - reads current time and date information from an external API and stores
///         #   the response in currentTime variable
///         # - retrieves a list of Wikipedia articles related to the day of the week
///         #   from currentTime
///         # - returns the list of articles as an output of the workflow
///         #
///         # Note: In Terraform you need to escape the $$ or it will cause errors.
///
///         - getCurrentTime:
///             call: http.get
///             args:
///                 url: $${sys.get_env("url")}
///             result: currentTime
///         - readWikipedia:
///             call: http.get
///             args:
///                 url: https://en.wikipedia.org/w/api.php
///                 query:
///                     action: opensearch
///                     search: $${currentTime.body.dayOfWeek}
///             result: wikiResult
///         - returnOutput:
///             return: $${wikiResult.body[1]}
/// ```
///
/// ## Import
///
/// This resource does not support import.
///
pub mod workflow {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkflowArgs {
        /// Describes the level of platform logging to apply to calls and call responses during
        /// executions of this workflow. If both the workflow and the execution specify a logging level,
        /// the execution level takes precedence.
        /// Possible values are: `CALL_LOG_LEVEL_UNSPECIFIED`, `LOG_ALL_CALLS`, `LOG_ERRORS_ONLY`, `LOG_NONE`.
        #[builder(into, default)]
        pub call_log_level: pulumi_wasm_rust::Output<Option<String>>,
        /// The KMS key used to encrypt workflow and execution data.
        /// Format: projects/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{cryptoKey}
        #[builder(into, default)]
        pub crypto_key_name: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub deletion_protection: pulumi_wasm_rust::Output<Option<bool>>,
        /// Description of the workflow provided by the user. Must be at most 1000 unicode characters long.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A set of key/value label pairs to assign to this Workflow.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the Workflow.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Creates a unique name beginning with the
        /// specified prefix. If this and name are unspecified, a random value is chosen for the name.
        #[builder(into, default)]
        pub name_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The region of the workflow.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the service account associated with the latest workflow version. This service
        /// account represents the identity of the workflow and determines what permissions the workflow has.
        /// Format: projects/{project}/serviceAccounts/{account} or {account}.
        /// Using - as a wildcard for the {project} or not providing one at all will infer the project from the account.
        /// The {account} value can be the email address or the unique_id of the service account.
        /// If not provided, workflow will use the project's default service account.
        /// Modifying this field for an existing workflow results in a new workflow revision.
        #[builder(into, default)]
        pub service_account: pulumi_wasm_rust::Output<Option<String>>,
        /// Workflow code to be executed. The size limit is 128KB.
        #[builder(into, default)]
        pub source_contents: pulumi_wasm_rust::Output<Option<String>>,
        /// User-defined environment variables associated with this workflow revision. This map has a maximum length of 20. Each string can take up to 4KiB. Keys cannot be empty strings and cannot start with “GOOGLE” or “WORKFLOWS".
        #[builder(into, default)]
        pub user_env_vars: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct WorkflowResult {
        /// Describes the level of platform logging to apply to calls and call responses during
        /// executions of this workflow. If both the workflow and the execution specify a logging level,
        /// the execution level takes precedence.
        /// Possible values are: `CALL_LOG_LEVEL_UNSPECIFIED`, `LOG_ALL_CALLS`, `LOG_ERRORS_ONLY`, `LOG_NONE`.
        pub call_log_level: pulumi_wasm_rust::Output<Option<String>>,
        /// The timestamp of when the workflow was created in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// The KMS key used to encrypt workflow and execution data.
        /// Format: projects/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{cryptoKey}
        pub crypto_key_name: pulumi_wasm_rust::Output<Option<String>>,
        pub deletion_protection: pulumi_wasm_rust::Output<Option<bool>>,
        /// Description of the workflow provided by the user. Must be at most 1000 unicode characters long.
        pub description: pulumi_wasm_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A set of key/value label pairs to assign to this Workflow.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the Workflow.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Creates a unique name beginning with the
        /// specified prefix. If this and name are unspecified, a random value is chosen for the name.
        pub name_prefix: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The region of the workflow.
        pub region: pulumi_wasm_rust::Output<Option<String>>,
        /// The revision of the workflow. A new one is generated if the service account or source contents is changed.
        pub revision_id: pulumi_wasm_rust::Output<String>,
        /// Name of the service account associated with the latest workflow version. This service
        /// account represents the identity of the workflow and determines what permissions the workflow has.
        /// Format: projects/{project}/serviceAccounts/{account} or {account}.
        /// Using - as a wildcard for the {project} or not providing one at all will infer the project from the account.
        /// The {account} value can be the email address or the unique_id of the service account.
        /// If not provided, workflow will use the project's default service account.
        /// Modifying this field for an existing workflow results in a new workflow revision.
        pub service_account: pulumi_wasm_rust::Output<String>,
        /// Workflow code to be executed. The size limit is 128KB.
        pub source_contents: pulumi_wasm_rust::Output<Option<String>>,
        /// State of the workflow deployment.
        pub state: pulumi_wasm_rust::Output<String>,
        /// The timestamp of when the workflow was last updated in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        pub update_time: pulumi_wasm_rust::Output<String>,
        /// User-defined environment variables associated with this workflow revision. This map has a maximum length of 20. Each string can take up to 4KiB. Keys cannot be empty strings and cannot start with “GOOGLE” or “WORKFLOWS".
        pub user_env_vars: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: WorkflowArgs) -> WorkflowResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let call_log_level_binding = args.call_log_level.get_inner();
        let crypto_key_name_binding = args.crypto_key_name.get_inner();
        let deletion_protection_binding = args.deletion_protection.get_inner();
        let description_binding = args.description.get_inner();
        let labels_binding = args.labels.get_inner();
        let name_binding = args.name.get_inner();
        let name_prefix_binding = args.name_prefix.get_inner();
        let project_binding = args.project.get_inner();
        let region_binding = args.region.get_inner();
        let service_account_binding = args.service_account.get_inner();
        let source_contents_binding = args.source_contents.get_inner();
        let user_env_vars_binding = args.user_env_vars.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:workflows/workflow:Workflow".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "callLogLevel".into(),
                    value: &call_log_level_binding,
                },
                register_interface::ObjectField {
                    name: "cryptoKeyName".into(),
                    value: &crypto_key_name_binding,
                },
                register_interface::ObjectField {
                    name: "deletionProtection".into(),
                    value: &deletion_protection_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
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
                    name: "namePrefix".into(),
                    value: &name_prefix_binding,
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
                    name: "serviceAccount".into(),
                    value: &service_account_binding,
                },
                register_interface::ObjectField {
                    name: "sourceContents".into(),
                    value: &source_contents_binding,
                },
                register_interface::ObjectField {
                    name: "userEnvVars".into(),
                    value: &user_env_vars_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "callLogLevel".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "cryptoKeyName".into(),
                },
                register_interface::ResultField {
                    name: "deletionProtection".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "namePrefix".into(),
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
                    name: "revisionId".into(),
                },
                register_interface::ResultField {
                    name: "serviceAccount".into(),
                },
                register_interface::ResultField {
                    name: "sourceContents".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
                register_interface::ResultField {
                    name: "userEnvVars".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        WorkflowResult {
            call_log_level: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("callLogLevel").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            crypto_key_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cryptoKeyName").unwrap(),
            ),
            deletion_protection: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deletionProtection").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            name_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namePrefix").unwrap(),
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
            revision_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("revisionId").unwrap(),
            ),
            service_account: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceAccount").unwrap(),
            ),
            source_contents: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceContents").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
            user_env_vars: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userEnvVars").unwrap(),
            ),
        }
    }
}
