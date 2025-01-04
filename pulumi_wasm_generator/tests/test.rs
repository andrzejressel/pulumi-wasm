use anyhow::{Context, Result};
use assert_cmd::assert::OutputAssertExt;
use pulumi_wasm_generator::generate_combined;
use rinja::Template;
use std::fs;
use std::fs::{File, FileTimes};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;
// DO NOT EDIT - START

#[test]
#[cfg_attr(not(feature = "generator_array-of-enum-map"), ignore)]
fn array_of_enum_map() -> Result<()> {
    run_pulumi_generator_test("array-of-enum-map", None)
}

#[test]
#[cfg_attr(not(feature = "generator_azure-native-nested-types"), ignore)]
fn azure_native_nested_types() -> Result<()> {
    run_pulumi_generator_test("azure-native-nested-types", None)
}

#[test]
#[cfg_attr(not(feature = "generator_cloudflare"), ignore)]
fn cloudflare() -> Result<()> {
    run_pulumi_generator_test("cloudflare", None)
}

#[test]
#[cfg_attr(not(feature = "generator_cyclic-types"), ignore)]
fn cyclic_types() -> Result<()> {
    run_pulumi_generator_test("cyclic-types", None)
}

#[test]
#[cfg_attr(not(feature = "generator_different-enum"), ignore)]
fn different_enum() -> Result<()> {
    run_pulumi_generator_test("different-enum", None)
}

#[test]
#[cfg_attr(not(feature = "generator_docker"), ignore)]
fn docker() -> Result<()> {
    run_pulumi_generator_test("docker", None)
}

#[test]
#[cfg_attr(not(feature = "generator_functions-secrets"), ignore)]
fn functions_secrets() -> Result<()> {
    run_pulumi_generator_test("functions-secrets", None)
}

#[test]
#[cfg_attr(not(feature = "generator_mini-awsnative"), ignore)]
fn mini_awsnative() -> Result<()> {
    run_pulumi_generator_test("mini-awsnative", None)
}

#[test]
#[cfg_attr(not(feature = "generator_nested-module"), ignore)]
fn nested_module() -> Result<()> {
    run_pulumi_generator_test("nested-module", None)
}

#[test]
#[cfg_attr(not(feature = "generator_nested-module-thirdparty"), ignore)]
fn nested_module_thirdparty() -> Result<()> {
    run_pulumi_generator_test("nested-module-thirdparty", None)
}

#[test]
#[cfg_attr(not(feature = "generator_output-funcs"), ignore)]
fn output_funcs() -> Result<()> {
    run_pulumi_generator_test("output-funcs", None)
}

#[test]
#[cfg_attr(not(feature = "generator_output-funcs-edgeorder"), ignore)]
fn output_funcs_edgeorder() -> Result<()> {
    run_pulumi_generator_test("output-funcs-edgeorder", None)
}

#[test]
#[cfg_attr(not(feature = "generator_plain-object-defaults"), ignore)]
fn plain_object_defaults() -> Result<()> {
    run_pulumi_generator_test("plain-object-defaults", None)
}

#[test]
#[cfg_attr(not(feature = "generator_plain-object-disable-defaults"), ignore)]
fn plain_object_disable_defaults() -> Result<()> {
    run_pulumi_generator_test("plain-object-disable-defaults", None)
}

#[test]
#[cfg_attr(not(feature = "generator_random"), ignore)]
fn random() -> Result<()> {
    run_pulumi_generator_test("random", None)
}

#[test]
#[cfg_attr(not(feature = "generator_reserved_names"), ignore)]
fn reserved_names() -> Result<()> {
    run_pulumi_generator_test("reserved_names", None)
}

#[test]
#[cfg_attr(not(feature = "generator_unions-inline"), ignore)]
fn unions_inline() -> Result<()> {
    run_pulumi_generator_test("unions-inline", None)
}

