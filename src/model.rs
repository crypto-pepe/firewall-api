use std::fmt::{Debug, Display, Error, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BanTarget {
    pub ip: Option<String>,
    pub user_agent: Option<String>,
}

const SEPARATOR: &str = "__";

impl Display for BanTarget {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut vv: Vec<String> = Vec::new();

        if self.user_agent.is_none() && self.ip.is_none() {
            return Err(Error);
        }
        if let Some(ip) = self.ip.as_ref() {
            vv.push(format!("ip:{}", ip));
        }
        if let Some(ua) = self.user_agent.as_ref() {
            vv.push(format!("user-agent:{}", ua));
        }

        f.write_str(&*vv.join(SEPARATOR))
    }
}

#[cfg(test)]
mod tests {
    use crate::model::BanTarget;

    struct TestCase {
        pub input: BanTarget,
        pub want: String,
    }

    #[test]
    fn target_to_key_ip() {
        let tc = TestCase {
            input: BanTarget {
                ip: Some("1.1.1.1".into()),
                user_agent: None,
            },
            want: "ip:1.1.1.1".into(),
        };

        assert_eq!(tc.input.to_string(), tc.want);
    }

    #[test]
    fn target_to_key_user_agent() {
        let tc = TestCase {
            input: BanTarget {
                ip: None,
                user_agent: Some("abc".into()),
            },
            want: "user-agent:abc".into(),
        };

        assert_eq!(tc.input.to_string(), tc.want);
    }

    #[test]
    fn target_to_key_ip_and_user_agent() {
        let tc = TestCase {
            input: BanTarget {
                ip: Some("1.1.1.1".into()),
                user_agent: Some("abc".into()),
            },
            want: "ip:1.1.1.1__user-agent:abc".into(),
        };

        assert_eq!(tc.input.to_string(), tc.want);
    }
}
