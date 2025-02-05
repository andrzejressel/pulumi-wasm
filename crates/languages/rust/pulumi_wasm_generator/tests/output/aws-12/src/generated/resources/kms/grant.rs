/// Provides a resource-based access control mechanism for a KMS customer master key.
///
/// > **Note:** All arguments including the grant token will be stored in the raw state as plain-text.
/// ## Import
///
/// Using `pulumi import`, import KMS Grants using the Key ID and Grant ID separated by a colon (`:`). For example:
///
/// ```sh
/// $ pulumi import aws:kms/grant:Grant test 1234abcd-12ab-34cd-56ef-1234567890ab:abcde1237f76e4ba7987489ac329fbfba6ad343d6f7075dbd1ef191f0120514
/// ```
pub mod grant {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GrantArgs {
        /// A structure that you can use to allow certain operations in the grant only when the desired encryption context is present. For more information about encryption context, see [Encryption Context](http://docs.aws.amazon.com/kms/latest/developerguide/encryption-context.html).
        #[builder(into, default)]
        pub constraints: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::kms::GrantConstraint>>,
        >,
        /// A list of grant tokens to be used when creating the grant. See [Grant Tokens](http://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token) for more information about grant tokens.
        #[builder(into, default)]
        pub grant_creation_tokens: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// The principal that is given permission to perform the operations that the grant permits in ARN format. Note that due to eventual consistency issues around IAM principals, the providers's state may not always be refreshed to reflect what is true in AWS.
        #[builder(into)]
        pub grantee_principal: pulumi_wasm_rust::InputOrOutput<String>,
        /// The unique identifier for the customer master key (CMK) that the grant applies to. Specify the key ID or the Amazon Resource Name (ARN) of the CMK. To specify a CMK in a different AWS account, you must use the key ARN.
        #[builder(into)]
        pub key_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// A friendly name for identifying the grant.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A list of operations that the grant permits. The permitted values are: `Decrypt`, `Encrypt`, `GenerateDataKey`, `GenerateDataKeyWithoutPlaintext`, `ReEncryptFrom`, `ReEncryptTo`, `Sign`, `Verify`, `GetPublicKey`, `CreateGrant`, `RetireGrant`, `DescribeKey`, `GenerateDataKeyPair`, or `GenerateDataKeyPairWithoutPlaintext`.
        #[builder(into)]
        pub operations: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// If set to false (the default) the grants will be revoked upon deletion, and if set to true the grants will try to be retired upon deletion. Note that retiring grants requires special permissions, hence why we default to revoking grants.
        /// See [RetireGrant](https://docs.aws.amazon.com/kms/latest/APIReference/API_RetireGrant.html) for more information.
        #[builder(into, default)]
        pub retire_on_delete: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The principal that is given permission to retire the grant by using RetireGrant operation in ARN format. Note that due to eventual consistency issues around IAM principals, the providers's state may not always be refreshed to reflect what is true in AWS.
        #[builder(into, default)]
        pub retiring_principal: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GrantResult {
        /// A structure that you can use to allow certain operations in the grant only when the desired encryption context is present. For more information about encryption context, see [Encryption Context](http://docs.aws.amazon.com/kms/latest/developerguide/encryption-context.html).
        pub constraints: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::kms::GrantConstraint>>,
        >,
        /// A list of grant tokens to be used when creating the grant. See [Grant Tokens](http://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token) for more information about grant tokens.
        pub grant_creation_tokens: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The unique identifier for the grant.
        pub grant_id: pulumi_wasm_rust::Output<String>,
        /// The grant token for the created grant. For more information, see [Grant Tokens](http://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token).
        pub grant_token: pulumi_wasm_rust::Output<String>,
        /// The principal that is given permission to perform the operations that the grant permits in ARN format. Note that due to eventual consistency issues around IAM principals, the providers's state may not always be refreshed to reflect what is true in AWS.
        pub grantee_principal: pulumi_wasm_rust::Output<String>,
        /// The unique identifier for the customer master key (CMK) that the grant applies to. Specify the key ID or the Amazon Resource Name (ARN) of the CMK. To specify a CMK in a different AWS account, you must use the key ARN.
        pub key_id: pulumi_wasm_rust::Output<String>,
        /// A friendly name for identifying the grant.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A list of operations that the grant permits. The permitted values are: `Decrypt`, `Encrypt`, `GenerateDataKey`, `GenerateDataKeyWithoutPlaintext`, `ReEncryptFrom`, `ReEncryptTo`, `Sign`, `Verify`, `GetPublicKey`, `CreateGrant`, `RetireGrant`, `DescribeKey`, `GenerateDataKeyPair`, or `GenerateDataKeyPairWithoutPlaintext`.
        pub operations: pulumi_wasm_rust::Output<Vec<String>>,
        /// If set to false (the default) the grants will be revoked upon deletion, and if set to true the grants will try to be retired upon deletion. Note that retiring grants requires special permissions, hence why we default to revoking grants.
        /// See [RetireGrant](https://docs.aws.amazon.com/kms/latest/APIReference/API_RetireGrant.html) for more information.
        pub retire_on_delete: pulumi_wasm_rust::Output<Option<bool>>,
        /// The principal that is given permission to retire the grant by using RetireGrant operation in ARN format. Note that due to eventual consistency issues around IAM principals, the providers's state may not always be refreshed to reflect what is true in AWS.
        pub retiring_principal: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: GrantArgs,
    ) -> GrantResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let constraints_binding = args.constraints.get_output(context).get_inner();
        let grant_creation_tokens_binding = args
            .grant_creation_tokens
            .get_output(context)
            .get_inner();
        let grantee_principal_binding = args
            .grantee_principal
            .get_output(context)
            .get_inner();
        let key_id_binding = args.key_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let operations_binding = args.operations.get_output(context).get_inner();
        let retire_on_delete_binding = args
            .retire_on_delete
            .get_output(context)
            .get_inner();
        let retiring_principal_binding = args
            .retiring_principal
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:kms/grant:Grant".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "constraints".into(),
                    value: &constraints_binding,
                },
                register_interface::ObjectField {
                    name: "grantCreationTokens".into(),
                    value: &grant_creation_tokens_binding,
                },
                register_interface::ObjectField {
                    name: "granteePrincipal".into(),
                    value: &grantee_principal_binding,
                },
                register_interface::ObjectField {
                    name: "keyId".into(),
                    value: &key_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "operations".into(),
                    value: &operations_binding,
                },
                register_interface::ObjectField {
                    name: "retireOnDelete".into(),
                    value: &retire_on_delete_binding,
                },
                register_interface::ObjectField {
                    name: "retiringPrincipal".into(),
                    value: &retiring_principal_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        GrantResult {
            constraints: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("constraints"),
            ),
            grant_creation_tokens: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("grantCreationTokens"),
            ),
            grant_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("grantId"),
            ),
            grant_token: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("grantToken"),
            ),
            grantee_principal: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("granteePrincipal"),
            ),
            key_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("keyId")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            operations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("operations"),
            ),
            retire_on_delete: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("retireOnDelete"),
            ),
            retiring_principal: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("retiringPrincipal"),
            ),
        }
    }
}
