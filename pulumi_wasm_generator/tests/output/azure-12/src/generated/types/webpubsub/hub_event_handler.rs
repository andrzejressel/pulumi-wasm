#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct HubEventHandler {
    /// An `auth` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "auth")]
    pub r#auth: Box<Option<super::super::types::webpubsub::HubEventHandlerAuth>>,
    /// Specifies the list of system events. Supported values are `connect`, `connected` and `disconnected`.
    #[builder(into, default)]
    #[serde(rename = "systemEvents")]
    pub r#system_events: Box<Option<Vec<String>>>,
    /// The Event Handler URL Template. Two predefined parameters `{hub}` and `{event}` are available to use in the template. The value of the EventHandler URL is dynamically calculated when the client request comes in. Example: `http://example.com/api/{hub}/{event}`.
    #[builder(into)]
    #[serde(rename = "urlTemplate")]
    pub r#url_template: Box<String>,
    /// Specifies the matching event names. There are 3 kind of patterns supported: * `*` matches any event name * `,` Combine multiple events with `,` for example `event1,event2`, it matches event `event1` and `event2` * The single event name, for example `event1`, it matches `event1`.
    #[builder(into, default)]
    #[serde(rename = "userEventPattern")]
    pub r#user_event_pattern: Box<Option<String>>,
}
