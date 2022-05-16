use std::collections::BTreeMap;
use std::fmt::{Debug, Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::http_error::BanTargetConversionError;

const TARGET_TYPE_ORDER: &'static [TargetType] = &[TargetType::IP, TargetType::UserAgent];

#[derive(Debug, Serialize, Deserialize)]
pub enum TargetType {
    #[serde(rename = "ip")]
    IP,
    #[serde(rename = "user-agent")]
    UserAgent,
}

impl Display for TargetType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            TargetType::IP => "ip",
            TargetType::UserAgent => "user-agent",
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BanTarget {
    #[serde(rename = "type")]
    pub target_type: TargetType,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BanTargetRequest {
    pub target: Vec<BanTarget>,
}


pub fn target_to_key(bt: &Vec<BanTarget>) -> Result<String, BanTargetConversionError> {
    let mut bt_value = BTreeMap::new();

    for t in bt {
        if bt_value.get(&t.target_type.to_string()).is_none() {
            bt_value.insert(t.target_type.to_string(), t.value.to_string());
        } else {
            return Err(BanTargetConversionError::InvalidTypeCount);
        }
    }

    let target = TARGET_TYPE_ORDER
        .into_iter()
        .fold(String::new(), |res: String, t| {
            if let Some(v) = bt_value.get(&*t.to_string()) {
                format!("{}{}{}", res, t, v)
            } else {
                res
            }
        });

    if target.is_empty() {
        return Err(BanTargetConversionError::InvalidTypeCount);
    }
    Ok(target)
}


#[cfg(test)]
mod tests {
    use crate::http_error::BanTargetConversionError;
    use crate::model::{target_to_key, BanTarget, TargetType};

    struct TestCase {
        pub input: Vec<BanTarget>,
        pub want: Result<String, BanTargetConversionError>,
    }

    #[test]
    fn target_to_key_ip_only() {
        let tc = TestCase {
            input: vec![BanTarget {
                target_type: TargetType::IP,
                value: "1.1.1.1".into(),
            }],
            want: Ok("ip1.1.1.1".into()),
        };

        assert_eq!(target_to_key(&tc.input), tc.want);
    }

    #[test]
    fn target_to_key_ip_and_user_agent() {
        let tc = TestCase {
            input: vec![
                BanTarget {
                    target_type: TargetType::IP,
                    value: "1.1.1.1".into(),
                },
                BanTarget {
                    target_type: TargetType::UserAgent,
                    value: "Mozilla/5.0 (Android 4.4; Mobile; rv:41.0) Gecko/41.0 Firefox/41.0".into(),
                },
            ],
            want: Ok("ip1.1.1.1user-agentMozilla/5.0 (Android 4.4; Mobile; rv:41.0) Gecko/41.0 Firefox/41.0".into()),
        };

        assert_eq!(target_to_key(&tc.input), tc.want);
    }

    #[test]
    fn target_to_key_ip_and_2_user_agent() {
        let tc = TestCase {
            input: vec![
                BanTarget {
                    target_type: TargetType::IP,
                    value: "1.1.1.1".into(),
                },
                BanTarget {
                    target_type: TargetType::UserAgent,
                    value: "Mozilla/5.0 (Android 4.4; Mobile; rv:41.0) Gecko/41.0 Firefox/41.0"
                        .into(),
                },
                BanTarget {
                    target_type: TargetType::UserAgent,
                    value: "Some other user-agent".into(),
                },
            ],
            want: Err(BanTargetConversionError::InvalidTypeCount),
        };

        assert_eq!(target_to_key(&tc.input), tc.want);
    }

    #[test]
    fn target_to_key_2_ip() {
        let tc = TestCase {
            input: vec![
                BanTarget {
                    target_type: TargetType::IP,
                    value: "1.1.1.1".into(),
                },
                BanTarget {
                    target_type: TargetType::IP,
                    value: "2.2.2.2".into(),
                },
            ],
            want: Err(BanTargetConversionError::InvalidTypeCount),
        };

        assert_eq!(target_to_key(&tc.input), tc.want);
    }

    #[test]
    fn target_to_key_empty() {
        let tc = TestCase {
            input: vec![],
            want: Err(BanTargetConversionError::InvalidTypeCount),
        };

        assert_eq!(target_to_key(&tc.input), tc.want);
    }
}
