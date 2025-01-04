pub mod get_elasticsearch {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetElasticsearchArgs {
        /// A `logs` block as defined below.
        #[builder(into, default)]
        pub logs: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::elasticcloud::GetElasticsearchLog>>,
        >,
        /// The name of the Elasticsearch resource.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which the Elasticsearch exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
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
    pub fn invoke(args: GetElasticsearchArgs) -> GetElasticsearchResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let logs_binding = args.logs.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:elasticcloud/getElasticsearch:getElasticsearch".into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "elasticCloudDeploymentId".into(),
                },
                register_interface::ResultField {
                    name: "elasticCloudEmailAddress".into(),
                },
                register_interface::ResultField {
                    name: "elasticCloudSsoDefaultUrl".into(),
                },
                register_interface::ResultField {
                    name: "elasticCloudUserId".into(),
                },
                register_interface::ResultField {
                    name: "elasticsearchServiceUrl".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "kibanaServiceUrl".into(),
                },
                register_interface::ResultField {
                    name: "kibanaSsoUri".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "logs".into(),
                },
                register_interface::ResultField {
                    name: "monitoringEnabled".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "skuName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetElasticsearchResult {
            elastic_cloud_deployment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("elasticCloudDeploymentId").unwrap(),
            ),
            elastic_cloud_email_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("elasticCloudEmailAddress").unwrap(),
            ),
            elastic_cloud_sso_default_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("elasticCloudSsoDefaultUrl").unwrap(),
            ),
            elastic_cloud_user_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("elasticCloudUserId").unwrap(),
            ),
            elasticsearch_service_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("elasticsearchServiceUrl").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            kibana_service_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kibanaServiceUrl").unwrap(),
            ),
            kibana_sso_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kibanaSsoUri").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            logs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logs").unwrap(),
            ),
            monitoring_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("monitoringEnabled").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            sku_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skuName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
