#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct MlTransformParameters {
    /// The parameters for the find matches algorithm. see Find Matches Parameters.
    #[builder(into)]
    #[serde(rename = "findMatchesParameters")]
    pub r#find_matches_parameters: Box<super::super::types::glue::MlTransformParametersFindMatchesParameters>,
    /// The type of machine learning transform. For information about the types of machine learning transforms, see [Creating Machine Learning Transforms](http://docs.aws.amazon.com/glue/latest/dg/add-job-machine-learning-transform.html).
    #[builder(into)]
    #[serde(rename = "transformType")]
    pub r#transform_type: Box<String>,
}
