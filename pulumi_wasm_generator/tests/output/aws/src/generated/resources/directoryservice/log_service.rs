/// Provides a Log subscription for AWS Directory Service that pushes logs to cloudwatch.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:cloudwatch:LogGroup
///     properties:
///       name: /aws/directoryservice/${exampleAwsDirectoryServiceDirectory.id}
///       retentionInDays: 14
///   ad-log-policyLogResourcePolicy:
///     type: aws:cloudwatch:LogResourcePolicy
///     name: ad-log-policy
///     properties:
///       policyDocument: ${["ad-log-policy"].json}
///       policyName: ad-log-policy
///   exampleLogService:
///     type: aws:directoryservice:LogService
///     name: example
///     properties:
///       directoryId: ${exampleAwsDirectoryServiceDirectory.id}
///       logGroupName: ${example.name}
/// variables:
///   ad-log-policy:
///     fn::invoke:
///       Function: aws:iam:getPolicyDocument
///       Arguments:
///         statements:
///           - actions:
///               - logs:CreateLogStream
///               - logs:PutLogEvents
///             principals:
///               - identifiers:
///                   - ds.amazonaws.com
///                 type: Service
///             resources:
///               - ${example.arn}:*
///             effect: Allow
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Directory Service Log Subscriptions using the directory id. For example:
///
/// ```sh
/// $ pulumi import aws:directoryservice/logService:LogService msad d-1234567890
/// ```
pub mod log_service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LogServiceArgs {
        /// ID of directory.
        #[builder(into)]
        pub directory_id: pulumi_wasm_rust::Output<String>,
        /// Name of the cloudwatch log group to which the logs should be published. The log group should be already created and the directory service principal should be provided with required permission to create stream and publish logs. Changing this value would delete the current subscription and create a new one. A directory can only have one log subscription at a time.
        #[builder(into)]
        pub log_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct LogServiceResult {
        /// ID of directory.
        pub directory_id: pulumi_wasm_rust::Output<String>,
        /// Name of the cloudwatch log group to which the logs should be published. The log group should be already created and the directory service principal should be provided with required permission to create stream and publish logs. Changing this value would delete the current subscription and create a new one. A directory can only have one log subscription at a time.
        pub log_group_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: LogServiceArgs) -> LogServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let directory_id_binding = args.directory_id.get_inner();
        let log_group_name_binding = args.log_group_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:directoryservice/logService:LogService".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "directoryId".into(),
                    value: &directory_id_binding,
                },
                register_interface::ObjectField {
                    name: "logGroupName".into(),
                    value: &log_group_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "directoryId".into(),
                },
                register_interface::ResultField {
                    name: "logGroupName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LogServiceResult {
            directory_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("directoryId").unwrap(),
            ),
            log_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logGroupName").unwrap(),
            ),
        }
    }
}