use serde_json::Value;

// TimeStampTool
#[derive(Debug, Clone)]
pub struct TimeStampKeys {
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

pub struct TimeStampTool {
    keys: TimeStampKeys,
    nullable: bool,
}

impl TimeStampTool {
    const IS_UPDATED_AT_NULLABLE_DEFAULT: bool = true;

    pub fn new(timestamps: Option<&Value>) -> Self {
        // timestamps: Option<Boolean | Object>
        if timestamps.is_none() {
            return Self {
                keys: TimeStampKeys {
                    created_at: None,
                    updated_at: None,
                },
                nullable: false,
            };
        }

        match timestamps.unwrap() {
            Value::Bool(b) => {
                if *b {
                    return Self {
                        keys: TimeStampKeys {
                            created_at: Some("created_at".into()),
                            updated_at: Some("updated_at".into()),
                        },
                        nullable: Self::IS_UPDATED_AT_NULLABLE_DEFAULT,
                    };
                } else {
                    return Self {
                        keys: TimeStampKeys {
                            created_at: None,
                            updated_at: None,
                        },
                        nullable: false,
                    };
                }
            }
            Value::Object(map) => {
                let created_at = map
                    .get("created_at")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());

                let updated_at = match map.get("updated_at") {
                    Some(Value::Object(o)) => {
                        o.get("key").and_then(|v| v.as_str()).map(|s| s.to_string())
                    }
                    Some(Value::String(s)) => Some(s.clone()),
                    _ => None,
                };

                let nullable = match map.get("updated_at") {
                    Some(Value::Object(o)) => o
                        .get("nullable")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(Self::IS_UPDATED_AT_NULLABLE_DEFAULT),
                    _ => Self::IS_UPDATED_AT_NULLABLE_DEFAULT,
                };

                return Self {
                    keys: TimeStampKeys {
                        created_at,
                        updated_at,
                    },
                    nullable,
                };
            }
            _ => {
                return Self {
                    keys: TimeStampKeys {
                        created_at: None,
                        updated_at: None,
                    },
                    nullable: false,
                }
            }
        }
    }

    pub fn get_keys(&self) -> &TimeStampKeys {
        &self.keys
    }

    pub fn is_timestamp_key(&self, key: &str) -> bool {
        if let Some(ref k) = self.keys.created_at {
            if k == key {
                return true;
            }
        }
        if let Some(ref k) = self.keys.updated_at {
            if k == key {
                return true;
            }
        }
        false
    }

    pub fn is_nullable(&self) -> bool {
        self.nullable
    }

    pub fn with_timestamps(&self) -> bool {
        self.keys.created_at.is_some() || self.keys.updated_at.is_some()
    }
}
