use std::collections::HashMap;

use aws_config::SdkConfig;
use aws_sdk_dynamodb::{
    Client,
    types::{AttributeValue, KeysAndAttributes},
};
use dynamodb_expression::Expression;
use serde::Deserialize;

/// Creates a DynamoDB client from an SDK config.
pub async fn connect(aws_config: &SdkConfig) -> Client {
    Client::new(aws_config)
}

/// Fetches a single item by primary key, deserializing it into `item`.
///
/// Returns `true` if the item was found and deserialized, `false` if it
/// does not exist. The `expression` controls projection (which attributes
/// are returned).
pub async fn get_item<'a, I>(
    client: &Client,
    table_name: impl Into<String>,
    key: HashMap<String, AttributeValue>,
    expression: Expression,
    item: &mut I,
) -> anyhow::Result<bool>
where
    I: Deserialize<'a>,
{
    let output = client
        .get_item()
        .table_name(table_name)
        .set_key(Some(key))
        .set_expression_attribute_names(expression.names().clone())
        .set_projection_expression(expression.projection().cloned())
        .send()
        .await?;

    match output.item {
        Some(i) => {
            *item = serde_dynamo::from_item(i)?;
            Ok(true)
        }
        None => Ok(false),
    }
}

/// Batch-fetches items across one or more tables, retrying unprocessed keys.
///
/// `request_items` maps each table name to a list of keys and a projection
/// expression. For each returned item, `item_cb` is called with:
/// - the table name the item came from
/// - the raw attribute map
/// - a deserializer closure — call it with `&mut I` to populate your value
///
/// The callback returns `(I, bool)`; returning `true` for the bool halts
/// iteration early. Unprocessed keys from DynamoDB are automatically retried
/// until all items have been fetched or the callback signals a stop.
pub async fn get_items<'a, I>(
    client: &Client,
    mut request_items: HashMap<String, (Vec<HashMap<String, AttributeValue>>, Expression)>,
    mut item_cb: impl FnMut(
        &str,
        &HashMap<String, AttributeValue>,
        Box<dyn FnOnce(&mut I) -> anyhow::Result<()>>,
    ) -> anyhow::Result<(I, bool)>,
) -> anyhow::Result<()>
where
    I: Deserialize<'a>,
{
    let mut req_items = HashMap::new();
    for (table_name, keys_attrs) in request_items.drain() {
        let mut builder = KeysAndAttributes::builder()
            .set_expression_attribute_names(keys_attrs.1.names().clone())
            .set_projection_expression(keys_attrs.1.projection().cloned());

        for key in keys_attrs.0 {
            builder = builder.keys(key);
        }

        req_items.insert(table_name, builder.build()?);
    }

    let mut request_items = req_items;
    loop {
        let output = client
            .batch_get_item()
            .set_request_items(Some(request_items))
            .send()
            .await?;

        let mut responses = output.responses.unwrap_or_default();
        for (table_name, items) in responses.drain() {
            for i in items {
                let (_, stop) = item_cb(
                    &table_name,
                    &i,
                    Box::new({
                        let i = i.clone();
                        |item: &mut I| {
                            *item = serde_dynamo::from_item(i)?;
                            Ok(())
                        }
                    }),
                )?;

                if stop {
                    break;
                }
            }
        }

        match output.unprocessed_keys {
            Some(unprocessed_keys) => {
                if unprocessed_keys.is_empty() {
                    break;
                }
                request_items = unprocessed_keys;
            }
            None => break,
        }
    }

    Ok(())
}

