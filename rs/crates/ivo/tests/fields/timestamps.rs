use ivo::{DefaultErrorTool, IvoField, IvoStruct, IvoStructMethods, Schema};

use crate::async_test_matrix;

async fn should_respect_created_at_timestamp_with_default_name() {
    type Timestamp = u32;

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        created_at: Timestamp,
        lax: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        lax: i32,
    }

    struct Timer {
        time: Timestamp,
    }

    impl Timer {
        fn new() -> Self {
            Self { time: 0 }
        }

        fn now(&mut self) -> Timestamp {
            self.time += 1;

            self.time
        }
    }

    let schema: Schema<DataInput, Data, Option<()>, Timestamp, DefaultErrorTool> = Schema::new(
        |f| {
            f.set("lax", IvoField::LAX.default(1234))
                .set_timestamps(|t| {
                    let mut timer = Timer::new();

                    t.date_fn(move || timer.now()).created_at(None)
                })
        },
        |o| o,
    );

    let model = schema.get_model();

    let (data, _) = model
        .create(&PartialDataInput { lax: Some(400) }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            created_at: 1,
            lax: 400,
        }
    );

    let (updates, _) = model
        .update(&data, &PartialDataInput { lax: Some(200) }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            created_at: None,
            lax: Some(200),
        }
    );
}

async_test_matrix!(should_respect_created_at_timestamp_with_default_name);

async fn should_respect_created_at_timestamp_with_custom_name() {
    type Timestamp = u32;

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        custom_created_at: Timestamp,
        lax: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        lax: i32,
    }

    struct Timer {
        time: Timestamp,
    }

    impl Timer {
        fn new() -> Self {
            Self { time: 0 }
        }

        fn now(&mut self) -> Timestamp {
            self.time += 1;

            self.time
        }
    }

    let schema: Schema<DataInput, Data, Option<()>, Timestamp, DefaultErrorTool> = Schema::new(
        |f| {
            f.set("lax", IvoField::LAX.default(1234))
                .set_timestamps(|t| {
                    let mut timer = Timer::new();

                    t.date_fn(move || timer.now())
                        .created_at(Some("custom_created_at"))
                })
        },
        |o| o,
    );

    let model = schema.get_model();

    let (data, _) = model
        .create(&PartialDataInput { lax: Some(400) }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            custom_created_at: 1,
            lax: 400,
        }
    );

    let (updates, _) = model
        .update(&data, &PartialDataInput { lax: Some(200) }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            custom_created_at: None,
            lax: Some(200),
        }
    );
}

async_test_matrix!(should_respect_created_at_timestamp_with_custom_name);

async fn should_respect_updated_at_timestamp_with_default_name() {
    type Timestamp = u32;

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: i32,
        updated_at: Timestamp,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        lax: i32,
    }

    struct Timer {
        time: Timestamp,
    }

    impl Timer {
        fn new() -> Self {
            Self { time: 0 }
        }

        fn now(&mut self) -> Timestamp {
            self.time += 1;

            self.time
        }
    }

    let schema: Schema<DataInput, Data, Option<()>, Timestamp, DefaultErrorTool> = Schema::new(
        |f| {
            f.set("lax", IvoField::LAX.default(1234))
                .set_timestamps(|t| {
                    let mut timer = Timer::new();

                    t.date_fn(move || timer.now()).updated_at(None, false)
                })
        },
        |o| o,
    );

    let model = schema.get_model();

    let (data, _) = model
        .create(&PartialDataInput { lax: Some(400) }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            lax: 400,
            updated_at: 1,
        }
    );

    let (updates, _) = model
        .update(&data, &PartialDataInput { lax: Some(200) }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            lax: Some(200),
            updated_at: Some(2),
        }
    );
}

async_test_matrix!(should_respect_updated_at_timestamp_with_default_name);

