use serde_json::Value as JsonValue;
use tiberius::ColumnType;

use crate::utils;

pub fn to_json(v: &str) -> Result<String, String> {
    //pub fn to_json(v: ColumnType) -> Result<JsonValue, String> {
    //     // if v.is_null() {
    //     //     return Ok(JsonValue::Null);
    //     // }

    // let res = "Sebas";
    // let res4 = match v {
    //     "bigint" => JsonValue::Array::Bool::Number::Object::String(ColumnType::int4),
    //     "int" => ColumnType::Int2,
    //     "nvarchar" => ColumnType::NVarchar,
    //     "bit" => ColumnType::Bit,
    //     "datetime" => ColumnType::Datetime,
    //     "uniqueidentifier" => ColumnType::Guid,
    // };

    /**

    */
    //     let rev = ColumnType::Guid;

    //     // let res = match v.type_info().name() {
    //     //     "CHAR" | "VARCHAR" | "TEXT" | "NAME" => {
    //     //         if let Ok(v) = ValueRef::to_owned(&v).try_decode() {
    //     //             JsonValue::String(v)
    //     //         } else {
    //     //             JsonValue::Null
    //     //         }
    //     //     }
    //     //     "FLOAT4" => {
    //     //         if let Ok(v) = ValueRef::to_owned(&v).try_decode::<f32>() {
    //     //             JsonValue::from(v)
    //     //         } else {
    //     //             JsonValue::Null
    //     //         }
    //     //     }
    //     //     "FLOAT8" => {
    //     //         if let Ok(v) = ValueRef::to_owned(&v).try_decode::<f64>() {
    //     //             JsonValue::from(v)
    //     //         } else {
    //     //             JsonValue::Null
    //     //         }
    //     //     }
    //     //     "INT2" => {
    //     //         if let Ok(v) = ValueRef::to_owned(&v).try_decode::<i16>() {
    //     //             JsonValue::Number(v.into())
    //     //         } else {
    //     //             JsonValue::Null
    //     //         }
    //     //     }
    //     //     "INT4" => {
    //     //         if let Ok(v) = ValueRef::to_owned(&v).try_decode::<i32>() {
    //     //             JsonValue::Number(v.into())
    //     //         } else {
    //     //             JsonValue::Null
    //     //         }
    //     //     }
    //     //     "INT8" => {
    //     //         if let Ok(v) = ValueRef::to_owned(&v).try_decode::<i64>() {
    //     //             JsonValue::Number(v.into())
    //     //         } else {
    //     //             JsonValue::Null
    //     //         }
    //     //     }
    //     //     "BOOL" => {
    //     //         if let Ok(v) = ValueRef::to_owned(&v).try_decode() {
    //     //             JsonValue::Bool(v)
    //     //         } else {
    //     //             JsonValue::Null
    //     //         }
    //     //     }
    //     //     "DATE" => {
    //     //         if let Ok(v) = ValueRef::to_owned(&v).try_decode::<Date>() {
    //     //             JsonValue::String(v.to_string())
    //     //         } else {
    //     //             JsonValue::Null
    //     //         }
    //     //     }
    //     //     "TIME" => {
    //     //         if let Ok(v) = ValueRef::to_owned(&v).try_decode::<Time>() {
    //     //             JsonValue::String(v.to_string())
    //     //         } else {
    //     //             JsonValue::Null
    //     //         }
    //     //     }
    //     //     "TIMESTAMP" => {
    //     //         if let Ok(v) = ValueRef::to_owned(&v).try_decode::<PrimitiveDateTime>() {
    //     //             JsonValue::String(v.to_string())
    //     //         } else {
    //     //             JsonValue::Null
    //     //         }
    //     //     }
    //     //     "TIMESTAMPTZ" => {
    //     //         if let Ok(v) = ValueRef::to_owned(&v).try_decode::<OffsetDateTime>() {
    //     //             JsonValue::String(v.to_string())
    //     //         } else {
    //     //             JsonValue::Null
    //     //         }
    //     //     }
    //     //     "JSON" | "JSONB" => ValueRef::to_owned(&v).try_decode().unwrap_or_default(),
    //     //     "BYTEA" => {
    //     //         if let Ok(v) = ValueRef::to_owned(&v).try_decode::<Vec<u8>>() {
    //     //             JsonValue::Array(v.into_iter().map(|n| JsonValue::Number(n.into())).collect())
    //     //         } else {
    //     //             JsonValue::Null
    //     //         }
    //     //     }
    //     //     "VOID" => JsonValue::Null,
    //     //     _ => return Err("Unsupported type".to_string()),
    //     // };
    Ok(v.to_owned())
}

/*

bigint

numeric

bit

smallint

decimal

smallmoney

int

tinyint

money

float

real
Date and time

date

datetimeoffset

datetime2

smalldatetime

datetime

time


char

varchar

text


nchar

nvarchar

ntext
Binary strings

binary

varbinary

image


cursor

rowversion

hierarchyid

uniqueidentifier

sql_variant

xml

Spatial Geometry Types

Spatial Geography Types

table

*/
