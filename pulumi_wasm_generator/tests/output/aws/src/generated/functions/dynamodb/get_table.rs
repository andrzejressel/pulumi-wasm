pub mod get_table {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTableArgs {
        /// Name of the DynamoDB table.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub server_side_encryption: pulumi_wasm_rust::Output<
            Option<super::super::super::types::dynamodb::GetTableServerSideEncryption>,
        >,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetTableResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        pub attributes: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::dynamodb::GetTableAttribute>,
        >,
        pub billing_mode: pulumi_wasm_rust::Output<String>,
        pub deletion_protection_enabled: pulumi_wasm_rust::Output<bool>,
        pub global_secondary_indexes: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::dynamodb::GetTableGlobalSecondaryIndex>,
        >,
        pub hash_key: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub local_secondary_indexes: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::dynamodb::GetTableLocalSecondaryIndex>,
        >,
        pub name: pulumi_wasm_rust::Output<String>,
        pub point_in_time_recovery: pulumi_wasm_rust::Output<
            super::super::super::types::dynamodb::GetTablePointInTimeRecovery,
        >,
        pub range_key: pulumi_wasm_rust::Output<String>,
        pub read_capacity: pulumi_wasm_rust::Output<i32>,
        pub replicas: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::dynamodb::GetTableReplica>,
        >,
        pub server_side_encryption: pulumi_wasm_rust::Output<
            super::super::super::types::dynamodb::GetTableServerSideEncryption,
        >,
        pub stream_arn: pulumi_wasm_rust::Output<String>,
        pub stream_enabled: pulumi_wasm_rust::Output<bool>,
        pub stream_label: pulumi_wasm_rust::Output<String>,
        pub stream_view_type: pulumi_wasm_rust::Output<String>,
        pub table_class: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub ttl: pulumi_wasm_rust::Output<
            super::super::super::types::dynamodb::GetTableTtl,
        >,
        pub write_capacity: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetTableArgs) -> GetTableResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let server_side_encryption_binding = args.server_side_encryption.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:dynamodb/getTable:getTable".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "serverSideEncryption".into(),
                    value: &server_side_encryption_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "attributes".into(),
                },
                register_interface::ResultField {
                    name: "billingMode".into(),
                },
                register_interface::ResultField {
                    name: "deletionProtectionEnabled".into(),
                },
                register_interface::ResultField {
                    name: "globalSecondaryIndexes".into(),
                },
                register_interface::ResultField {
                    name: "hashKey".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "localSecondaryIndexes".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "pointInTimeRecovery".into(),
                },
                register_interface::ResultField {
                    name: "rangeKey".into(),
                },
                register_interface::ResultField {
                    name: "readCapacity".into(),
                },
                register_interface::ResultField {
                    name: "replicas".into(),
                },
                register_interface::ResultField {
                    name: "serverSideEncryption".into(),
                },
                register_interface::ResultField {
                    name: "streamArn".into(),
                },
                register_interface::ResultField {
                    name: "streamEnabled".into(),
                },
                register_interface::ResultField {
                    name: "streamLabel".into(),
                },
                register_interface::ResultField {
                    name: "streamViewType".into(),
                },
                register_interface::ResultField {
                    name: "tableClass".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "ttl".into(),
                },
                register_interface::ResultField {
                    name: "writeCapacity".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetTableResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            attributes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("attributes").unwrap(),
            ),
            billing_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("billingMode").unwrap(),
            ),
            deletion_protection_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deletionProtectionEnabled").unwrap(),
            ),
            global_secondary_indexes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("globalSecondaryIndexes").unwrap(),
            ),
            hash_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hashKey").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            local_secondary_indexes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("localSecondaryIndexes").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            point_in_time_recovery: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pointInTimeRecovery").unwrap(),
            ),
            range_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rangeKey").unwrap(),
            ),
            read_capacity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("readCapacity").unwrap(),
            ),
            replicas: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicas").unwrap(),
            ),
            server_side_encryption: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverSideEncryption").unwrap(),
            ),
            stream_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("streamArn").unwrap(),
            ),
            stream_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("streamEnabled").unwrap(),
            ),
            stream_label: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("streamLabel").unwrap(),
            ),
            stream_view_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("streamViewType").unwrap(),
            ),
            table_class: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tableClass").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            ttl: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ttl").unwrap(),
            ),
            write_capacity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("writeCapacity").unwrap(),
            ),
        }
    }
}
