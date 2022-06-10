use std::fmt::{Debug, Display, Error, Formatter};

use crate::error::BanTargetConversionError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum UnbanEntity {
    Target(BanTarget),
    Pattern(String),
}

impl UnbanEntity {
    pub fn verify(&self) -> Result<(), BanTargetConversionError> {
        match self {
            UnbanEntity::Target(bt) => {
                if bt.ip.is_none() && bt.user_agent.is_none() {
                    Err(BanTargetConversionError::EmptyRequest)
                } else {
                    Ok(())
                }
            }
            UnbanEntity::Pattern(p) => {
                if p.ne("*") {
                    Err(BanTargetConversionError::PatternUnsupported)
                } else {
                    Ok(())
                }
            }
        }
    }
}

impl Display for UnbanEntity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*serde_json::to_string(self).map_err(|_| Error)?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BanTarget {
    pub ip: Option<String>,
    pub user_agent: Option<String>,
}

const SEPARATOR: &str = "__";

impl Display for BanTarget {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut ss = vec![];

        if self.user_agent.is_none() && self.ip.is_none() {
            return Err(Error);
        }

        if let Some(ip) = &self.ip {
            ss.push(format!("ip:{}", ip));
        }

        if let Some(user_agent) = &self.user_agent {
            ss.push(format!("user_agent:{}", user_agent));
        }

        f.write_str(&ss.join(SEPARATOR))
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
            want: "user_agent:abc".into(),
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
            want: "ip:1.1.1.1__user_agent:abc".into(),
        };

        assert_eq!(tc.input.to_string(), tc.want);
    }
}
