use std::{collections::HashMap, hash::BuildHasher};
use tera::{Error, Value, to_value};

pub fn to_ogham<S: BuildHasher>(
    value: &Value,
    _args: &HashMap<String, Value, S>,
) -> Result<Value, Error> {
    let start = "&#x169b;"; // >-

    let working_value = value.to_string();
    let new_value = working_value
        .replace("ng", "&#x168d;")
        .replace("st", "&#x168e;")
        .replace("b", "&#x1681;")
        .replace("l", "&#x1682;")
        .replace("f", "&#x1683;")
        .replace("s", "&#x1684;")
        .replace("n", "&#x1685;")
        .replace("h", "&#x1686;")
        .replace("d", "&#x1687;")
        .replace("t", "&#x1688;")
        .replace("c", "&#x1689;")
        .replace("q", "&#x168a;")
        .replace("m", "&#x168b;")
        .replace("g", "&#x168c;")
        .replace("r", "&#x168f;")
        .replace("a", "&#x1690;")
        .replace("o", "&#x1691;")
        .replace("u", "&#x1692;")
        .replace("e", "&#x1693;")
        .replace("i", "&#x1694;")
        .replace("p", "&#x169a;")
        .replace(" ", "&#x1680;");

    let out_value = format!("{}{}", start, new_value.replace("\"", ""));
    to_value(out_value).map_err(|e| Error::msg(e.to_string()))
}
