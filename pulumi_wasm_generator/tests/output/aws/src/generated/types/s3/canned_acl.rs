#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum CannedAcl {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "public-read")]
    PublicRead,
    #[serde(rename = "public-read-write")]
    PublicReadWrite,
    #[serde(rename = "aws-exec-read")]
    AwsExecRead,
    #[serde(rename = "authenticated-read")]
    AuthenticatedRead,
    #[serde(rename = "bucket-owner-read")]
    BucketOwnerRead,
    #[serde(rename = "bucket-owner-full-control")]
    BucketOwnerFullControl,
    #[serde(rename = "log-delivery-write")]
    LogDeliveryWrite,
}
