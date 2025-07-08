use crate::{
    cli::{OverrideOptions, QueryOptions},
    utils::{Terminal, Vars},
};

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

#[derive(Default, Debug, PartialEq, Eq)]
struct ProxyList {
    http: String,
    https: String,
    ftp: String,
    all: String,
    no: String,
}

impl ProxyList {
    fn into_vars(self) -> Vars {
        let mut vars = Vars::default();
        if !self.http.is_empty() {
            vars.push(("http_proxy", self.http));
        }
        if !self.https.is_empty() {
            vars.push(("https_proxy", self.https));
        }
        if !self.ftp.is_empty() {
            vars.push(("ftp_proxy", self.ftp));
        }
        if !self.all.is_empty() {
            vars.push(("all_proxy", self.all));
        }
        if !self.no.is_empty() {
            vars.push(("no_proxy", self.no));
        }
        vars
    }
}

#[allow(unused_variables)]
fn get_proxies(terminal: Terminal, query_options: QueryOptions) -> ProxyList {
    #[cfg(target_os = "linux")]
    return linux::get_proxies(query_options);

    #[cfg(target_os = "macos")]
    return macos::get_proxies(terminal);

    #[cfg(target_os = "windows")]
    return windows::get_proxies();

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    compile_error!("Unsupported OS");
}

pub fn init(
    terminal: Terminal,
    query_options: QueryOptions,
    override_options: OverrideOptions,
) -> String {
    let mut proxies = ProxyList::default();
    if !override_options.no_detect {
        proxies = get_proxies(terminal, query_options);
    }

    if let Some(http_proxy) = override_options.http_proxy {
        proxies.http = http_proxy;
    }
    if let Some(https_proxy) = override_options.https_proxy {
        proxies.https = https_proxy;
    }
    if let Some(ftp_proxy) = override_options.ftp_proxy {
        proxies.ftp = ftp_proxy;
    }
    if let Some(all_proxy) = override_options.all_proxy {
        proxies.all = all_proxy;
    }
    if let Some(no_proxy) = override_options.no_proxy {
        proxies.no = no_proxy;
    }

    terminal.set_envs_str(proxies.into_vars())
}

pub fn cat() -> String {
    const KEYS: [&str; 5] = [
        "http_proxy",
        "https_proxy",
        "ftp_proxy",
        "all_proxy",
        "no_proxy",
    ];

    let mut res = String::new();

    for key in KEYS {
        if let Ok(val) = std::env::var(key) {
            res.push_str(&format!("{key}: {val}\n"));
        }
    }

    // On Unix-like systems, the environment variables are case-sensitive.
    #[cfg(unix)]
    {
        const CAP_KEYS: [&str; 5] = [
            "HTTP_PROXY",
            "HTTPS_PROXY",
            "FTP_PROXY",
            "ALL_PROXY",
            "NO_PROXY",
        ];
        for key in CAP_KEYS {
            if let Ok(val) = std::env::var(key) {
                res.push_str(&format!("{key}: {val}\n"));
            }
        }
    }

    res
}
