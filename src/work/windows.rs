use std::borrow::Cow;
use std::collections::BTreeSet;

use colored::Colorize;

use super::ProxyList;

const PROXY_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Internet Settings";

pub fn get_proxies(force_socks5h: bool) -> ProxyList<'static> {
    from_registry(force_socks5h)
}

fn from_registry(force_socks5h: bool) -> ProxyList<'static> {
    // Get system proxy
    let hkcu = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
    let settings = hkcu.open_subkey(PROXY_KEY).unwrap();
    let proxy_enabled: u32 = settings.get_value("ProxyEnable").unwrap_or_default();
    let proxy_server: String = settings.get_value("ProxyServer").unwrap_or_default();
    let proxy_override: String = settings.get_value("ProxyOverride").unwrap_or_default();
    if !proxy_enabled.eq(&1) {
        return Default::default();
    }

    let mut proxy_list = parse_proxy_server(&proxy_server, force_socks5h);
    let no_proxy = parse_no_proxy(&proxy_override);

    proxy_list.no = Cow::Owned(no_proxy);
    proxy_list
}

fn parse_proxy_server(proxy_server: &str, force_socks5h: bool) -> ProxyList<'static> {
    let mut http = String::new();
    let mut https = String::new();
    let mut ftp = String::new();
    let mut all = String::new();

    for proxy in proxy_server.split(";") {
        match proxy.split("=").collect::<Vec<&str>>().as_slice() {
            ["http", value] => http = value.to_string(),
            ["https", value] => https = value.to_string(),
            ["ftp", value] => ftp = value.to_string(),
            ["socks", value] => {
                all = if value.starts_with("socks") {
                    value.to_string()
                } else {
                    format!("socks://{}", value)
                }
            }
            [value] => match value.split("://").collect::<Vec<_>>().as_slice() {
                [protocol, _] => match *protocol {
                    "http" => http = value.to_string(),
                    "https" => https = value.to_string(),
                    "socks" => all = value.to_string(),
                    _ => eprintln!("Invalid proxy protocol: {}", protocol),
                },
                [_] => {
                    if !value.is_empty() {
                        all = add_protocol_prefix(value, force_socks5h);
                    }
                }
                _ => eprintln!("Invalid proxy definition: {}", value),
            },
            _ => eprintln!("Invalid proxy definition: {}", proxy),
        }
    }

    ProxyList {
        http: Cow::Owned(http),
        https: Cow::Owned(https),
        ftp: Cow::Owned(ftp),
        all: Cow::Owned(all),
        no: Cow::Borrowed(""),
    }
}

fn add_protocol_prefix(value: &str, force_socks5h: bool) -> String {
    // Check if it's a bare IP address (no protocol prefix)
    if is_bare_ip_address(value) {
        if force_socks5h {
            format!("socks5h://{}", value)
        } else {
            format!("http://{}", value)
        }
    } else {
        value.to_string()
    }
}

fn is_bare_ip_address(value: &str) -> bool {
    // Check if it matches IP:port pattern without protocol
    if value.contains("://") {
        return false;
    }

    // Split by colon to get potential IP and port
    let parts: Vec<&str> = value.split(':').collect();
    if parts.len() != 2 {
        return false;
    }

    let ip_part = parts[0];
    let port_part = parts[1];

    // Check if port is a valid number
    if port_part.parse::<u16>().is_err() {
        return false;
    }

    // Check if IP part looks like an IPv4 address
    let ip_parts: Vec<&str> = ip_part.split('.').collect();
    if ip_parts.len() != 4 {
        return false;
    }

    for part in ip_parts {
        if part.parse::<u8>().is_err() {
            return false;
        }
    }

    true
}

fn parse_no_proxy(proxy_override: &str) -> String {
    let mut no_proxy = BTreeSet::new();

    let proxy_override = proxy_override.split(";").collect::<Vec<_>>();
    for it in proxy_override {
        let mut it = it.to_string();
        if it == "*" {
        } else if it.starts_with("<local>") {
            it = "localhost".to_string();
        } else {
            // Change wildcard to subnet
            if let Some(subnet) = eval_ipv4(&it) {
                while it.ends_with('*') || it.ends_with('.') {
                    it.pop();
                }
                match subnet {
                    32 => it.push_str("0.0.0.0/32"),
                    24 => it.push_str(".0.0.0/24"),
                    16 => it.push_str(".0.0/16"),
                    8 => it.push_str(".0/8"),
                    0 => {}
                    _ => unreachable!("Invalid subnet: {}", subnet),
                }
            } else {
                // no_proxy variable does not support wildcard
                let mut tmp = it.as_str();
                while tmp.starts_with('*') || tmp.starts_with('.') {
                    tmp = &tmp[1..];
                }
                if tmp.find('*').is_some() {
                    eprintln!(
                        "{}: Unsupported wildcard in the middle of the address: {}",
                        "Warning".red(),
                        tmp.yellow()
                    );
                }
                it = tmp.to_string();
            }
        }
        no_proxy.insert(it);
    }

    no_proxy.into_iter().collect::<Vec<_>>().join(",")
}

