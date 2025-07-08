use std::borrow::Cow;

use crate::cli::QueryOptions;

use super::ProxyList;

pub fn get_proxies(options: QueryOptions) -> ProxyList<'static> {
    from_dbus(options)
}

fn from_dbus(options: QueryOptions) -> ProxyList<'static> {
    let conn = zbus::blocking::Connection::session().expect("Failed to connect to dbus");
    let mconn = MConnection(conn);

    let mut no_proxies = Vec::new();
    for addr in options.no_query_addrs {
        if mconn.get_first_proxy(addr.clone()).is_empty() {
            no_proxies.push(addr);
        }
    }

    ProxyList {
        http: Cow::Owned(mconn.get_first_proxy(options.http_query_addr)),
        https: Cow::Owned(mconn.get_first_proxy(options.https_query_addr)),
        ftp: Cow::Owned(mconn.get_first_proxy(options.ftp_query_addr)),
        all: Cow::Owned(mconn.get_first_proxy(options.all_query_addr)),
        no: Cow::Owned(no_proxies.join(",")),
    }
}

struct MConnection(zbus::blocking::Connection);
impl MConnection {
    /// https://docs.flatpak.org/en/latest/portal-api-reference.html#gdbus-org.freedesktop.portal.ProxyResolver
    fn get_first_proxy(&self, addr: String) -> String {
        let proxies = self
            .0
            .call_method(
                Some("org.freedesktop.portal.Desktop"),
                "/org/freedesktop/portal/desktop",
                Some("org.freedesktop.portal.ProxyResolver"),
                "Lookup",
                &(addr,),
            )
            .expect("Failed to call method");
        let proxy = proxies
            .body()
            .deserialize::<Vec<String>>()
            .expect("Failed to get body")
            .first()
            .cloned()
            .unwrap_or_default();
        match proxy == "direct://" {
            true => String::new(),
            false => proxy,
        }
    }
}
