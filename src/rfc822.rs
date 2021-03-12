use std::collections::HashMap;

use crate::error::{Error, Result};

pub fn fmt_rfc822(rfc822_hmap: HashMap<String, String>) -> String
{
    let mut lines: Vec<String> = Vec::new();
    for (key, value) in rfc822_hmap
    {
        lines.push(format!("{}: {}", key, value));
    }

    lines.join("\n")
}

pub fn parse_rfc822_buffer(buffer: &str) -> Result<HashMap<String, String>>
{
    let lines = buffer.split("\n");
    
    let mut rfc822_dict: HashMap<String, String> = HashMap::new();
    let mut last_key_opt: Option<&str> = None;
    for line in lines
    {
        // RFC822 - Simple
        let sep_pos_opt = line.find(":");
        match sep_pos_opt
        {
            Some(sep_pos) => {
                let (key, mut value) = line.split_at(sep_pos);
                value = &value[1..value.len()]; // Lose the colon
                value = value.trim(); // Leading and trailing Whitespace is unimportant
                rfc822_dict.insert(key.to_string(), value.to_string());
                last_key_opt = Some(key);
            },
            None => {
                // Could be folded line continue.
            }
        }
        
        // RFC822 - Folded
        if line.starts_with(" ") || line.starts_with("\t")
        {
            match last_key_opt
            {
                Some(last_key) => {
                    let last_value = rfc822_dict.get_mut(last_key).unwrap();
                    last_value.push_str(line);
                },
                None => {
                    // Error!
                    return Err(Error::InvalidFoldedValue);
                }
            }
        }
    }

    // println!("{:?}", rfc822_dict);

    Ok(rfc822_dict)
}