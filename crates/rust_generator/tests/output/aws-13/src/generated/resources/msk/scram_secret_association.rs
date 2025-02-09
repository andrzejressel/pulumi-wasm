/// Associates SCRAM secrets stored in the Secrets Manager service with a Managed Streaming for Kafka (MSK) cluster.
///
/// !> This resource takes exclusive ownership over SCRAM secrets associated with a cluster. This includes removal of SCRAM secrets which are not explicitly configured. To prevent persistent drift, ensure any `aws.msk.SingleScramSecretAssociation` resources managed alongside this resource are included in the `secret_arn_list` argument.
///
/// > **Note:** The following assumes the MSK cluster has SASL/SCRAM authentication enabled. See below for example usage or refer to the [Username/Password Authentication](https://docs.aws.amazon.com/msk/latest/developerguide/msk-password.html) section of the MSK Developer Guide for more details.
///
/// To set up username and password authentication for a cluster, create an `aws.secretsmanager.Secret` resource and associate
/// a username and password with the secret with an `aws.secretsmanager.SecretVersion` resource. When creating a secret for the cluster,
/// the `name` must have the prefix `AmazonMSK_` and you must either use an existing custom AWS KMS key or create a new
/// custom AWS KMS key for your secret with the `aws.kms.Key` resource. It is important to note that a policy is required for the `aws.secretsmanager.Secret`
/// resource in order for Kafka to be able to read it. This policy is attached automatically when the `aws.msk.ScramSecretAssociation` is used,
/// however, this policy will not be in the state and as such, will present a diff on plan/apply. For that reason, you must use the `aws.secretsmanager.SecretPolicy`
/// resource](/docs/providers/aws/r/secretsmanager_secret_policy.html) as shown below in order to ensure that the state is in a clean state after the creation of secret and the association to the cluster.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleScramSecretAssociation:
///     type: aws:msk:ScramSecretAssociation
///     name: example
///     properties:
///       clusterArn: ${exampleCluster.arn}
///       secretArnLists:
///         - ${exampleSecret.arn}
///     options:
///       dependsOn:
///         - ${exampleSecretVersion}
///   exampleCluster:
///     type: aws:msk:Cluster
///     name: example
///     properties:
///       clusterName: example
///       clientAuthentication:
///         sasl:
///           scram: true
///   exampleSecret:
///     type: aws:secretsmanager:Secret
///     name: example
///     properties:
///       name: AmazonMSK_example
///       kmsKeyId: ${exampleKey.keyId}
///   exampleKey:
///     type: aws:kms:Key
///     name: example
///     properties:
///       description: Example Key for MSK Cluster Scram Secret Association
///   exampleSecretVersion:
///     type: aws:secretsmanager:SecretVersion
///     name: example
///     properties:
///       secretId: ${exampleSecret.id}
///       secretString:
///         fn::toJSON:
///           username: user
///           password: pass
///   exampleSecretPolicy:
///     type: aws:secretsmanager:SecretPolicy
///     name: example
///     properties:
///       secretArn: ${exampleSecret.arn}
///       policy: ${example.json}
/// variables:
///   example:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - sid: AWSKafkaResourcePolicy
///             effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - kafka.amazonaws.com
///             actions:
///               - secretsmanager:getSecretValue
///             resources:
///               - ${exampleSecret.arn}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import MSK SCRAM Secret Associations using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:msk/scramSecretAssociation:ScramSecretAssociation example arn:aws:kafka:us-west-2:123456789012:cluster/example/279c0212-d057-4dba-9aa9-1c4e5a25bfc7-3
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod scram_secret_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ScramSecretAssociationArgs {
        /// Amazon Resource Name (ARN) of the MSK cluster.
        #[builder(into)]
        pub cluster_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// List of AWS Secrets Manager secret ARNs.
        #[builder(into)]
        pub secret_arn_lists: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
    }
    #[allow(dead_code)]
    pub struct ScramSecretAssociationResult {
        /// Amazon Resource Name (ARN) of the MSK cluster.
        pub cluster_arn: pulumi_gestalt_rust::Output<String>,
        /// List of AWS Secrets Manager secret ARNs.
        pub secret_arn_lists: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ScramSecretAssociationArgs,
    ) -> ScramSecretAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cluster_arn_binding = args.cluster_arn.get_output(context);
        let secret_arn_lists_binding = args.secret_arn_lists.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:msk/scramSecretAssociation:ScramSecretAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterArn".into(),
                    value: cluster_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secretArnLists".into(),
                    value: secret_arn_lists_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ScramSecretAssociationResult {
            cluster_arn: o.get_field("clusterArn"),
            secret_arn_lists: o.get_field("secretArnLists"),
        }
    }
}
