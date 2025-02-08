#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RecordData {
    #[builder(into, default)]
    #[serde(rename = "algorithm")]
    pub r#algorithm: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "altitude")]
    pub r#altitude: Box<Option<f64>>,
    #[builder(into, default)]
    #[serde(rename = "certificate")]
    pub r#certificate: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "content")]
    pub r#content: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "digest")]
    pub r#digest: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "digestType")]
    pub r#digest_type: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "fingerprint")]
    pub r#fingerprint: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "flags")]
    pub r#flags: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "keyTag")]
    pub r#key_tag: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "latDegrees")]
    pub r#lat_degrees: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "latDirection")]
    pub r#lat_direction: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "latMinutes")]
    pub r#lat_minutes: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "latSeconds")]
    pub r#lat_seconds: Box<Option<f64>>,
    #[builder(into, default)]
    #[serde(rename = "longDegrees")]
    pub r#long_degrees: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "longDirection")]
    pub r#long_direction: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "longMinutes")]
    pub r#long_minutes: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "longSeconds")]
    pub r#long_seconds: Box<Option<f64>>,
    #[builder(into, default)]
    #[serde(rename = "matchingType")]
    pub r#matching_type: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "order")]
    pub r#order: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "precisionHorz")]
    pub r#precision_horz: Box<Option<f64>>,
    #[builder(into, default)]
    #[serde(rename = "precisionVert")]
    pub r#precision_vert: Box<Option<f64>>,
    #[builder(into, default)]
    #[serde(rename = "preference")]
    pub r#preference: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "priority")]
    pub r#priority: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "proto")]
    pub r#proto: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "publicKey")]
    pub r#public_key: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "regex")]
    pub r#regex: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "replacement")]
    pub r#replacement: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "selector")]
    pub r#selector: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "service")]
    pub r#service: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "size")]
    pub r#size: Box<Option<f64>>,
    #[builder(into, default)]
    #[serde(rename = "tag")]
    pub r#tag: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "target")]
    pub r#target: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "usage")]
    pub r#usage: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "weight")]
    pub r#weight: Box<Option<i32>>,
}
