#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct RecordData {
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "algorithm")]
    pub r#algorithm: Box<Option<i32>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "altitude")]
    pub r#altitude: Box<Option<f64>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "certificate")]
    pub r#certificate: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "content")]
    pub r#content: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "digest")]
    pub r#digest: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "digestType")]
    pub r#digest_type: Box<Option<i32>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "fingerprint")]
    pub r#fingerprint: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "flags")]
    pub r#flags: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "keyTag")]
    pub r#key_tag: Box<Option<i32>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "latDegrees")]
    pub r#lat_degrees: Box<Option<i32>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "latDirection")]
    pub r#lat_direction: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "latMinutes")]
    pub r#lat_minutes: Box<Option<i32>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "latSeconds")]
    pub r#lat_seconds: Box<Option<f64>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "longDegrees")]
    pub r#long_degrees: Box<Option<i32>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "longDirection")]
    pub r#long_direction: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "longMinutes")]
    pub r#long_minutes: Box<Option<i32>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "longSeconds")]
    pub r#long_seconds: Box<Option<f64>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "matchingType")]
    pub r#matching_type: Box<Option<i32>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "order")]
    pub r#order: Box<Option<i32>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "precisionHorz")]
    pub r#precision_horz: Box<Option<f64>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "precisionVert")]
    pub r#precision_vert: Box<Option<f64>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "preference")]
    pub r#preference: Box<Option<i32>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "priority")]
    pub r#priority: Box<Option<i32>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "proto")]
    pub r#proto: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<Option<i32>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "publicKey")]
    pub r#public_key: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "regex")]
    pub r#regex: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "replacement")]
    pub r#replacement: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "selector")]
    pub r#selector: Box<Option<i32>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "service")]
    pub r#service: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "size")]
    pub r#size: Box<Option<f64>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "tag")]
    pub r#tag: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "target")]
    pub r#target: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "type")]
    pub r#type: Box<Option<i32>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "usage")]
    pub r#usage: Box<Option<i32>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "weight")]
    pub r#weight: Box<Option<i32>>,
}
