use crate::schema::IntegerEnumValue;
use crate::schema::NumberEnumValue;
use crate::schema::{PulumiMap, StringEnumValue};
use pulumi_wasm_provider_common;
use serde::Deserialize;
use std::collections::BTreeSet;

pulumi_wasm_provider_common::generate_string_const!(StringConstant, "string");
pulumi_wasm_provider_common::generate_string_const!(IntegerConstant, "integer");
pulumi_wasm_provider_common::generate_string_const!(NumberConstant, "number");
pulumi_wasm_provider_common::generate_string_const!(BooleanConstant, "boolean");
pulumi_wasm_provider_common::generate_string_const!(ArrayConstant, "array");
pulumi_wasm_provider_common::generate_string_const!(ObjectConstant, "object");

#[derive(Deserialize)]
pub(crate) struct PulumiSchema {
    #[serde(default)]
    types: PulumiMap<TopLevelType>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum TopLevelType {
    Object(MyTypeObject),
    StringEnum(StringEnumType),
    IntegerEnum(IntegerEnumType),
    NumberEnum(NumberEnumType),
}

#[derive(Deserialize)]
#[serde(untagged)]
enum InnerType {
    StringEnum(StringEnumType),
    IntegerEnum(IntegerEnumType),
    NumberEnum(NumberEnumType),
    Boolean(BooleanType),
    AnyString(AnyStringType),
    AnyNumber(AnyNumberType),
    AnyInteger(AnyIntegerType),
    HashMap(HashMapType),
    Ref(RefType),
    OneOf(OneOfType),
    Array(ArrayType),
}

#[derive(Deserialize)]
struct MyTypeObject {
    #[serde(rename = "type")]
    type_: ObjectConstant,
    description: Option<String>,
    properties: PulumiMap<InnerType>,
    #[serde(default)]
    required: BTreeSet<String>,
}

#[derive(Deserialize)]
struct StringEnumType {
    #[serde(rename = "type")]
    type_: StringConstant,
    #[serde(rename = "enum")]
    enum_: Vec<StringEnumValue>,
}

#[derive(Deserialize)]
struct IntegerEnumType {
    #[serde(rename = "type")]
    type_: IntegerConstant,
    #[serde(rename = "enum")]
    enum_: Vec<IntegerEnumValue>,
}

#[derive(Deserialize)]
struct NumberEnumType {
    #[serde(rename = "type")]
    type_: NumberConstant,
    #[serde(rename = "enum")]
    enum_: Vec<NumberEnumValue>,
}

#[derive(Deserialize)]
struct AnyStringType {
    #[serde(rename = "type")]
    type_: StringConstant,
}

#[derive(Deserialize)]
struct BooleanType {
    #[serde(rename = "type")]
    type_: BooleanConstant,
}

#[derive(Deserialize)]
struct AnyIntegerType {
    #[serde(rename = "type")]
    type_: IntegerConstant,
}

#[derive(Deserialize)]
struct AnyNumberType {
    #[serde(rename = "type")]
    type_: NumberConstant,
}

#[derive(Deserialize)]
struct RefType {
    #[serde(rename = "$ref")]
    ref_: String,
}

#[derive(Deserialize)]
struct OneOfType {
    #[serde(rename = "oneOf")]
    one_of: Vec<InnerType>,
}

#[derive(Deserialize)]
struct ArrayType {
    #[serde(rename = "type")]
    type_: ArrayConstant,
    items: Box<InnerType>,
}

#[derive(Deserialize)]
struct HashMapType {
    #[serde(rename = "type")]
    type_: ObjectConstant,
    #[serde(rename = "additionalProperties")]
    additional_properties: Box<InnerType>,
}
