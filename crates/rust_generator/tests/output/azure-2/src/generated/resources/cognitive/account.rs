/// Manages a Cognitive Services Account.
///
/// > **Note:** Version v2.65.0 of the Azure Provider and later will attempt to Purge the Cognitive Account during deletion. This feature can be disabled using the `features` block within the `provider` block, see the provider documentation on the features block for more information.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleAccount:
///     type: azure:cognitive:Account
///     name: example
///     properties:
///       name: example-account
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       kind: Face
///       skuName: S0
///       tags:
///         Acceptance: Test
/// ```
///
/// ## Import
///
/// Cognitive Service Accounts can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cognitive/account:Account account1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.CognitiveServices/accounts/account1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod account {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountArgs {
        /// If `kind` is `TextAnalytics` this specifies the ID of the Search service.
        #[builder(into, default)]
        pub custom_question_answering_search_service_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// If `kind` is `TextAnalytics` this specifies the key of the Search service.
        ///
        /// > **NOTE:** `custom_question_answering_search_service_id` and `custom_question_answering_search_service_key` are used for [Custom Question Answering, the renamed version of QnA Maker](https://docs.microsoft.com/azure/cognitive-services/qnamaker/custom-question-answering), while `qna_runtime_endpoint` is used for [the old version of QnA Maker](https://docs.microsoft.com/azure/cognitive-services/qnamaker/overview/overview)
        #[builder(into, default)]
        pub custom_question_answering_search_service_key: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The subdomain name used for token-based authentication. This property is required when `network_acls` is specified. This property is also required when using the OpenAI service with libraries which assume the Azure OpenAI endpoint is a subdomain on `https://openai.azure.com/`, eg. `https://<custom_subdomain_name>.openai.azure.com/`.  Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub custom_subdomain_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `customer_managed_key` block as documented below.
        #[builder(into, default)]
        pub customer_managed_key: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cognitive::AccountCustomerManagedKey>,
        >,
        /// Whether to enable the dynamic throttling for this Cognitive Service Account.
        #[builder(into, default)]
        pub dynamic_throttling_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// List of FQDNs allowed for the Cognitive Account.
        #[builder(into, default)]
        pub fqdns: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cognitive::AccountIdentity>,
        >,
        /// Specifies the type of Cognitive Service Account that should be created. Possible values are `Academic`, `AnomalyDetector`, `Bing.Autosuggest`, `Bing.Autosuggest.v7`, `Bing.CustomSearch`, `Bing.Search`, `Bing.Search.v7`, `Bing.Speech`, `Bing.SpellCheck`, `Bing.SpellCheck.v7`, `CognitiveServices`, `ComputerVision`, `ContentModerator`, `ContentSafety`, `CustomSpeech`, `CustomVision.Prediction`, `CustomVision.Training`, `Emotion`, `Face`, `FormRecognizer`, `ImmersiveReader`, `LUIS`, `LUIS.Authoring`, `MetricsAdvisor`, `OpenAI`, `Personalizer`, `QnAMaker`, `Recommendations`, `SpeakerRecognition`, `Speech`, `SpeechServices`, `SpeechTranslation`, `TextAnalytics`, `TextTranslation` and `WebLM`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** New Bing Search resources cannot be created as their APIs are moving from Cognitive Services Platform to new surface area under Microsoft.com. Starting from October 30, 2020, existing instances of Bing Search APIs provisioned via Cognitive Services will be continuously supported for next 3 years or till the end of respective Enterprise Agreement, whichever happens first.
        ///
        /// > **NOTE:** You must create your first Face, Text Analytics, or Computer Vision resources from the Azure portal to review and acknowledge the terms and conditions. In Azure Portal, the checkbox to accept terms and conditions is only displayed when a US region is selected. More information on [Prerequisites](https://docs.microsoft.com/azure/cognitive-services/cognitive-services-apis-create-account-cli?tabs=windows#prerequisites).
        #[builder(into)]
        pub kind: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether local authentication methods is enabled for the Cognitive Account. Defaults to `true`.
        #[builder(into, default)]
        pub local_auth_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Azure AD Client ID (Application ID). This attribute is only set when kind is `MetricsAdvisor`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub metrics_advisor_aad_client_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The Azure AD Tenant ID. This attribute is only set when kind is `MetricsAdvisor`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub metrics_advisor_aad_tenant_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The super user of Metrics Advisor. This attribute is only set when kind is `MetricsAdvisor`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub metrics_advisor_super_user_name: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The website name of Metrics Advisor. This attribute is only set when kind is `MetricsAdvisor`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** This URL is mandatory if the `kind` is set to `QnAMaker`.
        #[builder(into, default)]
        pub metrics_advisor_website_name: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the name of the Cognitive Service Account. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `network_acls` block as defined below. When this property is specified, `custom_subdomain_name` is also required to be set.
        #[builder(into, default)]
        pub network_acls: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cognitive::AccountNetworkAcls>,
        >,
        /// Whether outbound network access is restricted for the Cognitive Account. Defaults to `false`.
        #[builder(into, default)]
        pub outbound_network_access_restricted: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Whether public network access is allowed for the Cognitive Account. Defaults to `true`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// A URL to link a QnAMaker cognitive account to a QnA runtime.
        #[builder(into, default)]
        pub qna_runtime_endpoint: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which the Cognitive Service Account is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the SKU Name for this Cognitive Service Account. Possible values are `F0`, `F1`, `S0`, `S`, `S1`, `S2`, `S3`, `S4`, `S5`, `S6`, `P0`, `P1`, `P2`, `E0` and `DC0`.
        ///
        /// > **NOTE:** SKU `DC0` is the commitment tier for Cognitive Services containers running in disconnected environments. You must obtain approval from Microsoft by submitting the [request form](https://aka.ms/csdisconnectedcontainers) first, before you can use this SKU. More information on [Purchase a commitment plan to use containers in disconnected environments](https://learn.microsoft.com/en-us/azure/cognitive-services/containers/disconnected-containers?tabs=stt#purchase-a-commitment-plan-to-use-containers-in-disconnected-environments).
        #[builder(into)]
        pub sku_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `storage` block as defined below.
        #[builder(into, default)]
        pub storages: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::cognitive::AccountStorage>>,
        >,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AccountResult {
        /// If `kind` is `TextAnalytics` this specifies the ID of the Search service.
        pub custom_question_answering_search_service_id: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// If `kind` is `TextAnalytics` this specifies the key of the Search service.
        ///
        /// > **NOTE:** `custom_question_answering_search_service_id` and `custom_question_answering_search_service_key` are used for [Custom Question Answering, the renamed version of QnA Maker](https://docs.microsoft.com/azure/cognitive-services/qnamaker/custom-question-answering), while `qna_runtime_endpoint` is used for [the old version of QnA Maker](https://docs.microsoft.com/azure/cognitive-services/qnamaker/overview/overview)
        pub custom_question_answering_search_service_key: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// The subdomain name used for token-based authentication. This property is required when `network_acls` is specified. This property is also required when using the OpenAI service with libraries which assume the Azure OpenAI endpoint is a subdomain on `https://openai.azure.com/`, eg. `https://<custom_subdomain_name>.openai.azure.com/`.  Changing this forces a new resource to be created.
        pub custom_subdomain_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `customer_managed_key` block as documented below.
        pub customer_managed_key: pulumi_gestalt_rust::Output<
            Option<super::super::types::cognitive::AccountCustomerManagedKey>,
        >,
        /// Whether to enable the dynamic throttling for this Cognitive Service Account.
        pub dynamic_throttling_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The endpoint used to connect to the Cognitive Service Account.
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// List of FQDNs allowed for the Cognitive Account.
        pub fqdns: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::cognitive::AccountIdentity>,
        >,
        /// Specifies the type of Cognitive Service Account that should be created. Possible values are `Academic`, `AnomalyDetector`, `Bing.Autosuggest`, `Bing.Autosuggest.v7`, `Bing.CustomSearch`, `Bing.Search`, `Bing.Search.v7`, `Bing.Speech`, `Bing.SpellCheck`, `Bing.SpellCheck.v7`, `CognitiveServices`, `ComputerVision`, `ContentModerator`, `ContentSafety`, `CustomSpeech`, `CustomVision.Prediction`, `CustomVision.Training`, `Emotion`, `Face`, `FormRecognizer`, `ImmersiveReader`, `LUIS`, `LUIS.Authoring`, `MetricsAdvisor`, `OpenAI`, `Personalizer`, `QnAMaker`, `Recommendations`, `SpeakerRecognition`, `Speech`, `SpeechServices`, `SpeechTranslation`, `TextAnalytics`, `TextTranslation` and `WebLM`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** New Bing Search resources cannot be created as their APIs are moving from Cognitive Services Platform to new surface area under Microsoft.com. Starting from October 30, 2020, existing instances of Bing Search APIs provisioned via Cognitive Services will be continuously supported for next 3 years or till the end of respective Enterprise Agreement, whichever happens first.
        ///
        /// > **NOTE:** You must create your first Face, Text Analytics, or Computer Vision resources from the Azure portal to review and acknowledge the terms and conditions. In Azure Portal, the checkbox to accept terms and conditions is only displayed when a US region is selected. More information on [Prerequisites](https://docs.microsoft.com/azure/cognitive-services/cognitive-services-apis-create-account-cli?tabs=windows#prerequisites).
        pub kind: pulumi_gestalt_rust::Output<String>,
        /// Whether local authentication methods is enabled for the Cognitive Account. Defaults to `true`.
        pub local_auth_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The Azure AD Client ID (Application ID). This attribute is only set when kind is `MetricsAdvisor`. Changing this forces a new resource to be created.
        pub metrics_advisor_aad_client_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Azure AD Tenant ID. This attribute is only set when kind is `MetricsAdvisor`. Changing this forces a new resource to be created.
        pub metrics_advisor_aad_tenant_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The super user of Metrics Advisor. This attribute is only set when kind is `MetricsAdvisor`. Changing this forces a new resource to be created.
        pub metrics_advisor_super_user_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The website name of Metrics Advisor. This attribute is only set when kind is `MetricsAdvisor`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** This URL is mandatory if the `kind` is set to `QnAMaker`.
        pub metrics_advisor_website_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name of the Cognitive Service Account. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `network_acls` block as defined below. When this property is specified, `custom_subdomain_name` is also required to be set.
        pub network_acls: pulumi_gestalt_rust::Output<
            Option<super::super::types::cognitive::AccountNetworkAcls>,
        >,
        /// Whether outbound network access is restricted for the Cognitive Account. Defaults to `false`.
        pub outbound_network_access_restricted: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// A primary access key which can be used to connect to the Cognitive Service Account.
        pub primary_access_key: pulumi_gestalt_rust::Output<String>,
        /// Whether public network access is allowed for the Cognitive Account. Defaults to `true`.
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A URL to link a QnAMaker cognitive account to a QnA runtime.
        pub qna_runtime_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the resource group in which the Cognitive Service Account is created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The secondary access key which can be used to connect to the Cognitive Service Account.
        pub secondary_access_key: pulumi_gestalt_rust::Output<String>,
        /// Specifies the SKU Name for this Cognitive Service Account. Possible values are `F0`, `F1`, `S0`, `S`, `S1`, `S2`, `S3`, `S4`, `S5`, `S6`, `P0`, `P1`, `P2`, `E0` and `DC0`.
        ///
        /// > **NOTE:** SKU `DC0` is the commitment tier for Cognitive Services containers running in disconnected environments. You must obtain approval from Microsoft by submitting the [request form](https://aka.ms/csdisconnectedcontainers) first, before you can use this SKU. More information on [Purchase a commitment plan to use containers in disconnected environments](https://learn.microsoft.com/en-us/azure/cognitive-services/containers/disconnected-containers?tabs=stt#purchase-a-commitment-plan-to-use-containers-in-disconnected-environments).
        pub sku_name: pulumi_gestalt_rust::Output<String>,
        /// A `storage` block as defined below.
        pub storages: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::cognitive::AccountStorage>>,
        >,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AccountArgs,
    ) -> AccountResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let custom_question_answering_search_service_id_binding = args
            .custom_question_answering_search_service_id
            .get_output(context);
        let custom_question_answering_search_service_key_binding = args
            .custom_question_answering_search_service_key
            .get_output(context);
        let custom_subdomain_name_binding = args
            .custom_subdomain_name
            .get_output(context);
        let customer_managed_key_binding = args.customer_managed_key.get_output(context);
        let dynamic_throttling_enabled_binding = args
            .dynamic_throttling_enabled
            .get_output(context);
        let fqdns_binding = args.fqdns.get_output(context);
        let identity_binding = args.identity.get_output(context);
        let kind_binding = args.kind.get_output(context);
        let local_auth_enabled_binding = args.local_auth_enabled.get_output(context);
        let location_binding = args.location.get_output(context);
        let metrics_advisor_aad_client_id_binding = args
            .metrics_advisor_aad_client_id
            .get_output(context);
        let metrics_advisor_aad_tenant_id_binding = args
            .metrics_advisor_aad_tenant_id
            .get_output(context);
        let metrics_advisor_super_user_name_binding = args
            .metrics_advisor_super_user_name
            .get_output(context);
        let metrics_advisor_website_name_binding = args
            .metrics_advisor_website_name
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let network_acls_binding = args.network_acls.get_output(context);
        let outbound_network_access_restricted_binding = args
            .outbound_network_access_restricted
            .get_output(context);
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_output(context);
        let qna_runtime_endpoint_binding = args.qna_runtime_endpoint.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let sku_name_binding = args.sku_name.get_output(context);
        let storages_binding = args.storages.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:cognitive/account:Account".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customQuestionAnsweringSearchServiceId".into(),
                    value: custom_question_answering_search_service_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customQuestionAnsweringSearchServiceKey".into(),
                    value: custom_question_answering_search_service_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customSubdomainName".into(),
                    value: custom_subdomain_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customerManagedKey".into(),
                    value: customer_managed_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dynamicThrottlingEnabled".into(),
                    value: dynamic_throttling_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fqdns".into(),
                    value: fqdns_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: identity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kind".into(),
                    value: kind_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "localAuthEnabled".into(),
                    value: local_auth_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metricsAdvisorAadClientId".into(),
                    value: metrics_advisor_aad_client_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metricsAdvisorAadTenantId".into(),
                    value: metrics_advisor_aad_tenant_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metricsAdvisorSuperUserName".into(),
                    value: metrics_advisor_super_user_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metricsAdvisorWebsiteName".into(),
                    value: metrics_advisor_website_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkAcls".into(),
                    value: network_acls_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "outboundNetworkAccessRestricted".into(),
                    value: outbound_network_access_restricted_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: public_network_access_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "qnaRuntimeEndpoint".into(),
                    value: qna_runtime_endpoint_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skuName".into(),
                    value: sku_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storages".into(),
                    value: storages_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AccountResult {
            custom_question_answering_search_service_id: o
                .get_field("customQuestionAnsweringSearchServiceId"),
            custom_question_answering_search_service_key: o
                .get_field("customQuestionAnsweringSearchServiceKey"),
            custom_subdomain_name: o.get_field("customSubdomainName"),
            customer_managed_key: o.get_field("customerManagedKey"),
            dynamic_throttling_enabled: o.get_field("dynamicThrottlingEnabled"),
            endpoint: o.get_field("endpoint"),
            fqdns: o.get_field("fqdns"),
            identity: o.get_field("identity"),
            kind: o.get_field("kind"),
            local_auth_enabled: o.get_field("localAuthEnabled"),
            location: o.get_field("location"),
            metrics_advisor_aad_client_id: o.get_field("metricsAdvisorAadClientId"),
            metrics_advisor_aad_tenant_id: o.get_field("metricsAdvisorAadTenantId"),
            metrics_advisor_super_user_name: o.get_field("metricsAdvisorSuperUserName"),
            metrics_advisor_website_name: o.get_field("metricsAdvisorWebsiteName"),
            name: o.get_field("name"),
            network_acls: o.get_field("networkAcls"),
            outbound_network_access_restricted: o
                .get_field("outboundNetworkAccessRestricted"),
            primary_access_key: o.get_field("primaryAccessKey"),
            public_network_access_enabled: o.get_field("publicNetworkAccessEnabled"),
            qna_runtime_endpoint: o.get_field("qnaRuntimeEndpoint"),
            resource_group_name: o.get_field("resourceGroupName"),
            secondary_access_key: o.get_field("secondaryAccessKey"),
            sku_name: o.get_field("skuName"),
            storages: o.get_field("storages"),
            tags: o.get_field("tags"),
        }
    }
}