async fn should_respect_updated_at_timestamp_with_custom_name() {
    type Timestamp = u32;

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: i32,
        custom_updated_at: Timestamp,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        lax: i32,
    }

    struct Timer {
        time: Timestamp,
    }

    impl Timer {
        fn new() -> Self {
            Self { time: 0 }
        }

        fn now(&mut self) -> Timestamp {
            self.time += 1;

            self.time
        }
    }

    let schema: Schema<DataInput, Data, Option<()>, Timestamp, DefaultErrorTool> = Schema::new(
        |f| {
            f.set("lax", IvoField::LAX.default(1234))
                .set_timestamps(|t| {
                    let mut timer = Timer::new();

                    t.date_fn(move || timer.now())
                        .updated_at(Some("custom_updated_at"), false)
                })
        },
        |o| o,
    );

    let model = schema.get_model();

    let (data, _) = model
        .create(&PartialDataInput { lax: Some(400) }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            lax: 400,
            custom_updated_at: 1,
        }
    );

    let (updates, _) = model
        .update(&data, &PartialDataInput { lax: Some(200) }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            lax: Some(200),
            custom_updated_at: Some(2),
        }
    );
}

async_test_matrix!(should_respect_updated_at_timestamp_with_custom_name);

async fn should_respect_optional_updated_at_timestamp_with_default_name() {
    type Timestamp = u32;

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: i32,
        updated_at: Option<Timestamp>,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        lax: i32,
    }

    struct Timer {
        time: Timestamp,
    }

    impl Timer {
        fn new() -> Self {
            Self { time: 0 }
        }

        fn now(&mut self) -> Timestamp {
            self.time += 1;

            self.time
        }
    }

    let schema: Schema<DataInput, Data, Option<()>, Timestamp, DefaultErrorTool> = Schema::new(
        |f| {
            f.set("lax", IvoField::LAX.default(1234))
                .set_timestamps(|t| {
                    let mut timer = Timer::new();

                    t.date_fn(move || timer.now()).updated_at(None, true)
                })
        },
        |o| o,
    );

    let model = schema.get_model();

    let (data, _) = model
        .create(&PartialDataInput { lax: Some(400) }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            lax: 400,
            updated_at: None,
        }
    );

    let (updates, _) = model
        .update(&data, &PartialDataInput { lax: Some(200) }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            lax: Some(200),
            updated_at: Some(Some(1)),
        }
    );

    assert_eq!(
        data.ivo_internal_clone_with(updates),
        Data {
            lax: 200,
            updated_at: Some(1),
        }
    );
}

async_test_matrix!(should_respect_optional_updated_at_timestamp_with_default_name);

async fn should_respect_optional_updated_at_timestamp_with_custom_name() {
    type Timestamp = u32;

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: i32,
        custom_updated_at: Option<Timestamp>,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        lax: i32,
    }

    struct Timer {
        time: Timestamp,
    }

    impl Timer {
        fn new() -> Self {
            Self { time: 0 }
        }

        fn now(&mut self) -> Timestamp {
            self.time += 1;

            self.time
        }
    }

    let schema: Schema<DataInput, Data, Option<()>, Timestamp, DefaultErrorTool> = Schema::new(
        |f| {
            f.set("lax", IvoField::LAX.default(1234))
                .set_timestamps(|t| {
                    let mut timer = Timer::new();

                    t.date_fn(move || timer.now())
                        .updated_at(Some("custom_updated_at"), true)
                })
        },
        |o| o,
    );

    let model = schema.get_model();

    let (data, _) = model
        .create(&PartialDataInput { lax: Some(400) }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            lax: 400,
            custom_updated_at: None,
        }
    );

    let (updates, _) = model
        .update(&data, &PartialDataInput { lax: Some(200) }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            lax: Some(200),
            custom_updated_at: Some(Some(1)),
        }
    );

    assert_eq!(
        data.ivo_internal_clone_with(updates),
        Data {
            lax: 200,
            custom_updated_at: Some(1),
        }
    );
}

async_test_matrix!(should_respect_optional_updated_at_timestamp_with_custom_name);
