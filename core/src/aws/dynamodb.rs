use std::collections::HashMap;

use aws_config::SdkConfig;
use aws_sdk_dynamodb::{
    Client,
    types::{AttributeValue, KeysAndAttributes},
};
use dynamodb_expression::Expression;
use serde::Deserialize;

pub async fn connect(aws_config: &SdkConfig) -> Client {
    Client::new(aws_config)
}

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
