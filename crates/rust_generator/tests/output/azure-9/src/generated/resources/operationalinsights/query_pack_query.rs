/// Manages a Log Analytics Query Pack Query.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleQueryPack = query_pack::create(
///         "exampleQueryPack",
///         QueryPackArgs::builder()
///             .location("${example.location}")
///             .name("example-laqp")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleQueryPackQuery = query_pack_query::create(
///         "exampleQueryPackQuery",
///         QueryPackQueryArgs::builder()
///             .body(
///                 "let newExceptionsTimeRange = 1d;\nlet timeRangeToCheckBefore = 7d;\nexceptions\n| where timestamp < ago(timeRangeToCheckBefore)\n| summarize count() by problemId\n| join kind= rightanti (\nexceptions\n| where timestamp >= ago(newExceptionsTimeRange)\n| extend stack = tostring(details[0].rawStack)\n| summarize count(), dcount(user_AuthenticatedId), min(timestamp), max(timestamp), any(stack) by problemId  \n) on problemId \n| order by  count_ desc\n",
///             )
///             .display_name("Exceptions - New in the last 24 hours")
///             .name("19952bc3-0bf9-49eb-b713-6b80e7a41847")
///             .query_pack_id("${exampleQueryPack.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Log Analytics Query Pack Queries can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:operationalinsights/queryPackQuery:QueryPackQuery example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/group1/providers/Microsoft.OperationalInsights/queryPacks/queryPack1/queries/15b49e87-8555-4d92-8a7b-2014b469a9df
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod query_pack_query {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct QueryPackQueryArgs {
        /// The additional properties that can be set for the Log Analytics Query Pack Query.
        #[builder(into, default)]
        pub additional_settings_json: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The body of the Log Analytics Query Pack Query.
        #[builder(into)]
        pub body: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A list of the related categories for the function. Possible values are `applications`, `audit`, `container`, `databases`, `desktopanalytics`, `management`, `monitor`, `network`, `resources`, `security`, `virtualmachines`, `windowsvirtualdesktop` and `workloads`.
        #[builder(into, default)]
        pub categories: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The description of the Log Analytics Query Pack Query.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The unique display name for the query within the Log Analytics Query Pack.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An unique UUID/GUID which identifies this Log Analytics Query Pack Query - one will be generated if not specified. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Log Analytics Query Pack. Changing this forces a new resource to be created.
        #[builder(into)]
        pub query_pack_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A list of the related resource types for the function. Possible values are `default`, `microsoft.aad/domainservices`, `microsoft.aadiam/tenants`, `microsoft.agfoodplatform/farmbeats`, `microsoft.analysisservices/servers`, `microsoft.apimanagement/service`, `microsoft.appconfiguration/configurationstores`, `microsoft.appplatform/spring`, `microsoft.attestation/attestationproviders`, `microsoft.authorization/tenants`, `microsoft.automation/automationaccounts`, `microsoft.autonomousdevelopmentplatform/accounts`, `microsoft.azurestackhci/virtualmachines`, `microsoft.batch/batchaccounts`, `microsoft.blockchain/blockchainmembers`, `microsoft.botservice/botservices`, `microsoft.cache/redis`, `microsoft.cdn/profiles`, `microsoft.cognitiveservices/accounts`, `microsoft.communication/communicationservices`, `microsoft.compute/virtualmachines`, `microsoft.compute/virtualmachinescalesets`, `microsoft.connectedcache/cachenodes`, `microsoft.connectedvehicle/platformaccounts`, `microsoft.conenctedvmwarevsphere/virtualmachines`, `microsoft.containerregistry/registries`, `microsoft.containerservice/managedclusters`, `microsoft.d365customerinsights/instances`, `microsoft.dashboard/grafana`, `microsoft.databricks/workspaces`, `microsoft.datacollaboration/workspaces`, `microsoft.datafactory/factories`, `microsoft.datalakeanalytics/accounts`, `microsoft.datalakestore/accounts`, `microsoft.datashare/accounts`, `microsoft.dbformariadb/servers`, `microsoft.dbformysql/servers`, `microsoft.dbforpostgresql/flexibleservers`, `microsoft.dbforpostgresql/servers`, `microsoft.dbforpostgresql/serversv2`, `microsoft.digitaltwins/digitaltwinsinstances`, `microsoft.documentdb/cassandraclusters`, `microsoft.documentdb/databaseaccounts`, `microsoft.desktopvirtualization/applicationgroups`, `microsoft.desktopvirtualization/hostpools`, `microsoft.desktopvirtualization/workspaces`, `microsoft.devices/iothubs`, `microsoft.devices/provisioningservices`, `microsoft.dynamics/fraudprotection/purchase`, `microsoft.eventgrid/domains`, `microsoft.eventgrid/topics`, `microsoft.eventgrid/partnernamespaces`, `microsoft.eventgrid/partnertopics`, `microsoft.eventgrid/systemtopics`, `microsoft.eventhub/namespaces`, `microsoft.experimentation/experimentworkspaces`, `microsoft.hdinsight/clusters`, `microsoft.healthcareapis/services`, `microsoft.informationprotection/datasecuritymanagement`, `microsoft.intune/operations`, `microsoft.insights/autoscalesettings`, `microsoft.insights/components`, `microsoft.insights/workloadmonitoring`, `microsoft.keyvault/vaults`, `microsoft.kubernetes/connectedclusters`, `microsoft.kusto/clusters`, `microsoft.loadtestservice/loadtests`, `microsoft.logic/workflows`, `microsoft.machinelearningservices/workspaces`, `microsoft.media/mediaservices`, `microsoft.netapp/netappaccounts/capacitypools`, `microsoft.network/applicationgateways`, `microsoft.network/azurefirewalls`, `microsoft.network/bastionhosts`, `microsoft.network/expressroutecircuits`, `microsoft.network/frontdoors`, `microsoft.network/loadbalancers`, `microsoft.network/networkinterfaces`, `microsoft.network/networksecuritygroups`, `microsoft.network/networksecurityperimeters`, `microsoft.network/networkwatchers/connectionmonitors`, `microsoft.network/networkwatchers/trafficanalytics`, `microsoft.network/publicipaddresses`, `microsoft.network/trafficmanagerprofiles`, `microsoft.network/virtualnetworks`, `microsoft.network/virtualnetworkgateways`, `microsoft.network/vpngateways`, `microsoft.networkfunction/azuretrafficcollectors`, `microsoft.openenergyplatform/energyservices`, `microsoft.openlogisticsplatform/workspaces`, `microsoft.operationalinsights/workspaces`, `microsoft.powerbi/tenants`, `microsoft.powerbi/tenants/workspaces`, `microsoft.powerbidedicated/capacities`, `microsoft.purview/accounts`, `microsoft.recoveryservices/vaults`, `microsoft.resources/azureactivity`, `microsoft.scvmm/virtualmachines`, `microsoft.search/searchservices`, `microsoft.security/antimalwaresettings`, `microsoft.securityinsights/amazon`, `microsoft.securityinsights/anomalies`, `microsoft.securityinsights/cef`, `microsoft.securityinsights/datacollection`, `microsoft.securityinsights/dnsnormalized`, `microsoft.securityinsights/mda`, `microsoft.securityinsights/mde`, `microsoft.securityinsights/mdi`, `microsoft.securityinsights/mdo`, `microsoft.securityinsights/networksessionnormalized`, `microsoft.securityinsights/office365`, `microsoft.securityinsights/purview`, `microsoft.securityinsights/securityinsights`, `microsoft.securityinsights/securityinsights/mcas`, `microsoft.securityinsights/tvm`, `microsoft.securityinsights/watchlists`, `microsoft.servicebus/namespaces`, `microsoft.servicefabric/clusters`, `microsoft.signalrservice/signalr`, `microsoft.signalrservice/webpubsub`, `microsoft.sql/managedinstances`, `microsoft.sql/servers`, `microsoft.sql/servers/databases`, `microsoft.storage/storageaccounts`, `microsoft.storagecache/caches`, `microsoft.streamanalytics/streamingjobs`, `microsoft.synapse/workspaces`, `microsoft.timeseriesinsights/environments`, `microsoft.videoindexer/accounts`, `microsoft.web/sites`, `microsoft.workloadmonitor/monitors`, `resourcegroup` and `subscription`.
        #[builder(into, default)]
        pub resource_types: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A list of the related Log Analytics solutions for the function. Possible values are `AADDomainServices`, `ADAssessment`, `ADAssessmentPlus`, `ADReplication`, `ADSecurityAssessment`, `AlertManagement`, `AntiMalware`, `ApplicationInsights`, `AzureAssessment`, `AzureSecurityOfThings`, `AzureSentinelDSRE`, `AzureSentinelPrivatePreview`, `BehaviorAnalyticsInsights`, `ChangeTracking`, `CompatibilityAssessment`, `ContainerInsights`, `Containers`, `CustomizedWindowsEventsFiltering`, `DeviceHealthProd`, `DnsAnalytics`, `ExchangeAssessment`, `ExchangeOnlineAssessment`, `IISAssessmentPlus`, `InfrastructureInsights`, `InternalWindowsEvent`, `LogManagement`, `Microsoft365Analytics`, `NetworkMonitoring`, `SCCMAssessmentPlus`, `SCOMAssessment`, `SCOMAssessmentPlus`, `Security`, `SecurityCenter`, `SecurityCenterFree`, `SecurityInsights`, `ServiceMap`, `SfBAssessment`, `SfBOnlineAssessment`, `SharePointOnlineAssessment`, `SPAssessment`, `SQLAdvancedThreatProtection`, `SQLAssessment`, `SQLAssessmentPlus`, `SQLDataClassification`, `SQLThreatDetection`, `SQLVulnerabilityAssessment`, `SurfaceHub`, `Updates`, `VMInsights`, `WEFInternalUat`, `WEF_10x`, `WEF_10xDSRE`, `WaaSUpdateInsights`, `WinLog`, `WindowsClientAssessmentPlus`, `WindowsEventForwarding`, `WindowsFirewall`, `WindowsServerAssessment`, `WireData` and `WireData2`.
        #[builder(into, default)]
        pub solutions: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A mapping of tags which should be assigned to the Log Analytics Query Pack Query.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct QueryPackQueryResult {
        /// The additional properties that can be set for the Log Analytics Query Pack Query.
        pub additional_settings_json: pulumi_gestalt_rust::Output<Option<String>>,
        /// The body of the Log Analytics Query Pack Query.
        pub body: pulumi_gestalt_rust::Output<String>,
        /// A list of the related categories for the function. Possible values are `applications`, `audit`, `container`, `databases`, `desktopanalytics`, `management`, `monitor`, `network`, `resources`, `security`, `virtualmachines`, `windowsvirtualdesktop` and `workloads`.
        pub categories: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The description of the Log Analytics Query Pack Query.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The unique display name for the query within the Log Analytics Query Pack.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// An unique UUID/GUID which identifies this Log Analytics Query Pack Query - one will be generated if not specified. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Log Analytics Query Pack. Changing this forces a new resource to be created.
        pub query_pack_id: pulumi_gestalt_rust::Output<String>,
        /// A list of the related resource types for the function. Possible values are `default`, `microsoft.aad/domainservices`, `microsoft.aadiam/tenants`, `microsoft.agfoodplatform/farmbeats`, `microsoft.analysisservices/servers`, `microsoft.apimanagement/service`, `microsoft.appconfiguration/configurationstores`, `microsoft.appplatform/spring`, `microsoft.attestation/attestationproviders`, `microsoft.authorization/tenants`, `microsoft.automation/automationaccounts`, `microsoft.autonomousdevelopmentplatform/accounts`, `microsoft.azurestackhci/virtualmachines`, `microsoft.batch/batchaccounts`, `microsoft.blockchain/blockchainmembers`, `microsoft.botservice/botservices`, `microsoft.cache/redis`, `microsoft.cdn/profiles`, `microsoft.cognitiveservices/accounts`, `microsoft.communication/communicationservices`, `microsoft.compute/virtualmachines`, `microsoft.compute/virtualmachinescalesets`, `microsoft.connectedcache/cachenodes`, `microsoft.connectedvehicle/platformaccounts`, `microsoft.conenctedvmwarevsphere/virtualmachines`, `microsoft.containerregistry/registries`, `microsoft.containerservice/managedclusters`, `microsoft.d365customerinsights/instances`, `microsoft.dashboard/grafana`, `microsoft.databricks/workspaces`, `microsoft.datacollaboration/workspaces`, `microsoft.datafactory/factories`, `microsoft.datalakeanalytics/accounts`, `microsoft.datalakestore/accounts`, `microsoft.datashare/accounts`, `microsoft.dbformariadb/servers`, `microsoft.dbformysql/servers`, `microsoft.dbforpostgresql/flexibleservers`, `microsoft.dbforpostgresql/servers`, `microsoft.dbforpostgresql/serversv2`, `microsoft.digitaltwins/digitaltwinsinstances`, `microsoft.documentdb/cassandraclusters`, `microsoft.documentdb/databaseaccounts`, `microsoft.desktopvirtualization/applicationgroups`, `microsoft.desktopvirtualization/hostpools`, `microsoft.desktopvirtualization/workspaces`, `microsoft.devices/iothubs`, `microsoft.devices/provisioningservices`, `microsoft.dynamics/fraudprotection/purchase`, `microsoft.eventgrid/domains`, `microsoft.eventgrid/topics`, `microsoft.eventgrid/partnernamespaces`, `microsoft.eventgrid/partnertopics`, `microsoft.eventgrid/systemtopics`, `microsoft.eventhub/namespaces`, `microsoft.experimentation/experimentworkspaces`, `microsoft.hdinsight/clusters`, `microsoft.healthcareapis/services`, `microsoft.informationprotection/datasecuritymanagement`, `microsoft.intune/operations`, `microsoft.insights/autoscalesettings`, `microsoft.insights/components`, `microsoft.insights/workloadmonitoring`, `microsoft.keyvault/vaults`, `microsoft.kubernetes/connectedclusters`, `microsoft.kusto/clusters`, `microsoft.loadtestservice/loadtests`, `microsoft.logic/workflows`, `microsoft.machinelearningservices/workspaces`, `microsoft.media/mediaservices`, `microsoft.netapp/netappaccounts/capacitypools`, `microsoft.network/applicationgateways`, `microsoft.network/azurefirewalls`, `microsoft.network/bastionhosts`, `microsoft.network/expressroutecircuits`, `microsoft.network/frontdoors`, `microsoft.network/loadbalancers`, `microsoft.network/networkinterfaces`, `microsoft.network/networksecuritygroups`, `microsoft.network/networksecurityperimeters`, `microsoft.network/networkwatchers/connectionmonitors`, `microsoft.network/networkwatchers/trafficanalytics`, `microsoft.network/publicipaddresses`, `microsoft.network/trafficmanagerprofiles`, `microsoft.network/virtualnetworks`, `microsoft.network/virtualnetworkgateways`, `microsoft.network/vpngateways`, `microsoft.networkfunction/azuretrafficcollectors`, `microsoft.openenergyplatform/energyservices`, `microsoft.openlogisticsplatform/workspaces`, `microsoft.operationalinsights/workspaces`, `microsoft.powerbi/tenants`, `microsoft.powerbi/tenants/workspaces`, `microsoft.powerbidedicated/capacities`, `microsoft.purview/accounts`, `microsoft.recoveryservices/vaults`, `microsoft.resources/azureactivity`, `microsoft.scvmm/virtualmachines`, `microsoft.search/searchservices`, `microsoft.security/antimalwaresettings`, `microsoft.securityinsights/amazon`, `microsoft.securityinsights/anomalies`, `microsoft.securityinsights/cef`, `microsoft.securityinsights/datacollection`, `microsoft.securityinsights/dnsnormalized`, `microsoft.securityinsights/mda`, `microsoft.securityinsights/mde`, `microsoft.securityinsights/mdi`, `microsoft.securityinsights/mdo`, `microsoft.securityinsights/networksessionnormalized`, `microsoft.securityinsights/office365`, `microsoft.securityinsights/purview`, `microsoft.securityinsights/securityinsights`, `microsoft.securityinsights/securityinsights/mcas`, `microsoft.securityinsights/tvm`, `microsoft.securityinsights/watchlists`, `microsoft.servicebus/namespaces`, `microsoft.servicefabric/clusters`, `microsoft.signalrservice/signalr`, `microsoft.signalrservice/webpubsub`, `microsoft.sql/managedinstances`, `microsoft.sql/servers`, `microsoft.sql/servers/databases`, `microsoft.storage/storageaccounts`, `microsoft.storagecache/caches`, `microsoft.streamanalytics/streamingjobs`, `microsoft.synapse/workspaces`, `microsoft.timeseriesinsights/environments`, `microsoft.videoindexer/accounts`, `microsoft.web/sites`, `microsoft.workloadmonitor/monitors`, `resourcegroup` and `subscription`.
        pub resource_types: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// A list of the related Log Analytics solutions for the function. Possible values are `AADDomainServices`, `ADAssessment`, `ADAssessmentPlus`, `ADReplication`, `ADSecurityAssessment`, `AlertManagement`, `AntiMalware`, `ApplicationInsights`, `AzureAssessment`, `AzureSecurityOfThings`, `AzureSentinelDSRE`, `AzureSentinelPrivatePreview`, `BehaviorAnalyticsInsights`, `ChangeTracking`, `CompatibilityAssessment`, `ContainerInsights`, `Containers`, `CustomizedWindowsEventsFiltering`, `DeviceHealthProd`, `DnsAnalytics`, `ExchangeAssessment`, `ExchangeOnlineAssessment`, `IISAssessmentPlus`, `InfrastructureInsights`, `InternalWindowsEvent`, `LogManagement`, `Microsoft365Analytics`, `NetworkMonitoring`, `SCCMAssessmentPlus`, `SCOMAssessment`, `SCOMAssessmentPlus`, `Security`, `SecurityCenter`, `SecurityCenterFree`, `SecurityInsights`, `ServiceMap`, `SfBAssessment`, `SfBOnlineAssessment`, `SharePointOnlineAssessment`, `SPAssessment`, `SQLAdvancedThreatProtection`, `SQLAssessment`, `SQLAssessmentPlus`, `SQLDataClassification`, `SQLThreatDetection`, `SQLVulnerabilityAssessment`, `SurfaceHub`, `Updates`, `VMInsights`, `WEFInternalUat`, `WEF_10x`, `WEF_10xDSRE`, `WaaSUpdateInsights`, `WinLog`, `WindowsClientAssessmentPlus`, `WindowsEventForwarding`, `WindowsFirewall`, `WindowsServerAssessment`, `WireData` and `WireData2`.
        pub solutions: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// A mapping of tags which should be assigned to the Log Analytics Query Pack Query.
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
        args: QueryPackQueryArgs,
    ) -> QueryPackQueryResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let additional_settings_json_binding = args
            .additional_settings_json
            .get_output(context);
        let body_binding = args.body.get_output(context);
        let categories_binding = args.categories.get_output(context);
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let query_pack_id_binding = args.query_pack_id.get_output(context);
        let resource_types_binding = args.resource_types.get_output(context);
        let solutions_binding = args.solutions.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:operationalinsights/queryPackQuery:QueryPackQuery".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "additionalSettingsJson".into(),
                    value: &additional_settings_json_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "body".into(),
                    value: &body_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "categories".into(),
                    value: &categories_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "queryPackId".into(),
                    value: &query_pack_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceTypes".into(),
                    value: &resource_types_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "solutions".into(),
                    value: &solutions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        QueryPackQueryResult {
            additional_settings_json: o.get_field("additionalSettingsJson"),
            body: o.get_field("body"),
            categories: o.get_field("categories"),
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            name: o.get_field("name"),
            query_pack_id: o.get_field("queryPackId"),
            resource_types: o.get_field("resourceTypes"),
            solutions: o.get_field("solutions"),
            tags: o.get_field("tags"),
        }
    }
}