/// Shared pagination loop used by all `query*` functions.
///
/// Iterates pages of DynamoDB query results, calling `item_cb` for each item
/// until `limit` items have been processed, the callback returns `stop=true`,
/// or there are no more pages. Setting `forward=false` reads in descending
/// sort-key order.
async fn do_query<'a, I>(
    client: &Client,
    table_name: impl Into<String>,
    expression: Expression,
    limit: Option<i32>,
    index_name: Option<String>,
    forward: bool,
    mut item_cb: impl FnMut(
        &HashMap<String, AttributeValue>,
        Box<dyn FnOnce(&mut I) -> anyhow::Result<()>>,
    ) -> anyhow::Result<(I, bool)>,
) -> anyhow::Result<()>
where
    I: Deserialize<'a>,
{
    let table_name = table_name.into();

    let mut item_count = 0;
    let mut exclusive_start_key = None;
    loop {
        let output = client
            .query()
            .table_name(table_name.clone())
            .set_index_name(index_name.clone())
            .set_expression_attribute_names(expression.names().clone())
            .set_expression_attribute_values(expression.values().clone())
            .set_key_condition_expression(expression.key_condition().cloned())
            .set_filter_expression(expression.filter().cloned())
            .set_projection_expression(expression.projection().cloned())
            .set_limit(limit)
            .scan_index_forward(forward)
            .set_exclusive_start_key(exclusive_start_key)
            .send()
            .await?;

        let mut stop_query = false;
        let items = output.items.unwrap_or_default();
        for i in &items {
            let (_, stop) = item_cb(
                i,
                Box::new({
                    let i = i.clone();
                    |item: &mut I| {
                        *item = serde_dynamo::from_item(i)?;
                        Ok(())
                    }
                }),
            )?;

            if stop {
                stop_query = true;
                break;
            }
        }

        if stop_query {
            break;
        }

        item_count += items.len() as i32;
        if let Some(limit) = limit
            && item_count >= limit
        {
            break;
        }

        exclusive_start_key = output.last_evaluated_key;
        if exclusive_start_key.is_none() {
            break;
        }
    }

    Ok(())
}

/// Queries a table in ascending sort-key order, paging automatically.
///
/// `item_cb` is called for each item with the raw attribute map and a
/// deserializer closure. Return `(item, true)` from the callback to stop
/// early. `limit` caps the total number of items processed across all pages.
pub async fn query<'a, I>(
    client: &Client,
    table_name: impl Into<String>,
    expression: Expression,
    limit: Option<i32>,
    item_cb: impl FnMut(
        &HashMap<String, AttributeValue>,
        Box<dyn FnOnce(&mut I) -> anyhow::Result<()>>,
    ) -> anyhow::Result<(I, bool)>,
) -> anyhow::Result<()>
where
    I: Deserialize<'a>,
{
    do_query(client, table_name, expression, limit, None, true, item_cb).await
}

/// Queries a GSI in ascending sort-key order, paging automatically.
///
/// Like [`query`] but targets a named global secondary index.
pub async fn query_index<'a, I>(
    client: &Client,
    table_name: impl Into<String>,
    expression: Expression,
    limit: Option<i32>,
    index_name: impl Into<String>,
    item_cb: impl FnMut(
        &HashMap<String, AttributeValue>,
        Box<dyn FnOnce(&mut I) -> anyhow::Result<()>>,
    ) -> anyhow::Result<(I, bool)>,
) -> anyhow::Result<()>
where
    I: Deserialize<'a>,
{
    do_query(
        client,
        table_name,
        expression,
        limit,
        Some(index_name.into()),
        true,
        item_cb,
    )
    .await
}

/// Queries a table in descending sort-key order, paging automatically.
///
/// Like [`query`] but reads newest-first (descending sort key).
pub async fn query_descending<'a, I>(
    client: &Client,
    table_name: impl Into<String>,
    expression: Expression,
    limit: Option<i32>,
    item_cb: impl FnMut(
        &HashMap<String, AttributeValue>,
        Box<dyn FnOnce(&mut I) -> anyhow::Result<()>>,
    ) -> anyhow::Result<(I, bool)>,
) -> anyhow::Result<()>
where
    I: Deserialize<'a>,
{
    do_query(client, table_name, expression, limit, None, false, item_cb).await
}

