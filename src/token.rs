use regex::Regex;
use reqwest::Client;
use serde::Deserialize;
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

// Function to perform bitwise operations similar to the JavaScript code
fn xr(a: i64, b: &str) -> i64 {
    let mut result = a;
    let b_bytes = b.as_bytes();

    for c in (0..b.len() - 2).step_by(3) {
        let d = if b_bytes[c + 2].is_ascii_lowercase() {
            b_bytes[c + 2] as i64 - 87
        } else {
            (b_bytes[c + 2] as char).to_digit(10).unwrap() as i64
        };

        let d = if b_bytes[c + 1] as char == '+' {
            result >> d
        } else {
            result << d
        };

        result = if b_bytes[c] as char == '+' {
            (result + d) & 4294967295
        } else {
            result ^ d
        };
    }
    result
}

// Function to calculate the translation token (tk)
fn sm(a: &str, token: &str) -> String {
    let mut b = token
        .split('.')
        .next()
        .unwrap_or("0")
        .parse::<i64>()
        .unwrap_or(0);
    let mut e = Vec::new();

    for g in a.chars() {
        let l = g as u32;

        if l < 128 {
            e.push(l);
        } else {
            if l < 2048 {
                e.push((l >> 6) | 192);
            } else {
                if (l & 64512) == 55296
                    && !a.is_empty()
                    && (a.chars().nth(1).unwrap() as u32 & 64512) == 56320
                {
                    let l =
                        65536 + (((l & 1023) << 10) + (a.chars().nth(1).unwrap() as u32 & 1023));
                    e.push((l >> 18) | 240);
                    e.push((l >> 12 & 63) | 128);
                } else {
                    e.push((l >> 12) | 224);
                    e.push((l >> 6 & 63) | 128);
                }
                e.push(l & 63 | 128);
            }
        }
    }

    for f in e {
        b += f as i64;
        b = xr(b, "+-a^+6");
    }

    b = xr(b, "+-3^+b+-f");
    b ^= token
        .split('.')
        .nth(1)
        .unwrap_or("0")
        .parse::<i64>()
        .unwrap_or(0);

    if b < 0 {
        b = (b & 2147483647) + 2147483648;
    }

    b %= 1e6 as i64;
    format!(
        "{}{}{}",
        "&tk=",
        b,
        ".".to_string()
            + &(b ^ token
                .split('.')
                .next()
                .unwrap_or("0")
                .parse::<i64>()
                .unwrap_or(0))
            .to_string()
    )
}

async fn update_token(client: &Client) -> Result<String, Box<dyn Error>> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() / 3600;
    let res = client
        .get("https://translate.google.com")
        .send()
        .await?
        .text()
        .await?;
    let re = Regex::new(r"TKK=(.*?)\(\)\)'")?;
    if let Some(caps) = re.captures(&res) {
        if let Some(tkk) = caps.get(1) {
            return Ok(tkk.as_str().to_string());
        }
    }
    Ok(format!("{}.0", now)) // Fallback if regex fails
}

pub async fn get(client: &Client, text: &str) -> Result<String, Box<dyn Error>> {
    let tkk = update_token(&client).await?;
    let tk = sm(text, &tkk);
    Ok(tk)
}

async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    let client = Client::new();

    let text = "Hello, world!";
    let tk = get(&client, text).await?;
    println!("Token: {}", tk);

    Ok(())
}
