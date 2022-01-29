use std::collections::HashMap;

use aws_sdk_dynamodb::{
    config,
    model::{AttributeValue, KeysAndAttributes},
    Blob, Client, Region,
};
use dynamodb_expression::Expression;

// helper trait for converting dynomite types to AWS SDK types
pub trait ToSdk<T> {
    fn to_sdk(self) -> T;
}

impl ToSdk<AttributeValue> for dynomite::AttributeValue {
    fn to_sdk(self) -> AttributeValue {
        if let Some(v) = self.b {
            AttributeValue::B(Blob::new(v.as_ref()))
        } else if let Some(v) = self.bool {
            AttributeValue::Bool(v)
        } else if let Some(mut v) = self.bs {
            AttributeValue::Bs(v.drain(..).map(|v| Blob::new(v.as_ref())).collect())
        } else if let Some(mut v) = self.l {
            AttributeValue::L(v.drain(..).map(|v| v.to_sdk()).collect())
        } else if let Some(mut v) = self.m {
            AttributeValue::M(v.drain().map(|(k, v)| (k, v.to_sdk())).collect())
        } else if let Some(v) = self.n {
            AttributeValue::N(v)
        } else if let Some(v) = self.ns {
            AttributeValue::Ns(v)
        } else if let Some(v) = self.null {
            AttributeValue::Null(v)
        } else if let Some(v) = self.s {
            AttributeValue::S(v)
        } else if let Some(v) = self.ss {
            AttributeValue::Ss(v)
        } else {
            unreachable!();
        }
    }
}

impl ToSdk<HashMap<String, AttributeValue>> for dynomite::Attributes {
    fn to_sdk(mut self) -> HashMap<String, AttributeValue> {
        self.drain().map(|(k, v)| (k, v.to_sdk())).collect()
    }
}

// helper trait for converting from AWS SDK types to dynomite types
pub trait ToRusoto<T> {
    fn to_rusoto(self) -> T;
}

impl ToRusoto<dynomite::AttributeValue> for AttributeValue {
    fn to_rusoto(self) -> dynomite::AttributeValue {
        match self {
            AttributeValue::B(v) => dynomite::dynamodb::AttributeValue {
                b: Some(v.into_inner().into()),
                ..Default::default()
            },
            AttributeValue::Bool(v) => dynomite::dynamodb::AttributeValue {
                bool: Some(v),
                ..Default::default()
            },
            AttributeValue::Bs(mut v) => dynomite::dynamodb::AttributeValue {
                bs: Some(v.drain(..).map(|v| v.into_inner().into()).collect()),
                ..Default::default()
            },
            AttributeValue::L(mut v) => dynomite::dynamodb::AttributeValue {
                l: Some(v.drain(..).map(|v| v.to_rusoto()).collect()),
                ..Default::default()
            },
            AttributeValue::M(mut v) => dynomite::dynamodb::AttributeValue {
                m: Some(v.drain().map(|(k, v)| (k, v.to_rusoto())).collect()),
                ..Default::default()
            },
            AttributeValue::N(v) => dynomite::dynamodb::AttributeValue {
                n: Some(v),
                ..Default::default()
            },
            AttributeValue::Ns(v) => dynomite::dynamodb::AttributeValue {
                ns: Some(v),
                ..Default::default()
            },
            AttributeValue::Null(v) => dynomite::dynamodb::AttributeValue {
                null: Some(v),
                ..Default::default()
            },
            AttributeValue::S(v) => dynomite::dynamodb::AttributeValue {
                s: Some(v),
                ..Default::default()
            },
            AttributeValue::Ss(v) => dynomite::dynamodb::AttributeValue {
                ss: Some(v),
                ..Default::default()
            },
            //AttributeValue::Unknown => unreachable!(),
            _ => unreachable!(),
        }
    }
}

impl ToRusoto<dynomite::Attributes> for HashMap<String, AttributeValue> {
    fn to_rusoto(mut self) -> dynomite::Attributes {
        self.drain().map(|(k, v)| (k, v.to_rusoto())).collect()
    }
}

pub async fn connect(region: impl Into<String>) -> Client {
    let shared_config = aws_config::load_from_env().await;

    let config = config::Builder::from(&shared_config)
        .region(Region::new(region.into()))
        .build();

    Client::from_conf(config)
}

pub async fn get_item<I>(
    client: &Client,
    table_name: impl Into<String>,
    key: impl dynomite::Item,
    expression: Expression,
    item: &mut I,
) -> anyhow::Result<bool>
where
    I: dynomite::Item,
{
    let output = client
        .get_item()
        .table_name(table_name)
        .set_key(Some(key.key().to_sdk()))
        .set_expression_attribute_names(expression.names().clone())
        .set_projection_expression(expression.projection().cloned())
        .send()
        .await?;

    match output.item {
        Some(i) => {
            *item = I::from_attrs(i.to_rusoto())?;
            Ok(true)
        }
        None => Ok(false),
    }
}

pub async fn get_items<I>(
    client: &Client,
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
        let mut builder = KeysAndAttributes::builder()
            .set_expression_attribute_names(keys_attrs.1.names().clone())
            .set_projection_expression(keys_attrs.1.projection().cloned());

        for key in keys_attrs.0 {
            builder = builder.keys(key.key().to_sdk());
        }

        req_items.insert(table_name, builder.build());
    }

    let mut request_items = req_items;
    loop {
        let output = client
            .batch_get_item()
            .set_request_items(Some(request_items))
            .send()
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
                            *item = I::from_attrs(i.to_rusoto())?;
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
    I: dynomite::Item,
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
        let items = output.items.unwrap_or_else(Vec::new);
        for i in &items {
            let (_, stop) = item_cb(
                i,
                Box::new({
                    let i = i.clone();
                    |item: &mut I| {
                        *item = I::from_attrs(i.to_rusoto())?;
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
    I: dynomite::Item,
{
    do_query(client, table_name, expression, limit, None, true, item_cb).await
}

pub async fn query_index<I>(
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
    I: dynomite::Item,
{
    do_query(client, table_name, expression, limit, None, false, item_cb).await
}

pub async fn query_index_descending<I>(
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
    I: dynomite::Item,
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
        let items = output.items.unwrap_or_else(Vec::new);
        for i in &items {
            let (_, stop) = item_cb(
                i,
                Box::new({
                    let i = i.clone();
                    |item: &mut I| {
                        *item = I::from_attrs(i.to_rusoto())?;
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
    I: dynomite::Item,
{
    do_scan(client, table_name, expression, limit, None, item_cb).await
}

pub async fn scan_index<I>(
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
