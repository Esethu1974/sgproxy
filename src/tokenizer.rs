use std::sync::OnceLock;

use anyhow::{Result, anyhow};
use serde_json::{Value, json};
use tiktoken_rs::{CoreBPE, cl100k_base, o200k_base};

pub fn count_openai_response_input_tokens(body: &Value) -> Result<Value> {
    let model = body.get("model").and_then(Value::as_str);
    let bpe = tokenizer_for_model(model)?;
    let mut total = 0i64;

    if let Some(instructions) = body.get("instructions").and_then(Value::as_str) {
        total += count_text(instructions, bpe);
    }
    if let Some(input) = body.get("input") {
        total += count_input_value(input, bpe);
    }

    Ok(json!({
        "object": "response.input_tokens",
        "input_tokens": total,
    }))
}

fn tokenizer_for_model(model: Option<&str>) -> Result<&'static CoreBPE> {
    let model = model.unwrap_or("").trim();
    if is_o200k_model(model) {
        if let Some(bpe) = O200K.get() {
            return Ok(bpe);
        }
        let bpe = o200k_base().map_err(|err| anyhow!("failed to init o200k tokenizer: {err}"))?;
        let _ = O200K.set(bpe);
        return O200K
            .get()
            .ok_or_else(|| anyhow!("failed to cache o200k tokenizer"));
    }
    if let Some(bpe) = CL100K.get() {
        return Ok(bpe);
    }
    let bpe = cl100k_base().map_err(|err| anyhow!("failed to init cl100k tokenizer: {err}"))?;
    let _ = CL100K.set(bpe);
    CL100K
        .get()
        .ok_or_else(|| anyhow!("failed to cache cl100k tokenizer"))
}

fn is_o200k_model(model: &str) -> bool {
    model.starts_with("gpt-5")
        || model.starts_with("gpt-4.1")
        || model.starts_with("gpt-4o")
        || model.starts_with("o1")
        || model.starts_with("o3")
        || model.starts_with("o4")
}

fn count_input_value(value: &Value, bpe: &CoreBPE) -> i64 {
    match value {
        Value::String(text) => count_text(text, bpe),
        Value::Array(items) => items.iter().map(|item| count_input_item(item, bpe)).sum(),
        _ => 0,
    }
}

fn count_input_item(item: &Value, bpe: &CoreBPE) -> i64 {
    if let Some(content) = item.get("content") {
        return count_content_value(content, bpe);
    }
    if let Some(output) = item.get("output") {
        return count_output_value(output, bpe);
    }
    0
}

fn count_content_value(value: &Value, bpe: &CoreBPE) -> i64 {
    match value {
        Value::String(text) => count_text(text, bpe),
        Value::Array(items) => items.iter().map(|item| count_content_part(item, bpe)).sum(),
        _ => 0,
    }
}

fn count_content_part(part: &Value, bpe: &CoreBPE) -> i64 {
    part.get("text")
        .and_then(Value::as_str)
        .map(|text| count_text(text, bpe))
        .or_else(|| {
            part.get("refusal")
                .and_then(Value::as_str)
                .map(|text| count_text(text, bpe))
        })
        .unwrap_or(0)
}

fn count_output_value(value: &Value, bpe: &CoreBPE) -> i64 {
    match value {
        Value::String(text) => count_text(text, bpe),
        Value::Array(items) => items.iter().map(|item| count_content_part(item, bpe)).sum(),
        _ => 0,
    }
}

fn count_text(text: &str, bpe: &CoreBPE) -> i64 {
    bpe.encode_ordinary(text).len() as i64
}

static CL100K: OnceLock<CoreBPE> = OnceLock::new();
static O200K: OnceLock<CoreBPE> = OnceLock::new();
