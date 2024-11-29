#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct NotificationPolicyFilters {
    /// Targeted actions for alert.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "actions")]
    pub r#actions: Box<Option<Vec<String>>>,
    /// Affected components for alert. Available values: `API`, `API Shield`, `Access`, `Always Online`, `Analytics`, `Apps Marketplace`, `Argo Smart Routing`, `Audit Logs`, `Authoritative DNS`, `Billing`, `Bot Management`, `Bring Your Own IP (BYOIP)`, `Browser Isolation`, `CDN Cache Purge`, `CDN/Cache`, `Cache Reserve`, `Challenge Platform`, `Cloud Access Security Broker (CASB)`, `Community Site`, `DNS Root Servers`, `DNS Updates`, `Dashboard`, `Data Loss Prevention (DLP)`, `Developer's Site`, `Digital Experience Monitoring (DEX)`, `Distributed Web Gateway`, `Durable Objects`, `Email Routing`, `Ethereum Gateway`, `Firewall`, `Gateway`, `Geo-Key Manager`, `Image Resizing`, `Images`, `Infrastructure`, `Lists`, `Load Balancing and Monitoring`, `Logs`, `Magic Firewall`, `Magic Transit`, `Magic WAN`, `Magic WAN Connector`, `Marketing Site`, `Mirage`, `Network`, `Notifications`, `Observatory`, `Page Shield`, `Pages`, `R2`, `Radar`, `Randomness Beacon`, `Recursive DNS`, `Registrar`, `Registration Data Access Protocol (RDAP)`, `SSL Certificate Provisioning`, `SSL for SaaS Provisioning`, `Security Center`, `Snippets`, `Spectrum`, `Speed Optimizations`, `Stream`, `Support Site`, `Time Services`, `Trace`, `Tunnel`, `Turnstile`, `WARP`, `Waiting Room`, `Web Analytics`, `Workers`, `Workers KV`, `Workers Preview`, `Zaraz`, `Zero Trust`, `Zero Trust Dashboard`, `Zone Versioning`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "affectedComponents")]
    pub r#affected_components: Box<Option<Vec<String>>>,
    /// Filter on Points of Presence.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "airportCodes")]
    pub r#airport_codes: Box<Option<Vec<String>>>,
    /// Alert trigger preferences. Example: `slo`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "alertTriggerPreferences")]
    pub r#alert_trigger_preferences: Box<Option<Vec<String>>>,
    /// State of the pool to alert on.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "enableds")]
    pub r#enableds: Box<Option<Vec<String>>>,
    /// Environment of pages. Available values: `ENVIRONMENT_PREVIEW`, `ENVIRONMENT_PRODUCTION`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "environments")]
    pub r#environments: Box<Option<Vec<String>>>,
    /// Source configuration to alert on for pool or origin.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "eventSources")]
    pub r#event_sources: Box<Option<Vec<String>>>,
    /// Stream event type to alert on.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "eventTypes")]
    pub r#event_types: Box<Option<Vec<String>>>,
    /// Pages event to alert. Available values: `EVENT_DEPLOYMENT_STARTED`, `EVENT_DEPLOYMENT_FAILED`, `EVENT_DEPLOYMENT_SUCCESS`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "events")]
    pub r#events: Box<Option<Vec<String>>>,
    /// Alert grouping.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "groupBies")]
    pub r#group_bies: Box<Option<Vec<String>>>,
    /// Identifier health check. Required when using `filters.0.status`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "healthCheckIds")]
    pub r#health_check_ids: Box<Option<Vec<String>>>,
    /// The incident impact level that will trigger the dispatch of a notification. Available values: `INCIDENT_IMPACT_NONE`, `INCIDENT_IMPACT_MINOR`, `INCIDENT_IMPACT_MAJOR`, `INCIDENT_IMPACT_CRITICAL`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "incidentImpacts")]
    pub r#incident_impacts: Box<Option<Vec<String>>>,
    /// Stream input id to alert on.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "inputIds")]
    pub r#input_ids: Box<Option<Vec<String>>>,
    /// A numerical limit. Example: `100`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "limits")]
    pub r#limits: Box<Option<Vec<String>>>,
    /// Megabits per second threshold for dos alert.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "megabitsPerSeconds")]
    pub r#megabits_per_seconds: Box<Option<Vec<String>>>,
    /// Health status to alert on for pool or origin.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "newHealths")]
    pub r#new_healths: Box<Option<Vec<String>>>,
    /// Tunnel health status to alert on.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "newStatuses")]
    pub r#new_statuses: Box<Option<Vec<String>>>,
    /// Packets per second threshold for dos alert.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "packetsPerSeconds")]
    pub r#packets_per_seconds: Box<Option<Vec<String>>>,
    /// Load balancer pool identifier.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "poolIds")]
    pub r#pool_ids: Box<Option<Vec<String>>>,
    /// Product name. Available values: `worker_requests`, `worker_durable_objects_requests`, `worker_durable_objects_duration`, `worker_durable_objects_data_transfer`, `worker_durable_objects_stored_data`, `worker_durable_objects_storage_deletes`, `worker_durable_objects_storage_writes`, `worker_durable_objects_storage_reads`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "products")]
    pub r#products: Box<Option<Vec<String>>>,
    /// Identifier of pages project.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "projectIds")]
    pub r#project_ids: Box<Option<Vec<String>>>,
    /// Protocol to alert on for dos.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "protocols")]
    pub r#protocols: Box<Option<Vec<String>>>,
    /// Requests per second threshold for dos alert.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "requestsPerSeconds")]
    pub r#requests_per_seconds: Box<Option<Vec<String>>>,
    /// Selectors for alert. Valid options depend on the alert type.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "selectors")]
    pub r#selectors: Box<Option<Vec<String>>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "services")]
    pub r#services: Box<Option<Vec<String>>>,
    /// A numerical limit. Example: `99.9`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "slos")]
    pub r#slos: Box<Option<Vec<String>>>,
    /// Status to alert on.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "statuses")]
    pub r#statuses: Box<Option<Vec<String>>>,
    /// Target host to alert on for dos.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "targetHostnames")]
    pub r#target_hostnames: Box<Option<Vec<String>>>,
    /// Target ip to alert on for dos in CIDR notation.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "targetIps")]
    pub r#target_ips: Box<Option<Vec<String>>>,
    /// Target domain to alert on.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "targetZoneNames")]
    pub r#target_zone_names: Box<Option<Vec<String>>>,
    /// Tunnel IDs to alert on.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "tunnelIds")]
    pub r#tunnel_ids: Box<Option<Vec<String>>>,
    /// Tunnel Names to alert on.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "tunnelNames")]
    pub r#tunnel_names: Box<Option<Vec<String>>>,
    /// Filter for alert.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "wheres")]
    pub r#wheres: Box<Option<Vec<String>>>,
    /// A list of zone identifiers.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "zones")]
    pub r#zones: Box<Option<Vec<String>>>,
}