#[test]
#[cfg_attr(not(feature = "generator_unions-inside-arrays"), ignore)]
fn unions_inside_arrays() -> Result<()> {
    run_pulumi_generator_test("unions-inside-arrays", None)
}

#[test]
#[cfg_attr(not(feature = "generator_workarounds"), ignore)]
fn workarounds() -> Result<()> {
    run_pulumi_generator_test("workarounds", None)
}

#[test]
#[cfg_attr(not(feature = "generator_azure-aadb2c"), ignore)]
fn azure_aadb2c() -> Result<()> {
    run_pulumi_generator_test("azure", Some("aadb2c"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-advisor"), ignore)]
fn azure_advisor() -> Result<()> {
    run_pulumi_generator_test("azure", Some("advisor"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-analysisservices"), ignore)]
fn azure_analysisservices() -> Result<()> {
    run_pulumi_generator_test("azure", Some("analysisservices"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-apimanagement"), ignore)]
fn azure_apimanagement() -> Result<()> {
    run_pulumi_generator_test("azure", Some("apimanagement"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-appconfiguration"), ignore)]
fn azure_appconfiguration() -> Result<()> {
    run_pulumi_generator_test("azure", Some("appconfiguration"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-appinsights"), ignore)]
fn azure_appinsights() -> Result<()> {
    run_pulumi_generator_test("azure", Some("appinsights"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-appplatform"), ignore)]
fn azure_appplatform() -> Result<()> {
    run_pulumi_generator_test("azure", Some("appplatform"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-appservice"), ignore)]
fn azure_appservice() -> Result<()> {
    run_pulumi_generator_test("azure", Some("appservice"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-arc"), ignore)]
fn azure_arc() -> Result<()> {
    run_pulumi_generator_test("azure", Some("arc"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-arckubernetes"), ignore)]
fn azure_arckubernetes() -> Result<()> {
    run_pulumi_generator_test("azure", Some("arckubernetes"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-arcmachine"), ignore)]
fn azure_arcmachine() -> Result<()> {
    run_pulumi_generator_test("azure", Some("arcmachine"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-armmsi"), ignore)]
fn azure_armmsi() -> Result<()> {
    run_pulumi_generator_test("azure", Some("armmsi"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-attestation"), ignore)]
fn azure_attestation() -> Result<()> {
    run_pulumi_generator_test("azure", Some("attestation"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-authorization"), ignore)]
fn azure_authorization() -> Result<()> {
    run_pulumi_generator_test("azure", Some("authorization"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-automanage"), ignore)]
fn azure_automanage() -> Result<()> {
    run_pulumi_generator_test("azure", Some("automanage"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-automation"), ignore)]
fn azure_automation() -> Result<()> {
    run_pulumi_generator_test("azure", Some("automation"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-avs"), ignore)]
fn azure_avs() -> Result<()> {
    run_pulumi_generator_test("azure", Some("avs"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-backup"), ignore)]
fn azure_backup() -> Result<()> {
    run_pulumi_generator_test("azure", Some("backup"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-batch"), ignore)]
fn azure_batch() -> Result<()> {
    run_pulumi_generator_test("azure", Some("batch"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-billing"), ignore)]
fn azure_billing() -> Result<()> {
    run_pulumi_generator_test("azure", Some("billing"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-blueprint"), ignore)]
fn azure_blueprint() -> Result<()> {
    run_pulumi_generator_test("azure", Some("blueprint"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-bot"), ignore)]
fn azure_bot() -> Result<()> {
    run_pulumi_generator_test("azure", Some("bot"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-cdn"), ignore)]
fn azure_cdn() -> Result<()> {
    run_pulumi_generator_test("azure", Some("cdn"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-chaosstudio"), ignore)]
fn azure_chaosstudio() -> Result<()> {
    run_pulumi_generator_test("azure", Some("chaosstudio"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-cognitive"), ignore)]
fn azure_cognitive() -> Result<()> {
    run_pulumi_generator_test("azure", Some("cognitive"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-communication"), ignore)]
fn azure_communication() -> Result<()> {
    run_pulumi_generator_test("azure", Some("communication"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-compute"), ignore)]
fn azure_compute() -> Result<()> {
    run_pulumi_generator_test("azure", Some("compute"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-confidentialledger"), ignore)]
fn azure_confidentialledger() -> Result<()> {
    run_pulumi_generator_test("azure", Some("confidentialledger"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-connections"), ignore)]
fn azure_connections() -> Result<()> {
    run_pulumi_generator_test("azure", Some("connections"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-consumption"), ignore)]
fn azure_consumption() -> Result<()> {
    run_pulumi_generator_test("azure", Some("consumption"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-containerapp"), ignore)]
fn azure_containerapp() -> Result<()> {
    run_pulumi_generator_test("azure", Some("containerapp"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-containerservice"), ignore)]
fn azure_containerservice() -> Result<()> {
    run_pulumi_generator_test("azure", Some("containerservice"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-core"), ignore)]
fn azure_core() -> Result<()> {
    run_pulumi_generator_test("azure", Some("core"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-cosmosdb"), ignore)]
fn azure_cosmosdb() -> Result<()> {
    run_pulumi_generator_test("azure", Some("cosmosdb"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-costmanagement"), ignore)]
fn azure_costmanagement() -> Result<()> {
    run_pulumi_generator_test("azure", Some("costmanagement"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-customip"), ignore)]
fn azure_customip() -> Result<()> {
    run_pulumi_generator_test("azure", Some("customip"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-dashboard"), ignore)]
fn azure_dashboard() -> Result<()> {
    run_pulumi_generator_test("azure", Some("dashboard"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-databasemigration"), ignore)]
fn azure_databasemigration() -> Result<()> {
    run_pulumi_generator_test("azure", Some("databasemigration"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-databoxedge"), ignore)]
fn azure_databoxedge() -> Result<()> {
    run_pulumi_generator_test("azure", Some("databoxedge"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-databricks"), ignore)]
fn azure_databricks() -> Result<()> {
    run_pulumi_generator_test("azure", Some("databricks"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-datadog"), ignore)]
fn azure_datadog() -> Result<()> {
    run_pulumi_generator_test("azure", Some("datadog"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-datafactory"), ignore)]
fn azure_datafactory() -> Result<()> {
    run_pulumi_generator_test("azure", Some("datafactory"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-dataprotection"), ignore)]
fn azure_dataprotection() -> Result<()> {
    run_pulumi_generator_test("azure", Some("dataprotection"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-datashare"), ignore)]
fn azure_datashare() -> Result<()> {
    run_pulumi_generator_test("azure", Some("datashare"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-desktopvirtualization"), ignore)]
fn azure_desktopvirtualization() -> Result<()> {
    run_pulumi_generator_test("azure", Some("desktopvirtualization"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-devcenter"), ignore)]
fn azure_devcenter() -> Result<()> {
    run_pulumi_generator_test("azure", Some("devcenter"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-devtest"), ignore)]
fn azure_devtest() -> Result<()> {
    run_pulumi_generator_test("azure", Some("devtest"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-digitaltwins"), ignore)]
fn azure_digitaltwins() -> Result<()> {
    run_pulumi_generator_test("azure", Some("digitaltwins"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-dns"), ignore)]
fn azure_dns() -> Result<()> {
    run_pulumi_generator_test("azure", Some("dns"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-domainservices"), ignore)]
fn azure_domainservices() -> Result<()> {
    run_pulumi_generator_test("azure", Some("domainservices"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-dynatrace"), ignore)]
fn azure_dynatrace() -> Result<()> {
    run_pulumi_generator_test("azure", Some("dynatrace"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-elasticcloud"), ignore)]
fn azure_elasticcloud() -> Result<()> {
    run_pulumi_generator_test("azure", Some("elasticcloud"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-elasticsan"), ignore)]
fn azure_elasticsan() -> Result<()> {
    run_pulumi_generator_test("azure", Some("elasticsan"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-eventgrid"), ignore)]
fn azure_eventgrid() -> Result<()> {
    run_pulumi_generator_test("azure", Some("eventgrid"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-eventhub"), ignore)]
fn azure_eventhub() -> Result<()> {
    run_pulumi_generator_test("azure", Some("eventhub"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-expressroute"), ignore)]
fn azure_expressroute() -> Result<()> {
    run_pulumi_generator_test("azure", Some("expressroute"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-extendedlocation"), ignore)]
fn azure_extendedlocation() -> Result<()> {
    run_pulumi_generator_test("azure", Some("extendedlocation"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-fluidrelay"), ignore)]
fn azure_fluidrelay() -> Result<()> {
    run_pulumi_generator_test("azure", Some("fluidrelay"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-frontdoor"), ignore)]
fn azure_frontdoor() -> Result<()> {
    run_pulumi_generator_test("azure", Some("frontdoor"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-graph"), ignore)]
fn azure_graph() -> Result<()> {
    run_pulumi_generator_test("azure", Some("graph"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-hdinsight"), ignore)]
fn azure_hdinsight() -> Result<()> {
    run_pulumi_generator_test("azure", Some("hdinsight"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-healthcare"), ignore)]
fn azure_healthcare() -> Result<()> {
    run_pulumi_generator_test("azure", Some("healthcare"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-hpc"), ignore)]
fn azure_hpc() -> Result<()> {
    run_pulumi_generator_test("azure", Some("hpc"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-hsm"), ignore)]
fn azure_hsm() -> Result<()> {
    run_pulumi_generator_test("azure", Some("hsm"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-iot"), ignore)]
fn azure_iot() -> Result<()> {
    run_pulumi_generator_test("azure", Some("iot"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-iotcentral"), ignore)]
fn azure_iotcentral() -> Result<()> {
    run_pulumi_generator_test("azure", Some("iotcentral"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-keyvault"), ignore)]
fn azure_keyvault() -> Result<()> {
    run_pulumi_generator_test("azure", Some("keyvault"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-kusto"), ignore)]
fn azure_kusto() -> Result<()> {
    run_pulumi_generator_test("azure", Some("kusto"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-lb"), ignore)]
fn azure_lb() -> Result<()> {
    run_pulumi_generator_test("azure", Some("lb"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-lighthouse"), ignore)]
fn azure_lighthouse() -> Result<()> {
    run_pulumi_generator_test("azure", Some("lighthouse"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-loadtest"), ignore)]
fn azure_loadtest() -> Result<()> {
    run_pulumi_generator_test("azure", Some("loadtest"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-loganalytics"), ignore)]
fn azure_loganalytics() -> Result<()> {
    run_pulumi_generator_test("azure", Some("loganalytics"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-logicapps"), ignore)]
fn azure_logicapps() -> Result<()> {
    run_pulumi_generator_test("azure", Some("logicapps"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-machinelearning"), ignore)]
fn azure_machinelearning() -> Result<()> {
    run_pulumi_generator_test("azure", Some("machinelearning"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-maintenance"), ignore)]
fn azure_maintenance() -> Result<()> {
    run_pulumi_generator_test("azure", Some("maintenance"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-managedapplication"), ignore)]
fn azure_managedapplication() -> Result<()> {
    run_pulumi_generator_test("azure", Some("managedapplication"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-managedlustre"), ignore)]
fn azure_managedlustre() -> Result<()> {
    run_pulumi_generator_test("azure", Some("managedlustre"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-management"), ignore)]
fn azure_management() -> Result<()> {
    run_pulumi_generator_test("azure", Some("management"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-managementgroups"), ignore)]
fn azure_managementgroups() -> Result<()> {
    run_pulumi_generator_test("azure", Some("managementgroups"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-managementresource"), ignore)]
fn azure_managementresource() -> Result<()> {
    run_pulumi_generator_test("azure", Some("managementresource"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-maps"), ignore)]
fn azure_maps() -> Result<()> {
    run_pulumi_generator_test("azure", Some("maps"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-marketplace"), ignore)]
fn azure_marketplace() -> Result<()> {
    run_pulumi_generator_test("azure", Some("marketplace"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-mixedreality"), ignore)]
fn azure_mixedreality() -> Result<()> {
    run_pulumi_generator_test("azure", Some("mixedreality"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-mobile"), ignore)]
fn azure_mobile() -> Result<()> {
    run_pulumi_generator_test("azure", Some("mobile"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-monitoring"), ignore)]
fn azure_monitoring() -> Result<()> {
    run_pulumi_generator_test("azure", Some("monitoring"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-msi"), ignore)]
fn azure_msi() -> Result<()> {
    run_pulumi_generator_test("azure", Some("msi"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-mssql"), ignore)]
fn azure_mssql() -> Result<()> {
    run_pulumi_generator_test("azure", Some("mssql"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-mysql"), ignore)]
fn azure_mysql() -> Result<()> {
    run_pulumi_generator_test("azure", Some("mysql"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-netapp"), ignore)]
fn azure_netapp() -> Result<()> {
    run_pulumi_generator_test("azure", Some("netapp"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-network"), ignore)]
fn azure_network() -> Result<()> {
    run_pulumi_generator_test("azure", Some("network"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-networkfunction"), ignore)]
fn azure_networkfunction() -> Result<()> {
    run_pulumi_generator_test("azure", Some("networkfunction"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-newrelic"), ignore)]
fn azure_newrelic() -> Result<()> {
    run_pulumi_generator_test("azure", Some("newrelic"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-nginx"), ignore)]
fn azure_nginx() -> Result<()> {
    run_pulumi_generator_test("azure", Some("nginx"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-notificationhub"), ignore)]
fn azure_notificationhub() -> Result<()> {
    run_pulumi_generator_test("azure", Some("notificationhub"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-operationalinsights"), ignore)]
fn azure_operationalinsights() -> Result<()> {
    run_pulumi_generator_test("azure", Some("operationalinsights"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-oracle"), ignore)]
fn azure_oracle() -> Result<()> {
    run_pulumi_generator_test("azure", Some("oracle"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-orbital"), ignore)]
fn azure_orbital() -> Result<()> {
    run_pulumi_generator_test("azure", Some("orbital"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-paloalto"), ignore)]
fn azure_paloalto() -> Result<()> {
    run_pulumi_generator_test("azure", Some("paloalto"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-pim"), ignore)]
fn azure_pim() -> Result<()> {
    run_pulumi_generator_test("azure", Some("pim"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-policy"), ignore)]
fn azure_policy() -> Result<()> {
    run_pulumi_generator_test("azure", Some("policy"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-portal"), ignore)]
fn azure_portal() -> Result<()> {
    run_pulumi_generator_test("azure", Some("portal"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-postgresql"), ignore)]
fn azure_postgresql() -> Result<()> {
    run_pulumi_generator_test("azure", Some("postgresql"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-powerbi"), ignore)]
fn azure_powerbi() -> Result<()> {
    run_pulumi_generator_test("azure", Some("powerbi"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-privatedns"), ignore)]
fn azure_privatedns() -> Result<()> {
    run_pulumi_generator_test("azure", Some("privatedns"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-privatelink"), ignore)]
fn azure_privatelink() -> Result<()> {
    run_pulumi_generator_test("azure", Some("privatelink"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-proximity"), ignore)]
fn azure_proximity() -> Result<()> {
    run_pulumi_generator_test("azure", Some("proximity"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-purview"), ignore)]
fn azure_purview() -> Result<()> {
    run_pulumi_generator_test("azure", Some("purview"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-recoveryservices"), ignore)]
fn azure_recoveryservices() -> Result<()> {
    run_pulumi_generator_test("azure", Some("recoveryservices"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-redhatopenshift"), ignore)]
fn azure_redhatopenshift() -> Result<()> {
    run_pulumi_generator_test("azure", Some("redhatopenshift"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-redis"), ignore)]
fn azure_redis() -> Result<()> {
    run_pulumi_generator_test("azure", Some("redis"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-relay"), ignore)]
fn azure_relay() -> Result<()> {
    run_pulumi_generator_test("azure", Some("relay"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-role"), ignore)]
fn azure_role() -> Result<()> {
    run_pulumi_generator_test("azure", Some("role"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-search"), ignore)]
fn azure_search() -> Result<()> {
    run_pulumi_generator_test("azure", Some("search"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-securitycenter"), ignore)]
fn azure_securitycenter() -> Result<()> {
    run_pulumi_generator_test("azure", Some("securitycenter"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-sentinel"), ignore)]
fn azure_sentinel() -> Result<()> {
    run_pulumi_generator_test("azure", Some("sentinel"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-servicebus"), ignore)]
fn azure_servicebus() -> Result<()> {
    run_pulumi_generator_test("azure", Some("servicebus"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-servicefabric"), ignore)]
fn azure_servicefabric() -> Result<()> {
    run_pulumi_generator_test("azure", Some("servicefabric"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-signalr"), ignore)]
fn azure_signalr() -> Result<()> {
    run_pulumi_generator_test("azure", Some("signalr"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-siterecovery"), ignore)]
fn azure_siterecovery() -> Result<()> {
    run_pulumi_generator_test("azure", Some("siterecovery"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-stack"), ignore)]
fn azure_stack() -> Result<()> {
    run_pulumi_generator_test("azure", Some("stack"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-storage"), ignore)]
fn azure_storage() -> Result<()> {
    run_pulumi_generator_test("azure", Some("storage"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-streamanalytics"), ignore)]
fn azure_streamanalytics() -> Result<()> {
    run_pulumi_generator_test("azure", Some("streamanalytics"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-synapse"), ignore)]
fn azure_synapse() -> Result<()> {
    run_pulumi_generator_test("azure", Some("synapse"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-systemcenter"), ignore)]
fn azure_systemcenter() -> Result<()> {
    run_pulumi_generator_test("azure", Some("systemcenter"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-trafficmanager"), ignore)]
fn azure_trafficmanager() -> Result<()> {
    run_pulumi_generator_test("azure", Some("trafficmanager"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-trustedsigning"), ignore)]
fn azure_trustedsigning() -> Result<()> {
    run_pulumi_generator_test("azure", Some("trustedsigning"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-videoindexer"), ignore)]
fn azure_videoindexer() -> Result<()> {
    run_pulumi_generator_test("azure", Some("videoindexer"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-voice"), ignore)]
fn azure_voice() -> Result<()> {
    run_pulumi_generator_test("azure", Some("voice"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-waf"), ignore)]
fn azure_waf() -> Result<()> {
    run_pulumi_generator_test("azure", Some("waf"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-webpubsub"), ignore)]
fn azure_webpubsub() -> Result<()> {
    run_pulumi_generator_test("azure", Some("webpubsub"))
}

#[test]
#[cfg_attr(not(feature = "generator_azure-workloadssap"), ignore)]
fn azure_workloadssap() -> Result<()> {
    run_pulumi_generator_test("azure", Some("workloadssap"))
}

#[test]
#[cfg_attr(not(feature = "generator_filtering-ns1"), ignore)]
fn filtering_ns1() -> Result<()> {
    run_pulumi_generator_test("filtering", Some("ns1"))
}

#[test]
#[cfg_attr(not(feature = "generator_filtering-ns2"), ignore)]
fn filtering_ns2() -> Result<()> {
    run_pulumi_generator_test("filtering", Some("ns2"))
}
// DO NOT EDIT - END

// provider_name is `name` from yaml file
pub fn run_pulumi_generator_test(test_name: &str, filter: Option<&str>) -> Result<()> {
    let directory_name = match filter {
        Some(filter) => format!("{test_name}-{filter}"),
        None => test_name.to_string(),
    };
    let root_path = format!("tests/output/{directory_name}");
    let root = Path::new(&root_path);

    let schema = find_schema_files(test_name);
    fs::create_dir_all(root)?;

    create_symlink(&schema, &root.join(schema.file_name().unwrap()))?;

    generate_combined(
        schema.as_path(),
        &root.join("src").join("generated"),
        filter,
    )?;

    let times = FileTimes::new().set_modified(SystemTime::UNIX_EPOCH);

    let cargo_toml_content = CargoToml {
        version: directory_name.as_str(),
    }
    .render()?;
    let lib_rs = root.join("src/lib.rs");
    fs::write(root.join("Cargo.toml"), cargo_toml_content)?;
    fs::create_dir_all(root.join("src"))?;
    fs::write(&lib_rs, "include!(\"generated/main.rs\");")?;
    fs::copy("../rust-toolchain.toml", root.join("rust-toolchain.toml"))?;
    File::options()
        .write(true)
        .open(lib_rs)
        .context("Cannot open file")?
        .set_times(times)
        .context("Cannot set times")?;

    Command::new("cargo")
        .args(["component", "build"])
        .env_remove("CARGO_LLVM_COV")
        .env_remove("RUSTFLAGS")
        .env("CARGO_TARGET_DIR", "../__target")
        .env("CARGO_INCREMENTAL", "0")
        .current_dir(root)
        .assert()
        .success();

    Command::new("cargo")
        .args(["test", "--doc"])
        .env_remove("CARGO_LLVM_COV")
        .env_remove("RUSTFLAGS")
        .env("CARGO_TARGET_DIR", "../__target")
        .env("CARGO_INCREMENTAL", "0")
        .current_dir(root)
        .assert()
        .success();

    Command::new("cargo")
        .args(["doc", "--no-deps"])
        .env_remove("CARGO_LLVM_COV")
        .env_remove("RUSTFLAGS")
        .env("CARGO_TARGET_DIR", "../__target")
        .env("CARGO_INCREMENTAL", "0")
        .current_dir(root)
        .assert()
        .success();

    Ok(())
}

pub fn find_schema_files(name: &str) -> PathBuf {
    let possible_paths = vec![
        Path::new("tests/test_cases").join(format!("{name}.json")),
        Path::new("../providers").join(format!("{name}.json")),
        Path::new("../pulumi/tests/testdata/codegen")
            .join(name)
            .join("schema.yaml"),
        Path::new("../pulumi/tests/testdata/codegen")
            .join(name)
            .join("schema.json"),
        Path::new("../pulumi/tests/testdata/codegen").join(format!("{name}.yaml")),
        Path::new("../pulumi/tests/testdata/codegen").join(format!("{name}.json")),
        Path::new("../pulumi-java/pkg/codegen/testing/test/testdata")
            .join(name)
            .join("schema.yaml"),
        Path::new("../pulumi-java/pkg/codegen/testing/test/testdata")
            .join(name)
            .join("schema.json"),
        Path::new("../pulumi-java/pkg/codegen/testing/test/testdata").join(format!("{name}.yaml")),
        Path::new("../pulumi-java/pkg/codegen/testing/test/testdata").join(format!("{name}.json")),
    ];

    for path in possible_paths {
        if path.exists() {
            return path;
        }
    }

    panic!("No schema file found for provider: {name}");
}

fn create_symlink(src: &Path, dst: &Path) -> std::io::Result<()> {
    if dst.exists() {
        fs::remove_file(dst)?;
    }
    use pathdiff::diff_paths;
    let relative_path = diff_paths(src, dst.parent().unwrap()).unwrap();
    #[cfg(unix)]
    std::os::unix::fs::symlink(&relative_path, dst)?;
    #[cfg(windows)]
    std::os::windows::fs::symlink_file(&relative_path, dst)?;
    Ok(())
}

#[derive(Template)]
#[template(path = "test_Cargo.toml.jinja")]
struct CargoToml<'a> {
    version: &'a str,
}
