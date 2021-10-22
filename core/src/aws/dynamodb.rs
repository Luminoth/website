use std::collections::HashMap;
use std::str::FromStr;

use dynamodb_expression::Expression;
use rusoto_core::*;
use rusoto_dynamodb::*;

pub async fn connect(region: impl AsRef<str>) -> anyhow::Result<DynamoDbClient> {
    Ok(DynamoDbClient::new(Region::from_str(region.as_ref())?))
}

pub async fn get_item<I>(
    client: &DynamoDbClient,
    table_name: impl Into<String>,
    key: impl dynomite::Item,
    expression: Expression,
    item: &mut I,
) -> anyhow::Result<bool>
where
    I: dynomite::Item,
{
    let table_name = table_name.into();

    let output = client
        .get_item(GetItemInput {
            table_name,
            key: key.key(),
            expression_attribute_names: expression.names().clone(),
            projection_expression: expression.projection().cloned(),
            ..Default::default()
        })
        .await?;

    match output.item {
        Some(i) => {
            *item = I::from_attrs(i)?;
            Ok(true)
        }
        None => Ok(false),
    }
}

pub async fn get_items<I>(
    client: &DynamoDbClient,
    mut request_items: HashMap<String, (Vec<impl dynomite::Item>, Expression)>,
    mut item_cb: impl FnMut(
        &str,
        &HashMap<String, AttributeValue>,
        Box<dyn FnOnce(&mut I) -> anyhow::Result<()>>,
    ) -> anyhow::Result<(I, bool)>,
) -> anyhow::Result<()>
where
    I: dynomite::Item,
{
    let mut req_items = HashMap::new();
    for (table_name, keys_attrs) in request_items.drain() {
        let mut keys = Vec::new();
        for key in keys_attrs.0 {
            keys.push(key.key());
        }

        req_items.insert(
            table_name,
            KeysAndAttributes {
                keys,
                expression_attribute_names: keys_attrs.1.names().clone(),
                projection_expression: keys_attrs.1.projection().cloned(),
                ..Default::default()
            },
        );
    }

    let mut request_items = req_items;
    loop {
        let output = client
            .batch_get_item(BatchGetItemInput {
                request_items,
                ..Default::default()
            })
            .await?;

        let mut responses = output.responses.unwrap_or_else(HashMap::new);
        for (table_name, items) in responses.drain() {
            for i in items {
                let (_, stop) = item_cb(
                    &table_name,
                    &i,
                    Box::new({
                        let i = i.clone();
                        |item: &mut I| {
                            *item = I::from_attrs(i)?;
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

async fn do_query<I>(
    client: &DynamoDbClient,
    table_name: impl Into<String>,
    expression: Expression,
    limit: Option<i64>,
    index_name: Option<String>,
    forward: bool,
    mut item_cb: impl FnMut(
        &HashMap<String, AttributeValue>,
        Box<dyn FnOnce(&mut I) -> anyhow::Result<()>>,
    ) -> anyhow::Result<(I, bool)>,
) -> anyhow::Result<()>
where
    I: dynomite::Item,
{
    let table_name = table_name.into();

    let mut item_count = 0;
    let mut exclusive_start_key = None;
    loop {
        let output = client
            .query(QueryInput {
                table_name: table_name.clone(),
                expression_attribute_names: expression.names().clone(),
                expression_attribute_values: expression.values().clone(),
                key_condition_expression: expression.key_condition().cloned(),
                filter_expression: expression.filter().cloned(),
                projection_expression: expression.projection().cloned(),
                index_name: index_name.clone(),
                limit,
                scan_index_forward: Some(forward),
                exclusive_start_key,
                ..Default::default()
            })
            .await?;

        let mut stop_query = false;
        let items = output.items.unwrap_or_else(Vec::new);
        for i in &items {
            let (_, stop) = item_cb(
                i,
                Box::new({
                    let i = i.clone();
                    |item: &mut I| {
                        *item = I::from_attrs(i)?;
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

        item_count += items.len() as i64;
        if let Some(limit) = limit {
            if item_count >= limit {
                break;
            }
        }

        exclusive_start_key = output.last_evaluated_key;
        if exclusive_start_key.is_none() {
            break;
        }
    }

    Ok(())
}

pub async fn query<I>(
    client: &DynamoDbClient,
    table_name: impl Into<String>,
    expression: Expression,
    limit: Option<i64>,
    item_cb: impl FnMut(
        &HashMap<String, AttributeValue>,
        Box<dyn FnOnce(&mut I) -> anyhow::Result<()>>,
    ) -> anyhow::Result<(I, bool)>,
) -> anyhow::Result<()>
where
    I: dynomite::Item,
{
    do_query(client, table_name, expression, limit, None, true, item_cb).await
}

pub async fn query_index<I>(
    client: &DynamoDbClient,
    table_name: impl Into<String>,
    expression: Expression,
    limit: Option<i64>,
    index_name: impl Into<String>,
    item_cb: impl FnMut(
        &HashMap<String, AttributeValue>,
        Box<dyn FnOnce(&mut I) -> anyhow::Result<()>>,
    ) -> anyhow::Result<(I, bool)>,
) -> anyhow::Result<()>
where
    I: dynomite::Item,
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

pub async fn query_descending<I>(
    client: &DynamoDbClient,
    table_name: impl Into<String>,
    expression: Expression,
    limit: Option<i64>,
    item_cb: impl FnMut(
        &HashMap<String, AttributeValue>,
        Box<dyn FnOnce(&mut I) -> anyhow::Result<()>>,
    ) -> anyhow::Result<(I, bool)>,
) -> anyhow::Result<()>
where
    I: dynomite::Item,
{
    do_query(client, table_name, expression, limit, None, false, item_cb).await
}

pub async fn query_index_descending<I>(
    client: &DynamoDbClient,
    table_name: impl Into<String>,
    expression: Expression,
    limit: Option<i64>,
    index_name: impl Into<String>,
    item_cb: impl FnMut(
        &HashMap<String, AttributeValue>,
        Box<dyn FnOnce(&mut I) -> anyhow::Result<()>>,
    ) -> anyhow::Result<(I, bool)>,
) -> anyhow::Result<()>
where
    I: dynomite::Item,
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

async fn do_scan<I>(
    client: &DynamoDbClient,
    table_name: impl Into<String>,
    expression: Expression,
    limit: Option<i64>,
    index_name: Option<String>,
    mut item_cb: impl FnMut(
        &HashMap<String, AttributeValue>,
        Box<dyn FnOnce(&mut I) -> anyhow::Result<()>>,
    ) -> anyhow::Result<(I, bool)>,
) -> anyhow::Result<()>
where
    I: dynomite::Item,
{
    let table_name = table_name.into();

    let mut item_count = 0;
    let mut exclusive_start_key = None;
    loop {
        let output = client
            .scan(ScanInput {
                table_name: table_name.clone(),
                expression_attribute_names: expression.names().clone(),
                expression_attribute_values: expression.values().clone(),
                filter_expression: expression.filter().cloned(),
                projection_expression: expression.projection().cloned(),
                index_name: index_name.clone(),
                limit,
                exclusive_start_key,
                ..Default::default()
            })
            .await?;

        let mut stop_scan = false;
        let items = output.items.unwrap_or_else(Vec::new);
        for i in &items {
            let (_, stop) = item_cb(
                i,
                Box::new({
                    let i = i.clone();
                    |item: &mut I| {
                        *item = I::from_attrs(i)?;
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

        item_count += items.len() as i64;
        if let Some(limit) = limit {
            if item_count >= limit {
                break;
            }
        }

        exclusive_start_key = output.last_evaluated_key;
        if exclusive_start_key.is_none() {
            break;
        }
    }

    Ok(())
}

pub async fn scan<I>(
    client: &DynamoDbClient,
    table_name: impl Into<String>,
    expression: Expression,
    limit: Option<i64>,
    item_cb: impl FnMut(
        &HashMap<String, AttributeValue>,
        Box<dyn FnOnce(&mut I) -> anyhow::Result<()>>,
    ) -> anyhow::Result<(I, bool)>,
) -> anyhow::Result<()>
where
    I: dynomite::Item,
{
    do_scan(client, table_name, expression, limit, None, item_cb).await
}

pub async fn scan_index<I>(
    client: &DynamoDbClient,
    table_name: impl Into<String>,
    expression: Expression,
    limit: Option<i64>,
    index_name: impl Into<String>,
    item_cb: impl FnMut(
        &HashMap<String, AttributeValue>,
        Box<dyn FnOnce(&mut I) -> anyhow::Result<()>>,
    ) -> anyhow::Result<(I, bool)>,
) -> anyhow::Result<()>
where
    I: dynomite::Item,
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
