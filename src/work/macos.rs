use crate::utils::Terminal;
use super::ProxyList;
use std::process::Command;

pub fn get_proxies(terminal: Terminal) -> ProxyList {
    let mut proxies = ProxyList::default();
    
    // Get HTTP proxy
    if let Some(http_proxy) = get_proxy_for_protocol("http") {
        proxies.http = http_proxy;
    }
    
    // Get HTTPS proxy
    if let Some(https_proxy) = get_proxy_for_protocol("https") {
        proxies.https = https_proxy;
    }
    
    // Get FTP proxy
    if let Some(ftp_proxy) = get_proxy_for_protocol("ftp") {
        proxies.ftp = ftp_proxy;
    }
    
    // Get SOCKS proxy (for all_proxy)
    if let Some(socks_proxy) = get_proxy_for_protocol("socks") {
        proxies.all = socks_proxy;
    }
    
    // Get proxy bypass list
    if let Some(no_proxy) = get_proxy_bypass_list() {
        proxies.no = no_proxy;
    }
    
    proxies
}

fn get_proxy_for_protocol(protocol: &str) -> Option<String> {
    let output = Command::new("scutil")
        .arg("--proxy")
        .output()
        .ok()?;
    
    let proxy_info = String::from_utf8(output.stdout).ok()?;
    
    match protocol {
        "http" => {
            if proxy_info.contains("HTTPEnable : 1") {
                let host = extract_value(&proxy_info, "HTTPProxy")?;
                let port = extract_value(&proxy_info, "HTTPPort")?;
                Some(format!("http://{}:{}", host, port))
            } else {
                None
            }
        }
        "https" => {
            if proxy_info.contains("HTTPSEnable : 1") {
                let host = extract_value(&proxy_info, "HTTPSProxy")?;
                let port = extract_value(&proxy_info, "HTTPSPort")?;
                Some(format!("https://{}:{}", host, port))
            } else {
                None
            }
        }
        "ftp" => {
            if proxy_info.contains("FTPEnable : 1") {
                let host = extract_value(&proxy_info, "FTPProxy")?;
                let port = extract_value(&proxy_info, "FTPPort")?;
                Some(format!("ftp://{}:{}", host, port))
            } else {
                None
            }
        }
        "socks" => {
            if proxy_info.contains("SOCKSEnable : 1") {
                let host = extract_value(&proxy_info, "SOCKSProxy")?;
                let port = extract_value(&proxy_info, "SOCKSPort")?;
                Some(format!("socks://{}:{}", host, port))
            } else {
                None
            }
        }
        _ => None,
    }
}

fn get_proxy_bypass_list() -> Option<String> {
    let output = Command::new("scutil")
        .arg("--proxy")
        .output()
        .ok()?;
    
    let proxy_info = String::from_utf8(output.stdout).ok()?;
    
    // Look for ProxyAutoConfigEnable or ExceptionsList
    if let Some(exceptions) = extract_array_value(&proxy_info, "ExceptionsList") {
        Some(exceptions.join(","))
    } else {
        None
    }
}

fn extract_value(text: &str, key: &str) -> Option<String> {
    let pattern = format!("{} : ", key);
    let start = text.find(&pattern)? + pattern.len();
    let end = text[start..].find('\n').unwrap_or(text.len() - start) + start;
    let value = text[start..end].trim();
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

fn extract_array_value(text: &str, key: &str) -> Option<Vec<String>> {
    let pattern = format!("{} : {{", key);
    let start = text.find(&pattern)?;
    let array_start = start + pattern.len();
    
    // Find the closing brace
    let mut brace_count = 1;
    let mut end = array_start;
    let chars: Vec<char> = text[array_start..].chars().collect();
    
    for (i, &ch) in chars.iter().enumerate() {
        match ch {
            '{' => brace_count += 1,
            '}' => {
                brace_count -= 1;
                if brace_count == 0 {
                    end = array_start + i;
                    break;
                }
            }
            _ => {}
        }
    }
    
    let array_content = &text[array_start..end];
    let mut items = Vec::new();
    
    for line in array_content.lines() {
        let line = line.trim();
        if line.starts_with('"') && line.ends_with('"') {
            let item = &line[1..line.len()-1];
            items.push(item.to_string());
        }
    }
    
    if items.is_empty() {
        None
    } else {
        Some(items)
    }
}
