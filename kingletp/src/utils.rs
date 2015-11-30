use url::{ParseResult, Url};

pub fn url_from_http(scheme: &str, authority: &str, path: &str) -> ParseResult<Url> {
    Url::parse(&format!("{}://{}{}", scheme, authority, path)[..])
}
