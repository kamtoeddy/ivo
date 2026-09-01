use chrono::{DateTime, Utc};
use ivo::ivo_schema;

type Timestamp = DateTime<Utc>;

#[ivo_schema(
    input(TimestampsInput, derive(Debug, Clone, PartialEq)),
    output(TimestampsData, derive(Debug, Clone, PartialEq))
)]
mod timestamps_schema {
    use super::Timestamp;
    use chrono::Utc;

    struct Fields {
        #[lax("default-username".to_string())]
        pub username: String,

        #[created_at]
        pub created_at: Timestamp,

        #[updated_at]
        pub updated_at: Timestamp,
    }

    #[timestamps(|| Utc::now())]
    const _: () = ();
}
