use std::fmt::{Debug, Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BanTarget {
    pub ip: String,
    pub user_agent: Option<String>,
}

impl Display for BanTarget {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.user_agent.is_none() {
            f.write_str(&*self.ip)
        } else {
            f.write_str(&*format!(
                "{}_{}",
                &*self.ip,
                self.user_agent.as_ref().unwrap()
            ))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BanTargetRequest {
    pub target: BanTarget,
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
                ip: "1.1.1.1".into(),
                user_agent: None,
            },
            want: "1.1.1.1".into(),
        };

        assert_eq!(tc.input.to_string(), tc.want);
    }

    #[test]
    fn target_to_key_ip_and_user_agent() {
        let tc = TestCase {
            input: BanTarget {
                ip: "1.1.1.1".into(),
                user_agent: Some("abc".into()),
            },
            want: "1.1.1.1_abc".into(),
        };

        assert_eq!(tc.input.to_string(), tc.want);
    }
}
