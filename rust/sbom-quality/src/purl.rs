//! Package URLs, parsed with the purl crate that mergebom validates them with as well.

use std::{collections::BTreeMap, str::FromStr};

use purl::GenericPurl;

#[derive(Debug, PartialEq, Eq)]
pub struct Purl {
    pub package_type: String,

    /// The name including the namespace, for example `redhat/glibc`.
    pub name: String,

    pub version: Option<String>,

    pub qualifiers: BTreeMap<String, String>,
}

impl Purl {
    /// Parses a purl, or returns `None` for a string that does not conform to the Package URL
    /// specification. Type-specific rules are not applied, like in mergebom.
    pub fn parse(purl: &str) -> Option<Self> {
        let purl = GenericPurl::<String>::from_str(purl).ok()?;
        let name = match purl.namespace() {
            Some(namespace) => format!("{namespace}/{name}", name = purl.name()),
            None => purl.name().to_owned(),
        };

        Some(Self {
            package_type: purl.package_type().to_lowercase(),
            name,
            version: purl
                .version()
                .filter(|version| !version.is_empty())
                .map(ToOwned::to_owned),
            qualifiers: purl
                .qualifiers()
                .iter()
                .map(|(key, value)| (key.as_ref().to_owned(), value.to_owned()))
                .collect(),
        })
    }

    /// The last segment of the name, without the namespace.
    pub fn short_name(&self) -> &str {
        self.name
            .rsplit_once('/')
            .map_or(self.name.as_str(), |(_, name)| name)
    }
}

/// Decodes `%XX` escapes. Invalid escapes are kept as they are.
pub fn percent_decode(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while let Some(&byte) = bytes.get(index) {
        let escaped = bytes
            .get(index + 1..index + 3)
            .filter(|hex| byte == b'%' && hex.iter().all(u8::is_ascii_hexdigit))
            .and_then(|hex| std::str::from_utf8(hex).ok())
            .and_then(|hex| u8::from_str_radix(hex, 16).ok());
        match escaped {
            Some(escaped) => {
                decoded.push(escaped);
                index += 3;
            }
            None => {
                decoded.push(byte);
                index += 1;
            }
        }
    }
    String::from_utf8_lossy(&decoded).into_owned()
}

/// Escapes everything except unreserved characters, as purl qualifier values need it.
pub fn percent_encode(input: &str) -> String {
    input
        .bytes()
        .map(|byte| {
            if byte.is_ascii_alphanumeric() || b"-._~".contains(&byte) {
                char::from(byte).to_string()
            } else {
                format!("%{byte:02X}")
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn parse_purl_with_qualifiers() {
        let purl = Purl::parse("pkg:rpm/redhat/glibc@2.34?arch=x86_64&distro=rhel-9.9")
            .expect("the purl should be valid");

        assert_eq!(
            purl,
            Purl {
                package_type: "rpm".to_owned(),
                name: "redhat/glibc".to_owned(),
                version: Some("2.34".to_owned()),
                qualifiers: BTreeMap::from([
                    ("arch".to_owned(), "x86_64".to_owned()),
                    ("distro".to_owned(), "rhel-9.9".to_owned()),
                ]),
            }
        );
        assert_eq!(purl.short_name(), "glibc");
    }

    #[rstest]
    #[case::with_version("pkg:maven/org.example/lib@1.0", Some("1.0"))]
    #[case::without_version("pkg:golang/k8s.io/api", None)]
    #[case::empty_version("pkg:golang/k8s.io/api@", None)]
    #[case::encoded_version("pkg:golang/opa@v1.2%2Bdirty", Some("v1.2+dirty"))]
    fn parse_purl_version(#[case] purl: &str, #[case] version: Option<&str>) {
        let purl = Purl::parse(purl).expect("the purl should be valid");
        assert_eq!(purl.version.as_deref(), version);
    }

    #[rstest]
    #[case::empty("")]
    #[case::no_scheme("glibc@2.34")]
    #[case::no_name("pkg:rpm")]
    #[case::empty_name("pkg:rpm/@1")]
    fn parse_malformed_purl(#[case] purl: &str) {
        assert_eq!(Purl::parse(purl), None);
    }

    #[rstest]
    #[case("sha256%3Aab", "sha256:ab")]
    #[case("100%", "100%")]
    #[case("%zz", "%zz")]
    #[case("%2B%2b", "++")]
    fn decode_percent_escapes(#[case] input: &str, #[case] decoded: &str) {
        assert_eq!(percent_decode(input), decoded);
    }

    #[rstest]
    #[case("sha256:ab", "sha256%3Aab")]
    #[case("oci.stackable.tech/sdp/kafka", "oci.stackable.tech%2Fsdp%2Fkafka")]
    #[case("a-b_c.d~e", "a-b_c.d~e")]
    fn encode_percent_escapes(#[case] input: &str, #[case] encoded: &str) {
        assert_eq!(percent_encode(input), encoded);
    }
}
