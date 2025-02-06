#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RoutineRemoteFunctionOptions {
    /// Fully qualified name of the user-provided connection object which holds
    /// the authentication information to send requests to the remote service.
    /// Format: "projects/{projectId}/locations/{locationId}/connections/{connectionId}"
    #[builder(into, default)]
    #[serde(rename = "connection")]
    pub r#connection: Box<Option<String>>,
    /// Endpoint of the user-provided remote service, e.g.
    /// `https://us-east1-my_gcf_project.cloudfunctions.net/remote_add`
    #[builder(into, default)]
    #[serde(rename = "endpoint")]
    pub r#endpoint: Box<Option<String>>,
    /// Max number of rows in each batch sent to the remote service. If absent or if 0,
    /// BigQuery dynamically decides the number of rows in a batch.
    #[builder(into, default)]
    #[serde(rename = "maxBatchingRows")]
    pub r#max_batching_rows: Box<Option<String>>,
    /// User-defined context as a set of key/value pairs, which will be sent as function
    /// invocation context together with batched arguments in the requests to the remote
    /// service. The total number of bytes of keys and values must be less than 8KB.
    /// An object containing a list of "key": value pairs. Example:
    /// `{ "name": "wrench", "mass": "1.3kg", "count": "3" }`.
    #[builder(into, default)]
    #[serde(rename = "userDefinedContext")]
    pub r#user_defined_context: Box<Option<std::collections::HashMap<String, String>>>,
}
