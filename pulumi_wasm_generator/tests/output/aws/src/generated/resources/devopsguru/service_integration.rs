/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:devopsguru:ServiceIntegration
///     properties:
///       kmsServerSideEncryption:
///         optInStatus: ENABLED
///         type: AWS_OWNED_KMS_KEY
///       logsAnomalyDetection:
///         optInStatus: ENABLED
///       opsCenter:
///         optInStatus: ENABLED
/// ```
///
/// ### Customer Managed KMS Key
///
/// ```yaml
/// resources:
///   example:
///     type: aws:kms:Key
///   exampleServiceIntegration:
///     type: aws:devopsguru:ServiceIntegration
///     name: example
///     properties:
///       kmsServerSideEncryption:
///         kmsKeyId: ${test.arn}
///         optInStatus: ENABLED
///         type: CUSTOMER_MANAGED_KEY
///       logsAnomalyDetection:
///         optInStatus: DISABLED
///       opsCenter:
///         optInStatus: DISABLED
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DevOps Guru Service Integration using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:devopsguru/serviceIntegration:ServiceIntegration example us-east-1
/// ```
pub mod service_integration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceIntegrationArgs {
        /// Information about whether DevOps Guru is configured to encrypt server-side data using KMS. See `kms_server_side_encryption` below.
        #[builder(into, default)]
        pub kms_server_side_encryption: pulumi_wasm_rust::Output<
            Option<
                super::super::types::devopsguru::ServiceIntegrationKmsServerSideEncryption,
            >,
        >,
        /// Information about whether DevOps Guru is configured to perform log anomaly detection on Amazon CloudWatch log groups. See `logs_anomaly_detection` below.
        #[builder(into, default)]
        pub logs_anomaly_detection: pulumi_wasm_rust::Output<
            Option<
                super::super::types::devopsguru::ServiceIntegrationLogsAnomalyDetection,
            >,
        >,
        /// Information about whether DevOps Guru is configured to create an OpsItem in AWS Systems Manager OpsCenter for each created insight. See `ops_center` below.
        #[builder(into, default)]
        pub ops_center: pulumi_wasm_rust::Output<
            Option<super::super::types::devopsguru::ServiceIntegrationOpsCenter>,
        >,
    }
    #[allow(dead_code)]
    pub struct ServiceIntegrationResult {
        /// Information about whether DevOps Guru is configured to encrypt server-side data using KMS. See `kms_server_side_encryption` below.
        pub kms_server_side_encryption: pulumi_wasm_rust::Output<
            Option<
                super::super::types::devopsguru::ServiceIntegrationKmsServerSideEncryption,
            >,
        >,
        /// Information about whether DevOps Guru is configured to perform log anomaly detection on Amazon CloudWatch log groups. See `logs_anomaly_detection` below.
        pub logs_anomaly_detection: pulumi_wasm_rust::Output<
            Option<
                super::super::types::devopsguru::ServiceIntegrationLogsAnomalyDetection,
            >,
        >,
        /// Information about whether DevOps Guru is configured to create an OpsItem in AWS Systems Manager OpsCenter for each created insight. See `ops_center` below.
        pub ops_center: pulumi_wasm_rust::Output<
            Option<super::super::types::devopsguru::ServiceIntegrationOpsCenter>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ServiceIntegrationArgs) -> ServiceIntegrationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let kms_server_side_encryption_binding = args
            .kms_server_side_encryption
            .get_inner();
        let logs_anomaly_detection_binding = args.logs_anomaly_detection.get_inner();
        let ops_center_binding = args.ops_center.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:devopsguru/serviceIntegration:ServiceIntegration".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "kmsServerSideEncryption".into(),
                    value: &kms_server_side_encryption_binding,
                },
                register_interface::ObjectField {
                    name: "logsAnomalyDetection".into(),
                    value: &logs_anomaly_detection_binding,
                },
                register_interface::ObjectField {
                    name: "opsCenter".into(),
                    value: &ops_center_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "kmsServerSideEncryption".into(),
                },
                register_interface::ResultField {
                    name: "logsAnomalyDetection".into(),
                },
                register_interface::ResultField {
                    name: "opsCenter".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ServiceIntegrationResult {
            kms_server_side_encryption: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsServerSideEncryption").unwrap(),
            ),
            logs_anomaly_detection: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logsAnomalyDetection").unwrap(),
            ),
            ops_center: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("opsCenter").unwrap(),
            ),
        }
    }
}