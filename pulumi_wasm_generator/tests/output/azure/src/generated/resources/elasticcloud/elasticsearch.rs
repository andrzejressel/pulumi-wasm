/// Manages an Elasticsearch in Elastic Cloud.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = resource_group::create(
///         "test",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let testElasticsearch = elasticsearch::create(
///         "testElasticsearch",
///         ElasticsearchArgs::builder()
///             .elastic_cloud_email_address("user@example.com")
///             .location("${test.location}")
///             .name("example-elasticsearch")
///             .resource_group_name("${test.name}")
///             .sku_name("ess-consumption-2024_Monthly")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Elasticsearch's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:elasticcloud/elasticsearch:Elasticsearch example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Elastic/monitors/monitor1
/// ```
///
pub mod elasticsearch {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ElasticsearchArgs {
        /// Specifies the Email Address which should be associated with this Elasticsearch account. Changing this forces a new Elasticsearch to be created.
        #[builder(into)]
        pub elastic_cloud_email_address: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the Elasticsearch resource should exist. Changing this forces a new Elasticsearch to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// A `logs` block as defined below.
        #[builder(into, default)]
        pub logs: pulumi_wasm_rust::Output<
            Option<super::super::types::elasticcloud::ElasticsearchLogs>,
        >,
        /// Specifies if the Elasticsearch should have monitoring configured? Defaults to `true`. Changing this forces a new Elasticsearch to be created.
        #[builder(into, default)]
        pub monitoring_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name which should be used for this Elasticsearch resource. Changing this forces a new Elasticsearch to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group where the Elasticsearch resource should exist. Changing this forces a new Elasticsearch to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the SKU for this Elasticsearch. Changing this forces a new Elasticsearch to be created.
        ///
        /// > **NOTE:** The SKU depends on the Elasticsearch Plans available for your account and is a combination of PlanID_Term.
        /// Ex: If the plan ID is "planXYZ" and term is "Yearly", the SKU will be "planXYZ_Yearly".
        /// You may find your eligible plans [here](https://portal.azure.com/#view/Microsoft_Azure_Marketplace/GalleryItemDetailsBladeNopdl/id/elastic.ec-azure-pp) or in the online documentation [here](https://azuremarketplace.microsoft.com/en-us/marketplace/apps/elastic.ec-azure-pp?tab=PlansAndPrice) for more details or in case of any issues with the SKU.
        #[builder(into)]
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Elasticsearch resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ElasticsearchResult {
        /// The ID of the Deployment within Elastic Cloud.
        pub elastic_cloud_deployment_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the Email Address which should be associated with this Elasticsearch account. Changing this forces a new Elasticsearch to be created.
        pub elastic_cloud_email_address: pulumi_wasm_rust::Output<String>,
        /// The Default URL used for Single Sign On (SSO) to Elastic Cloud.
        pub elastic_cloud_sso_default_url: pulumi_wasm_rust::Output<String>,
        /// The ID of the User Account within Elastic Cloud.
        pub elastic_cloud_user_id: pulumi_wasm_rust::Output<String>,
        /// The URL to the Elasticsearch Service associated with this Elasticsearch.
        pub elasticsearch_service_url: pulumi_wasm_rust::Output<String>,
        /// The URL to the Kibana Dashboard associated with this Elasticsearch.
        pub kibana_service_url: pulumi_wasm_rust::Output<String>,
        /// The URI used for SSO to the Kibana Dashboard associated with this Elasticsearch.
        pub kibana_sso_uri: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the Elasticsearch resource should exist. Changing this forces a new Elasticsearch to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// A `logs` block as defined below.
        pub logs: pulumi_wasm_rust::Output<
            Option<super::super::types::elasticcloud::ElasticsearchLogs>,
        >,
        /// Specifies if the Elasticsearch should have monitoring configured? Defaults to `true`. Changing this forces a new Elasticsearch to be created.
        pub monitoring_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name which should be used for this Elasticsearch resource. Changing this forces a new Elasticsearch to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Elasticsearch resource should exist. Changing this forces a new Elasticsearch to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the SKU for this Elasticsearch. Changing this forces a new Elasticsearch to be created.
        ///
        /// > **NOTE:** The SKU depends on the Elasticsearch Plans available for your account and is a combination of PlanID_Term.
        /// Ex: If the plan ID is "planXYZ" and term is "Yearly", the SKU will be "planXYZ_Yearly".
        /// You may find your eligible plans [here](https://portal.azure.com/#view/Microsoft_Azure_Marketplace/GalleryItemDetailsBladeNopdl/id/elastic.ec-azure-pp) or in the online documentation [here](https://azuremarketplace.microsoft.com/en-us/marketplace/apps/elastic.ec-azure-pp?tab=PlansAndPrice) for more details or in case of any issues with the SKU.
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Elasticsearch resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ElasticsearchArgs) -> ElasticsearchResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let elastic_cloud_email_address_binding = args
            .elastic_cloud_email_address
            .get_inner();
        let location_binding = args.location.get_inner();
        let logs_binding = args.logs.get_inner();
        let monitoring_enabled_binding = args.monitoring_enabled.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let sku_name_binding = args.sku_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:elasticcloud/elasticsearch:Elasticsearch".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "elasticCloudEmailAddress".into(),
                    value: &elastic_cloud_email_address_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "logs".into(),
                    value: &logs_binding,
                },
                register_interface::ObjectField {
                    name: "monitoringEnabled".into(),
                    value: &monitoring_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "skuName".into(),
                    value: &sku_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
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
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ElasticsearchResult {
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