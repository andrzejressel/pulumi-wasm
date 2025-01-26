pub mod get_account_public_access_block {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAccountPublicAccessBlockArgs {
        /// AWS account ID to configure. Defaults to automatically determined account ID of the AWS provider.
        #[builder(into, default)]
        pub account_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetAccountPublicAccessBlockResult {
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether or not Amazon S3 should block public ACLs for buckets in this account is enabled. Returns as `true` or `false`.
        pub block_public_acls: pulumi_wasm_rust::Output<bool>,
        /// Whether or not Amazon S3 should block public bucket policies for buckets in this account is enabled. Returns as `true` or `false`.
        pub block_public_policy: pulumi_wasm_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Whether or not Amazon S3 should ignore public ACLs for buckets in this account is enabled. Returns as `true` or `false`.
        pub ignore_public_acls: pulumi_wasm_rust::Output<bool>,
        /// Whether or not Amazon S3 should restrict public bucket policies for buckets in this account is enabled. Returns as `true` or `false`.
        pub restrict_public_buckets: pulumi_wasm_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetAccountPublicAccessBlockArgs,
    ) -> GetAccountPublicAccessBlockResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:s3/getAccountPublicAccessBlock:getAccountPublicAccessBlock"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "blockPublicAcls".into(),
                },
                register_interface::ResultField {
                    name: "blockPublicPolicy".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "ignorePublicAcls".into(),
                },
                register_interface::ResultField {
                    name: "restrictPublicBuckets".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetAccountPublicAccessBlockResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            block_public_acls: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("blockPublicAcls").unwrap(),
            ),
            block_public_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("blockPublicPolicy").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ignore_public_acls: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ignorePublicAcls").unwrap(),
            ),
            restrict_public_buckets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("restrictPublicBuckets").unwrap(),
            ),
        }
    }
}
