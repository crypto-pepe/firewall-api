use std::fmt::{Debug, Display, Error, Formatter};

use serde::{Deserialize, Serialize};

use crate::http_error::BanTargetConversionError;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BanTarget {
    pub ip: Option<String>,
    pub user_agent: Option<String>,
}

impl Display for BanTarget {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.user_agent.is_none() && self.ip.is_none() {
            return Err(Error);
        }
        if self.user_agent.is_none() {
            return f.write_str(&*format!("ip:{}", &*self.ip.as_ref().unwrap()));
        }
        if self.ip.is_none() {
            return f.write_str(&*format!(
                "user-agent:{}",
                &*self.user_agent.as_ref().unwrap()
            ));
        }
        f.write_str(&*format!(
            "ip:{}_user-agent:{}",
            &*self.ip.as_ref().unwrap(),
            self.user_agent.as_ref().unwrap()
        ))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BanTargetRequest {
    pub target: BanTarget,
}

impl BanTargetRequest {
    pub fn verify(&self) -> Result<(), BanTargetConversionError> {
        if self.target.ip.is_none() && self.target.user_agent.is_none() {
            return Err(BanTargetConversionError::FieldRequired);
        }
        Ok(())
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
            want: "ip:1.1.1.1_user-agent:abc".into(),
        };

        assert_eq!(tc.input.to_string(), tc.want);
    }
}
