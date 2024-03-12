use crate::{utils, DbInstance};

use deadpool_tiberius::deadpool::managed::{self};
use futures_util::TryStreamExt;
use serde_json::{
    value::Value::{Bool as JsonBool, String as JsonString},
    Value as JsonValue,
};
use sqlx::Row;
use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::ops::Deref;
use tauri::State;
use tiberius::{Column, ColumnType, QueryItem};

use tokio::sync::MutexGuard;

pub async fn get_tables(db: &State<'_, DbInstance>) -> Result<Vec<String>, String> {
    //Result<Vec<String>, String> {
    let mut conn_long_lived = db.mssql_pool.lock().await;
    let pool = conn_long_lived.as_ref().unwrap();
    let mut conn = pool.get().await.unwrap();

    let mut sql_query = conn
        .query(
            "select TABLE_NAME from INFORMATION_SCHEMA.TABLES WHERE TABLE_TYPE = 'BASE TABLE';",
            &[],
        )
        .await
        .unwrap();

    let mut final_result: Vec<String> = Default::default();
    while let Some(query_item) = sql_query.try_next().await.unwrap() {
        match query_item {
            QueryItem::Metadata(meta) if meta.result_index() == 0 => {
                //metadata
            }

            QueryItem::Row(row) if row.result_index() == 0 => {
                //0...N rows
                let table_name_column: Option<&str> = row.get("TABLE_NAME");
                if let Some(value) = table_name_column {
                    println!("{:?}", value);
                    final_result.push(value.to_owned());
                } else {
                    println!("NULL");
                }
            }
            QueryItem::Metadata(_meta) => {
                //metadata
            }
            QueryItem::Row(row) => {
                println!("LENG");
                println!("{}", row.len());
            }
        }
    }
    Ok(final_result)
}

pub async fn get_columns_definition(
    db: &State<'_, DbInstance>,
    table_name: String,
) -> Result<HashMap<String, HashMap<String, JsonValue>>, String> {
    println!("Getting column defintinion");

    let mut conn_long_lived = db.mssql_pool.lock().await;
    let pool = conn_long_lived.as_ref().unwrap();
    let mut conn = pool.get().await.unwrap();
    let strr = "SELECT COL.COLUMN_NAME,
                COL.DATA_TYPE,
                CASE
                                WHEN COL.IS_NULLABLE = 'YES' THEN CAST(1 As BIT)
                                ELSE CAST(0 As BIT)
                END IS_NULLABLE,
                COL.COLUMN_DEFAULT,
                CASE
                                WHEN TC.CONSTRAINT_TYPE = 'PRIMARY KEY' THEN CAST(1 AS BIT)
                                ELSE CAST(0 As BIT)
                END IS_PK
            FROM INFORMATION_SCHEMA.COLUMNS AS COL
            LEFT JOIN INFORMATION_SCHEMA.CONSTRAINT_COLUMN_USAGE AS CCU ON COL.COLUMN_NAME = CCU.COLUMN_NAME
            LEFT JOIN INFORMATION_SCHEMA.TABLE_CONSTRAINTS AS TC ON TC.CONSTRAINT_NAME = CCU.CONSTRAINT_NAME
            WHERE COL.TABLE_NAME = 'Accounts';";

    let sql_query = conn.query(strr, &[]).await.unwrap();

    let mut result = HashMap::<String, HashMap<String, JsonValue>>::new();
    let vecc = sql_query.into_results().await.unwrap();

    for i in vecc.iter() {
        println!("{}", vecc.iter().count());
        println!("Getting info");
        for row in i.iter() {
            println!("i.iter");
            println!("{}", i.iter().count());
            let column_name: Option<&str> = row.get("COLUMN_NAME");
            let mut column_name_value: &str = "";
            let data_type: Option<&str> = row.get("DATA_TYPE");
            let mut data_type_value: &str = "";
            let is_nullable: Option<bool> = row.get("IS_NULLABLE");
            let mut is_nullable_value: bool = false;
            let column_default: Option<&str> = row.get("COLUMN_DEFAULT");
            let mut column_default_value: &str = "";
            let is_pk: Option<bool> = row.get("IS_PK");
            let mut is_pk_value: bool = false;
            println!("Getting colun");

            if let Some(value) = column_name {
                println!("columan name: {:?}", value);
                column_name_value = value;
            } else {
                println!("NULL");
            }
            if let Some(value) = data_type {
                println!("data type: {:?}", value);
                data_type_value = value;
            } else {
                println!("NULL");
            }
            if let Some(value) = is_nullable {
                println!("is nullable: {:?}", value);
                is_nullable_value = value;
            } else {
                println!("NULL");
            }
            if let Some(value) = column_default {
                column_default_value = value;
            } else {
                println!("NULLS");
                column_default_value = "NULL";
            }
            if let Some(value) = is_pk {
                is_pk_value = value;
            } else {
                println!("NULLS");
            }

            let column_props = utils::create_column_definition_map(
                JsonString("NVARCHAR".to_owned()),
                JsonBool(true),
                JsonValue::String("String".to_owned()),
                JsonBool(false),
            );
            result.insert(column_name_value.to_owned(), column_props);
        }
        for element in result.iter() {
            println!("{}", element.0);
            for x in element.1.iter() {
                println!("{}", x.0);
                println!("{}", x.1);
            }
        }
    }

    // rows.iter().for_each(|row| {
    //     let column_props = utils::create_column_definition_map(
    //         JsonString(row.get(1)),
    //         JsonBool(row.get::<bool, usize>(2)),
    //         decode::to_json(row.try_get_raw(3).unwrap()).unwrap(),
    //         JsonBool(row.get::<bool, usize>(4)),
    //     );
    //     result.insert(row.get(0), column_props);
    // });

    Ok(result)
}
