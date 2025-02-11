#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_elasticsearch {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetElasticsearchArgs {
        /// A `logs` block as defined below.
        #[builder(into, default)]
        pub logs: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::elasticcloud::GetElasticsearchLog>>,
        >,
        /// The name of the Elasticsearch resource.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the resource group in which the Elasticsearch exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetElasticsearchResult {
        /// The ID of the Deployment within Elastic Cloud.
        pub elastic_cloud_deployment_id: pulumi_gestalt_rust::Output<String>,
        /// The Email Address which is associated with this Elasticsearch account.
        pub elastic_cloud_email_address: pulumi_gestalt_rust::Output<String>,
        /// The Default URL used for Single Sign On (SSO) to Elastic Cloud.
        pub elastic_cloud_sso_default_url: pulumi_gestalt_rust::Output<String>,
        /// The ID of the User Account within Elastic Cloud.
        pub elastic_cloud_user_id: pulumi_gestalt_rust::Output<String>,
        /// The URL to the Elasticsearch Service associated with this Elasticsearch.
        pub elasticsearch_service_url: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The URL to the Kibana Dashboard associated with this Elasticsearch.
        pub kibana_service_url: pulumi_gestalt_rust::Output<String>,
        /// The URI used for SSO to the Kibana Dashboard associated with this Elasticsearch.
        pub kibana_sso_uri: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region in which this Elasticsearch exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// A `logs` block as defined below.
        pub logs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::elasticcloud::GetElasticsearchLog>,
        >,
        /// Specifies if monitoring is enabled on this Elasticsearch or not.
        pub monitoring_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The name (key) of the Tag which should be filtered.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the SKU used for this Elasticsearch.
        pub sku_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags assigned to the Elasticsearch.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetElasticsearchArgs,
    ) -> GetElasticsearchResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let logs_binding = args.logs.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:elasticcloud/getElasticsearch:getElasticsearch".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logs".into(),
                    value: &logs_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetElasticsearchResult {
            elastic_cloud_deployment_id: o.get_field("elasticCloudDeploymentId"),
            elastic_cloud_email_address: o.get_field("elasticCloudEmailAddress"),
            elastic_cloud_sso_default_url: o.get_field("elasticCloudSsoDefaultUrl"),
            elastic_cloud_user_id: o.get_field("elasticCloudUserId"),
            elasticsearch_service_url: o.get_field("elasticsearchServiceUrl"),
            id: o.get_field("id"),
            kibana_service_url: o.get_field("kibanaServiceUrl"),
            kibana_sso_uri: o.get_field("kibanaSsoUri"),
            location: o.get_field("location"),
            logs: o.get_field("logs"),
            monitoring_enabled: o.get_field("monitoringEnabled"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            sku_name: o.get_field("skuName"),
            tags: o.get_field("tags"),
        }
    }
}
