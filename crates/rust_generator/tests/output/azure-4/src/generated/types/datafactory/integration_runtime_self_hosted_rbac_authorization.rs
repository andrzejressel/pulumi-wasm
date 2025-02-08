#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct IntegrationRuntimeSelfHostedRbacAuthorization {
    /// The resource identifier of the integration runtime to be shared.
    /// 
    /// > **Please Note**: RBAC Authorization creates a [linked Self-hosted Integration Runtime targeting the Shared Self-hosted Integration Runtime in resource_id](https://docs.microsoft.com/azure/data-factory/create-shared-self-hosted-integration-runtime-powershell#share-the-self-hosted-integration-runtime-with-another-data-factory). The linked Self-hosted Integration Runtime needs Contributor access granted to the Shared Self-hosted Data Factory.
    /// 
    /// For more information on the configuration, please check out the [Azure documentation](https://docs.microsoft.com/rest/api/datafactory/integrationruntimes/createorupdate#linkedintegrationruntimerbacauthorization)
    #[builder(into)]
    #[serde(rename = "resourceId")]
    pub r#resource_id: Box<String>,
}
