#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetAccountSasPermissions {
    /// Should Add permissions be enabled for this SAS?
    #[builder(into)]
    #[serde(rename = "add")]
    pub r#add: Box<bool>,
    /// Should Create permissions be enabled for this SAS?
    #[builder(into)]
    #[serde(rename = "create")]
    pub r#create: Box<bool>,
    /// Should Delete permissions be enabled for this SAS?
    #[builder(into)]
    #[serde(rename = "delete")]
    pub r#delete: Box<bool>,
    /// Should Filter by Index Tags permissions be enabled for this SAS?
    /// 
    /// Refer to the [SAS creation reference from Azure](https://docs.microsoft.com/rest/api/storageservices/constructing-an-account-sas)
    /// for additional details on the fields above.
    #[builder(into)]
    #[serde(rename = "filter")]
    pub r#filter: Box<bool>,
    /// Should List permissions be enabled for this SAS?
    #[builder(into)]
    #[serde(rename = "list")]
    pub r#list: Box<bool>,
    /// Should Process permissions be enabled for this SAS?
    #[builder(into)]
    #[serde(rename = "process")]
    pub r#process: Box<bool>,
    /// Should Read permissions be enabled for this SAS?
    #[builder(into)]
    #[serde(rename = "read")]
    pub r#read: Box<bool>,
    /// Should Get / Set Index Tags permissions be enabled for this SAS?
    #[builder(into)]
    #[serde(rename = "tag")]
    pub r#tag: Box<bool>,
    /// Should Update permissions be enabled for this SAS?
    #[builder(into)]
    #[serde(rename = "update")]
    pub r#update: Box<bool>,
    /// Should Write permissions be enabled for this SAS?
    #[builder(into)]
    #[serde(rename = "write")]
    pub r#write: Box<bool>,
}
