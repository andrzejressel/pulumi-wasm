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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod service_integration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceIntegrationArgs {
        /// Information about whether DevOps Guru is configured to encrypt server-side data using KMS. See `kms_server_side_encryption` below.
        #[builder(into, default)]
        pub kms_server_side_encryption: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::devopsguru::ServiceIntegrationKmsServerSideEncryption,
            >,
        >,
        /// Information about whether DevOps Guru is configured to perform log anomaly detection on Amazon CloudWatch log groups. See `logs_anomaly_detection` below.
        #[builder(into, default)]
        pub logs_anomaly_detection: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::devopsguru::ServiceIntegrationLogsAnomalyDetection,
            >,
        >,
        /// Information about whether DevOps Guru is configured to create an OpsItem in AWS Systems Manager OpsCenter for each created insight. See `ops_center` below.
        #[builder(into, default)]
        pub ops_center: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::devopsguru::ServiceIntegrationOpsCenter>,
        >,
    }
    #[allow(dead_code)]
    pub struct ServiceIntegrationResult {
        /// Information about whether DevOps Guru is configured to encrypt server-side data using KMS. See `kms_server_side_encryption` below.
        pub kms_server_side_encryption: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::devopsguru::ServiceIntegrationKmsServerSideEncryption,
            >,
        >,
        /// Information about whether DevOps Guru is configured to perform log anomaly detection on Amazon CloudWatch log groups. See `logs_anomaly_detection` below.
        pub logs_anomaly_detection: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::devopsguru::ServiceIntegrationLogsAnomalyDetection,
            >,
        >,
        /// Information about whether DevOps Guru is configured to create an OpsItem in AWS Systems Manager OpsCenter for each created insight. See `ops_center` below.
        pub ops_center: pulumi_gestalt_rust::Output<
            Option<super::super::types::devopsguru::ServiceIntegrationOpsCenter>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServiceIntegrationArgs,
    ) -> ServiceIntegrationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let kms_server_side_encryption_binding = args
            .kms_server_side_encryption
            .get_output(context);
        let logs_anomaly_detection_binding = args
            .logs_anomaly_detection
            .get_output(context);
        let ops_center_binding = args.ops_center.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:devopsguru/serviceIntegration:ServiceIntegration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsServerSideEncryption".into(),
                    value: kms_server_side_encryption_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logsAnomalyDetection".into(),
                    value: logs_anomaly_detection_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "opsCenter".into(),
                    value: ops_center_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ServiceIntegrationResult {
            kms_server_side_encryption: o.get_field("kmsServerSideEncryption"),
            logs_anomaly_detection: o.get_field("logsAnomalyDetection"),
            ops_center: o.get_field("opsCenter"),
        }
    }
}
