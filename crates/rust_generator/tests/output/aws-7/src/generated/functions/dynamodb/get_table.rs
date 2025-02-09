#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        args: GetTableArgs,
    ) -> GetTableResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let server_side_encryption_binding = args
            .server_side_encryption
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:dynamodb/getTable:getTable".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serverSideEncryption".into(),
                    value: server_side_encryption_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetTableResult {
            arn: o.get_field("arn"),
            attributes: o.get_field("attributes"),
            billing_mode: o.get_field("billingMode"),
            deletion_protection_enabled: o.get_field("deletionProtectionEnabled"),
            global_secondary_indexes: o.get_field("globalSecondaryIndexes"),
            hash_key: o.get_field("hashKey"),
            id: o.get_field("id"),
            local_secondary_indexes: o.get_field("localSecondaryIndexes"),
            name: o.get_field("name"),
            point_in_time_recovery: o.get_field("pointInTimeRecovery"),
            range_key: o.get_field("rangeKey"),
            read_capacity: o.get_field("readCapacity"),
            replicas: o.get_field("replicas"),
            server_side_encryption: o.get_field("serverSideEncryption"),
            stream_arn: o.get_field("streamArn"),
            stream_enabled: o.get_field("streamEnabled"),
            stream_label: o.get_field("streamLabel"),
            stream_view_type: o.get_field("streamViewType"),
            table_class: o.get_field("tableClass"),
            tags: o.get_field("tags"),
            ttl: o.get_field("ttl"),
            write_capacity: o.get_field("writeCapacity"),
        }
    }
}
