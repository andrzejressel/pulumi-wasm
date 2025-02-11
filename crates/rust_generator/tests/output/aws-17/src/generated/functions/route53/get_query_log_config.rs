#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_query_log_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetQueryLogConfigArgs {
        /// One or more name/value pairs to use as filters. There are
        /// several valid keys, for a full reference, check out
        /// [Route53resolver Filter value in the AWS API reference][1].
        ///
        /// In addition to all arguments above, the following attributes are exported:
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::route53::GetQueryLogConfigFilter>>,
        >,
        /// The name of the query logging configuration.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID of the Route53 Resolver Query Logging Configuration.
        #[builder(into, default)]
        pub resolver_query_log_config_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Map of tags to assign to the service.
        ///
        /// [1]: https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_Filter.html
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetQueryLogConfigResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub destination_arn: pulumi_gestalt_rust::Output<String>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::route53::GetQueryLogConfigFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<Option<String>>,
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        pub resolver_query_log_config_id: pulumi_gestalt_rust::Output<Option<String>>,
        pub share_status: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetQueryLogConfigArgs,
    ) -> GetQueryLogConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filters_binding = args.filters.get_output(context);
        let name_binding = args.name.get_output(context);
        let resolver_query_log_config_id_binding = args
            .resolver_query_log_config_id
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:route53/getQueryLogConfig:getQueryLogConfig".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: &filters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resolverQueryLogConfigId".into(),
                    value: &resolver_query_log_config_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetQueryLogConfigResult {
            arn: o.get_field("arn"),
            destination_arn: o.get_field("destinationArn"),
            filters: o.get_field("filters"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            owner_id: o.get_field("ownerId"),
            resolver_query_log_config_id: o.get_field("resolverQueryLogConfigId"),
            share_status: o.get_field("shareStatus"),
            tags: o.get_field("tags"),
        }
    }
}
