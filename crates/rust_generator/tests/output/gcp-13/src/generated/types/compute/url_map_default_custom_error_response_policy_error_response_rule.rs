#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct UrlMapDefaultCustomErrorResponsePolicyErrorResponseRule {
    /// Valid values include:
    /// - A number between 400 and 599: For example 401 or 503, in which case the load balancer applies the policy if the error code exactly matches this value.
    /// - 5xx: Load Balancer will apply the policy if the backend service responds with any response code in the range of 500 to 599.
    /// - 4xx: Load Balancer will apply the policy if the backend service responds with any response code in the range of 400 to 499.
    /// Values must be unique within matchResponseCodes and across all errorResponseRules of CustomErrorResponsePolicy.
    #[builder(into, default)]
    #[serde(rename = "matchResponseCodes")]
    pub r#match_response_codes: Box<Option<Vec<String>>>,
    /// The HTTP status code returned with the response containing the custom error content.
    /// If overrideResponseCode is not supplied, the same response code returned by the original backend bucket or backend service is returned to the client.
    #[builder(into, default)]
    #[serde(rename = "overrideResponseCode")]
    pub r#override_response_code: Box<Option<i32>>,
    /// The full path to a file within backendBucket. For example: /errors/defaultError.html
    /// path must start with a leading slash. path cannot have trailing slashes.
    /// If the file is not available in backendBucket or the load balancer cannot reach the BackendBucket, a simple Not Found Error is returned to the client.
    /// The value must be from 1 to 1024 characters.
    #[builder(into, default)]
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
}
