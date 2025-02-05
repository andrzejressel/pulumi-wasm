pub mod get_elasticsearch {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetElasticsearchArgs {
        /// A `logs` block as defined below.
        #[builder(into, default)]
        pub logs: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::super::types::elasticcloud::GetElasticsearchLog>>,
        >,
        /// The name of the Elasticsearch resource.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the resource group in which the Elasticsearch exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetElasticsearchResult {
        /// The ID of the Deployment within Elastic Cloud.
        pub elastic_cloud_deployment_id: pulumi_wasm_rust::Output<String>,
        /// The Email Address which is associated with this Elasticsearch account.
        pub elastic_cloud_email_address: pulumi_wasm_rust::Output<String>,
        /// The Default URL used for Single Sign On (SSO) to Elastic Cloud.
        pub elastic_cloud_sso_default_url: pulumi_wasm_rust::Output<String>,
        /// The ID of the User Account within Elastic Cloud.
        pub elastic_cloud_user_id: pulumi_wasm_rust::Output<String>,
        /// The URL to the Elasticsearch Service associated with this Elasticsearch.
        pub elasticsearch_service_url: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The URL to the Kibana Dashboard associated with this Elasticsearch.
        pub kibana_service_url: pulumi_wasm_rust::Output<String>,
        /// The URI used for SSO to the Kibana Dashboard associated with this Elasticsearch.
        pub kibana_sso_uri: pulumi_wasm_rust::Output<String>,
        /// The Azure Region in which this Elasticsearch exists.
        pub location: pulumi_wasm_rust::Output<String>,
        /// A `logs` block as defined below.
        pub logs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::elasticcloud::GetElasticsearchLog>,
        >,
        /// Specifies if monitoring is enabled on this Elasticsearch or not.
        pub monitoring_enabled: pulumi_wasm_rust::Output<bool>,
        /// The name (key) of the Tag which should be filtered.
        pub name: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The name of the SKU used for this Elasticsearch.
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the Elasticsearch.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetElasticsearchArgs,
    ) -> GetElasticsearchResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let logs_binding = args.logs.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:elasticcloud/getElasticsearch:getElasticsearch".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "logs".into(),
                    value: &logs_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetElasticsearchResult {
            elastic_cloud_deployment_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("elasticCloudDeploymentId"),
            ),
            elastic_cloud_email_address: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("elasticCloudEmailAddress"),
            ),
            elastic_cloud_sso_default_url: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("elasticCloudSsoDefaultUrl"),
            ),
            elastic_cloud_user_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("elasticCloudUserId"),
            ),
            elasticsearch_service_url: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("elasticsearchServiceUrl"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            kibana_service_url: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("kibanaServiceUrl"),
            ),
            kibana_sso_uri: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("kibanaSsoUri"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            logs: pulumi_wasm_rust::__private::into_domain(o.extract_field("logs")),
            monitoring_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("monitoringEnabled"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            sku_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("skuName"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
