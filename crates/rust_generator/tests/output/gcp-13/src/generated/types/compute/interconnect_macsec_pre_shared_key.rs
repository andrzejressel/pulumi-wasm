#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InterconnectMacsecPreSharedKey {
    /// (Optional, Deprecated)
    /// If set to true, the Interconnect connection is configured with a should-secure
    /// MACsec security policy, that allows the Google router to fallback to cleartext
    /// traffic if the MKA session cannot be established. By default, the Interconnect
    /// connection is configured with a must-secure security policy that drops all traffic
    /// if the MKA session cannot be established with your router.
    /// 
    /// > **Warning:** `failOpen` is deprecated and will be removed in a future major release. Use other `failOpen` instead.
    #[builder(into, default)]
    #[serde(rename = "failOpen")]
    pub r#fail_open: Box<Option<bool>>,
    /// A name for this pre-shared key. The name must be 1-63 characters long, and
    /// comply with RFC1035. Specifically, the name must be 1-63 characters long and match
    /// the regular expression `a-z?` which means the first character
    /// must be a lowercase letter, and all following characters must be a dash, lowercase
    /// letter, or digit, except the last character, which cannot be a dash.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// A RFC3339 timestamp on or after which the key is valid. startTime can be in the
    /// future. If the keychain has a single key, startTime can be omitted. If the keychain
    /// has multiple keys, startTime is mandatory for each key. The start times of keys must
    /// be in increasing order. The start times of two consecutive keys must be at least 6
    /// hours apart.
    #[builder(into, default)]
    #[serde(rename = "startTime")]
    pub r#start_time: Box<Option<String>>,
}
