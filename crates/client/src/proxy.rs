//! Proxied connections for the collaboration WebSocket.

use anyhow::Result;
use http_client::Url;

pub(crate) trait AsyncReadWrite:
    tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send + 'static
{
}
impl<T: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send + 'static> AsyncReadWrite
    for T
{
}

/// Whether `NO_PROXY` in the environment excludes `host` from proxying,
/// matching the exclusions the HTTP client already applies to its own
/// requests.
pub(crate) fn excluded_from_proxy(host: &str) -> bool {
    http_client::read_no_proxy_from_env().is_some_and(|no_proxy| no_proxy_matches(&no_proxy, host))
}

pub fn no_proxy_matches(no_proxy: &str, host: &str) -> bool {
    no_proxy
        .split(',')
        .map(str::trim)
        .filter(|entry| !entry.is_empty())
        .any(|entry| {
            if entry == "*" {
                return true;
            }
            // Bare IPv6 literals also contain colons, so only strip a port
            // when there is exactly one colon and the suffix parses as one.
            let entry = match entry.rsplit_once(':') {
                Some((prefix, suffix))
                    if !prefix.contains(':') && suffix.parse::<u16>().is_ok() =>
                {
                    prefix
                }
                _ => entry,
            };
            let entry = entry.strip_prefix("*.").unwrap_or(entry);
            let entry = entry.strip_prefix('.').unwrap_or(entry);
            host.eq_ignore_ascii_case(entry)
                || (host.len() > entry.len()
                    && host.as_bytes()[host.len() - entry.len() - 1] == b'.'
                    && host[host.len() - entry.len()..].eq_ignore_ascii_case(entry))
        })
}

pub(crate) async fn connect_proxy_stream(
    _proxy: &Url,
    _rpc_host: (&str, u16),
) -> Result<Box<dyn AsyncReadWrite>> {
    anyhow::bail!("proxy handshake is disabled");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_proxy_matches() {
        assert!(no_proxy_matches("example.com", "example.com"));
        assert!(no_proxy_matches("example.com", "sub.example.com"));
        assert!(no_proxy_matches(".example.com", "sub.example.com"));
        assert!(no_proxy_matches("*.example.com", "sub.example.com"));
        assert!(no_proxy_matches("other.org, example.com", "example.com"));
        assert!(no_proxy_matches("example.com:443", "example.com"));
        assert!(no_proxy_matches("*", "anything.at.all"));
        assert!(no_proxy_matches("EXAMPLE.com", "example.COM"));
        assert!(no_proxy_matches("::1", "::1"));

        assert!(!no_proxy_matches("example.com", "notexample.com"));
        assert!(!no_proxy_matches("example.com", "example.org"));
        assert!(!no_proxy_matches("", "example.com"));
        assert!(!no_proxy_matches("sub.example.com", "example.com"));
    }
}