/// Queries a GSI in descending sort-key order, paging automatically.
///
/// Like [`query_index`] but reads newest-first (descending sort key).
pub async fn query_index_descending<'a, I>(
    client: &Client,
    table_name: impl Into<String>,
    expression: Expression,
    limit: Option<i32>,
    index_name: impl Into<String>,
    item_cb: impl FnMut(
        &HashMap<String, AttributeValue>,
        Box<dyn FnOnce(&mut I) -> anyhow::Result<()>>,
    ) -> anyhow::Result<(I, bool)>,
) -> anyhow::Result<()>
where
    I: Deserialize<'a>,
{
    do_query(
        client,
        table_name,
        expression,
        limit,
        Some(index_name.into()),
        false,
        item_cb,
    )
    .await
}

/// Shared pagination loop used by all `scan*` functions.
///
/// Iterates pages of DynamoDB scan results, calling `item_cb` for each item
/// until `limit` items have been processed, the callback returns `stop=true`,
/// or there are no more pages.
async fn do_scan<'a, I>(
    client: &Client,
    table_name: impl Into<String>,
    expression: Expression,
    limit: Option<i32>,
    index_name: Option<String>,
    mut item_cb: impl FnMut(
        &HashMap<String, AttributeValue>,
        Box<dyn FnOnce(&mut I) -> anyhow::Result<()>>,
    ) -> anyhow::Result<(I, bool)>,
) -> anyhow::Result<()>
where
    I: Deserialize<'a>,
{
    let table_name = table_name.into();

    let mut item_count = 0;
    let mut exclusive_start_key = None;
    loop {
        let output = client
            .scan()
            .table_name(table_name.clone())
            .set_index_name(index_name.clone())
            .set_expression_attribute_names(expression.names().clone())
            .set_expression_attribute_values(expression.values().clone())
            .set_filter_expression(expression.filter().cloned())
            .set_projection_expression(expression.projection().cloned())
            .set_limit(limit)
            .set_exclusive_start_key(exclusive_start_key)
            .send()
            .await?;

        let mut stop_scan = false;
        let items = output.items.unwrap_or_default();
        for i in &items {
            let (_, stop) = item_cb(
                i,
                Box::new({
                    let i = i.clone();
                    |item: &mut I| {
                        *item = serde_dynamo::from_item(i)?;
                        Ok(())
                    }
                }),
            )?;

            if stop {
                stop_scan = true;
                break;
            }
        }

        if stop_scan {
            break;
        }

        item_count += items.len() as i32;
        if let Some(limit) = limit
            && item_count >= limit
        {
            break;
        }

        exclusive_start_key = output.last_evaluated_key;
        if exclusive_start_key.is_none() {
            break;
        }
    }

    Ok(())
}

/// Scans an entire table with an optional filter, paging automatically.
///
/// `item_cb` is called for each item with the raw attribute map and a
/// deserializer closure. Return `(item, true)` from the callback to stop
/// early. `limit` caps the total number of items processed across all pages.
pub async fn scan<'a, I>(
    client: &Client,
    table_name: impl Into<String>,
    expression: Expression,
    limit: Option<i32>,
    item_cb: impl FnMut(
        &HashMap<String, AttributeValue>,
        Box<dyn FnOnce(&mut I) -> anyhow::Result<()>>,
    ) -> anyhow::Result<(I, bool)>,
) -> anyhow::Result<()>
where
    I: Deserialize<'a>,
{
    do_scan(client, table_name, expression, limit, None, item_cb).await
}

/// Scans a GSI with an optional filter, paging automatically.
///
/// Like [`scan`] but targets a named global secondary index.
pub async fn scan_index<'a, I>(
    client: &Client,
    table_name: impl Into<String>,
    expression: Expression,
    limit: Option<i32>,
    index_name: impl Into<String>,
    item_cb: impl FnMut(
        &HashMap<String, AttributeValue>,
        Box<dyn FnOnce(&mut I) -> anyhow::Result<()>>,
    ) -> anyhow::Result<(I, bool)>,
) -> anyhow::Result<()>
where
    I: Deserialize<'a>,
{
    do_scan(
        client,
        table_name,
        expression,
        limit,
        Some(index_name.into()),
        item_cb,
    )
    .await
}
