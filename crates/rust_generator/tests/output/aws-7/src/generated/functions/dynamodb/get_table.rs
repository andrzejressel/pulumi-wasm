pub mod get_table {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTableArgs {
        /// Name of the DynamoDB table.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub server_side_encryption: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::super::types::dynamodb::GetTableServerSideEncryption>,
        >,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetTableResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub attributes: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::dynamodb::GetTableAttribute>,
        >,
        pub billing_mode: pulumi_gestalt_rust::Output<String>,
        pub deletion_protection_enabled: pulumi_gestalt_rust::Output<bool>,
        pub global_secondary_indexes: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::dynamodb::GetTableGlobalSecondaryIndex>,
        >,
        pub hash_key: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub local_secondary_indexes: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::dynamodb::GetTableLocalSecondaryIndex>,
        >,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub point_in_time_recovery: pulumi_gestalt_rust::Output<
            super::super::super::types::dynamodb::GetTablePointInTimeRecovery,
        >,
        pub range_key: pulumi_gestalt_rust::Output<String>,
        pub read_capacity: pulumi_gestalt_rust::Output<i32>,
        pub replicas: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::dynamodb::GetTableReplica>,
        >,
        pub server_side_encryption: pulumi_gestalt_rust::Output<
            super::super::super::types::dynamodb::GetTableServerSideEncryption,
        >,
        pub stream_arn: pulumi_gestalt_rust::Output<String>,
        pub stream_enabled: pulumi_gestalt_rust::Output<bool>,
        pub stream_label: pulumi_gestalt_rust::Output<String>,
        pub stream_view_type: pulumi_gestalt_rust::Output<String>,
        pub table_class: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        pub ttl: pulumi_gestalt_rust::Output<
            super::super::super::types::dynamodb::GetTableTtl,
        >,
        pub write_capacity: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetTableArgs,
    ) -> GetTableResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let server_side_encryption_binding = args
            .server_side_encryption
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:dynamodb/getTable:getTable".into(),
            version: super::super::super::get_version(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetTableResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            attributes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("attributes"),
            ),
            billing_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("billingMode"),
            ),
            deletion_protection_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deletionProtectionEnabled"),
            ),
            global_secondary_indexes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("globalSecondaryIndexes"),
            ),
            hash_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hashKey"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            local_secondary_indexes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("localSecondaryIndexes"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            point_in_time_recovery: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pointInTimeRecovery"),
            ),
            range_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("rangeKey"),
            ),
            read_capacity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("readCapacity"),
            ),
            replicas: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("replicas"),
            ),
            server_side_encryption: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serverSideEncryption"),
            ),
            stream_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("streamArn"),
            ),
            stream_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("streamEnabled"),
            ),
            stream_label: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("streamLabel"),
            ),
            stream_view_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("streamViewType"),
            ),
            table_class: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tableClass"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            ttl: pulumi_gestalt_rust::__private::into_domain(o.extract_field("ttl")),
            write_capacity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("writeCapacity"),
            ),
        }
    }
}
