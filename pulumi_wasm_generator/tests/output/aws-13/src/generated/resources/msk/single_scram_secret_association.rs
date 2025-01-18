/// Associates a single SCRAM secret with a Managed Streaming for Kafka (MSK) cluster.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = single_scram_secret_association::create(
///         "example",
///         SingleScramSecretAssociationArgs::builder()
///             .cluster_arn("${exampleAwsMskCluster.arn}")
///             .secret_arn("${exampleAwsSecretsmanagerSecret.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import an MSK SCRAM Secret Association using the `cluster_arn` and `secret_arn`. For example:
///
/// ```sh
/// $ pulumi import aws:msk/singleScramSecretAssociation:SingleScramSecretAssociation example arn:aws:kafka:us-west-2:123456789012:cluster/example/279c0212-d057-4dba-9aa9-1c4e5a25bfc7-3,arn:aws:secretsmanager:us-east-1:123456789012:secret:example-123456
/// ```
pub mod single_scram_secret_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SingleScramSecretAssociationArgs {
        /// Amazon Resource Name (ARN) of the MSK cluster.
        #[builder(into)]
        pub cluster_arn: pulumi_wasm_rust::Output<String>,
        /// AWS Secrets Manager secret ARN.
        #[builder(into)]
        pub secret_arn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct SingleScramSecretAssociationResult {
        /// Amazon Resource Name (ARN) of the MSK cluster.
        pub cluster_arn: pulumi_wasm_rust::Output<String>,
        /// AWS Secrets Manager secret ARN.
        pub secret_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: SingleScramSecretAssociationArgs,
    ) -> SingleScramSecretAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cluster_arn_binding = args.cluster_arn.get_inner();
        let secret_arn_binding = args.secret_arn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:msk/singleScramSecretAssociation:SingleScramSecretAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clusterArn".into(),
                    value: &cluster_arn_binding,
                },
                register_interface::ObjectField {
                    name: "secretArn".into(),
                    value: &secret_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "clusterArn".into(),
                },
                register_interface::ResultField {
                    name: "secretArn".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SingleScramSecretAssociationResult {
            cluster_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterArn").unwrap(),
            ),
            secret_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secretArn").unwrap(),
            ),
        }
    }
}