fn eval_ipv4(addr: &str) -> Option<u8> {
    let parts = addr.split('.').collect::<Vec<_>>();
    if parts.len() > 4 {
        return None;
    }
    let mut subnet = 0;
    let mut flag = false;
    for (idx, part) in parts.into_iter().enumerate() {
        if part.is_empty() {
            return None;
        }
        match part.parse::<u8>() {
            Ok(_) => {
                if flag {
                    return None;
                }
            }
            Err(_) => {
                if part == "*" {
                    if !flag {
                        subnet = (4 - idx) * 8;
                    }
                    flag = true;
                } else {
                    return None;
                }
            }
        }
    }
    Some(subnet as u8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_proxy_server() {
        let proxy_server = "";
        assert_eq!(
            parse_proxy_server(proxy_server, false),
            ProxyList::default()
        );

        let proxy_server = "127.0.0.1:8080";
        assert_eq!(
            parse_proxy_server(proxy_server, false),
            ProxyList {
                http: Cow::Borrowed(""),
                https: Cow::Borrowed(""),
                ftp: Cow::Borrowed(""),
                all: Cow::Owned("http://127.0.0.1:8080".to_string()),
                no: Cow::Borrowed("")
            }
        );

        let proxy_server = "127.0.0.1:7890";
        assert_eq!(
            parse_proxy_server(proxy_server, true),
            ProxyList {
                http: Cow::Borrowed(""),
                https: Cow::Borrowed(""),
                ftp: Cow::Borrowed(""),
                all: Cow::Owned("socks5h://127.0.0.1:7890".to_string()),
                no: Cow::Borrowed("")
            }
        );

        let proxy_server =
            "http=127.0.0.1:7890;https=127.0.0.1:7890;ftp=127.0.0.1:7890;socks=127.0.0.1:7890";
        assert_eq!(
            parse_proxy_server(proxy_server, false),
            ProxyList {
                http: Cow::Owned("127.0.0.1:7890".to_string()),
                https: Cow::Owned("127.0.0.1:7890".to_string()),
                ftp: Cow::Owned("127.0.0.1:7890".to_string()),
                all: Cow::Owned("socks://127.0.0.1:7890".to_string()),
                no: Cow::Borrowed("")
            }
        );
    }

    #[test]
    fn test_parse_no_proxy() {
        let proxy_override = "";
        assert_eq!(parse_no_proxy(proxy_override), "");

        let proxy_override = "<local>";
        assert_eq!(parse_no_proxy(proxy_override), "localhost");

        let proxy_override = "localhost;10.0.*.*";
        assert_eq!(
            parse_no_proxy(proxy_override),
            "10.0.0.0/16,localhost".to_string()
        );

        let proxy_override = "10.0.*";
        assert_eq!(parse_no_proxy(proxy_override), "10.0.0.0/16");

        let proxy_override = "*.google.com";
        assert_eq!(parse_no_proxy(proxy_override), "google.com");

        let proxy_override = "*.google.com;*.baidu.com";
        assert_eq!(parse_no_proxy(proxy_override), "baidu.com,google.com");
    }

    #[test]
    fn test_is_bare_ip_address() {
        assert!(is_bare_ip_address("127.0.0.1:8080"));
        assert!(is_bare_ip_address("192.168.1.1:3128"));
        assert!(!is_bare_ip_address("http://127.0.0.1:8080"));
        assert!(!is_bare_ip_address("socks://127.0.0.1:1080"));
        assert!(!is_bare_ip_address("proxy.example.com:8080"));
        assert!(!is_bare_ip_address("127.0.0.1"));
        assert!(!is_bare_ip_address("127.0.0.1:abc"));
    }

    #[test]
    fn test_add_protocol_prefix() {
        assert_eq!(
            add_protocol_prefix("127.0.0.1:7890", false),
            "http://127.0.0.1:7890"
        );
        assert_eq!(
            add_protocol_prefix("127.0.0.1:7890", true),
            "socks5h://127.0.0.1:7890"
        );
        assert_eq!(
            add_protocol_prefix("http://127.0.0.1:7890", false),
            "http://127.0.0.1:7890"
        );
        assert_eq!(
            add_protocol_prefix("proxy.example.com:8080", false),
            "proxy.example.com:8080"
        );
    }
}
