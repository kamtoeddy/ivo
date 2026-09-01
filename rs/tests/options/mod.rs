mod compile_fail;
mod ignore;
mod ignore_update;
mod parallel_hooks;
mod post_validate;
mod required;

use ivo::ivo_schema;

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod grouped_on_delete_schema {
    struct Fields {
        #[lax(1234)]
        pub lax: i32,

        #[lax(5678)]
        pub lax_1: i32,
    }

    #[on_delete(|_data, _opts| {
        if true {
            panic!("[options.on_delete]: handler triggered");
        }
    })]
    const _: () = ();
}

async fn should_properly_trigger_on_delete_handlers() {
    let data = grouped_on_delete_schema::Data { lax: 2, lax_1: 3 };

    grouped_on_delete_schema::DataModel.delete(&data, ());
}

async_test_matrix!(
    "[options.on_delete]: handler triggered",
    should_properly_trigger_on_delete_handlers
);

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod multiple_grouped_on_delete_schema {
    struct Fields {
        #[lax(1234)]
        pub lax: i32,

        #[lax(5678)]
        pub lax_1: i32,
    }

    #[on_delete(|_data, _opts| {})]
    const _: () = ();

    #[on_delete(|_data, _opts| {
        if true {
            panic!("[options.on_delete]: second handler triggered");
        }
    })]
    const _: () = ();
}

async fn should_properly_trigger_all_on_delete_handlers() {
    let data = multiple_grouped_on_delete_schema::Data { lax: 2, lax_1: 3 };

    multiple_grouped_on_delete_schema::DataModel.delete(&data, ());
}

async_test_matrix!(
    "[options.on_delete]: second handler triggered",
    should_properly_trigger_all_on_delete_handlers
);

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod allow_constant_and_dependents_on_success_schema {
    struct Fields {
        #[constant(1234)]
        pub id: i32,

        #[lax(5678)]
        pub lax: i32,

        #[depends_on("lax")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.input().lax.unwrap_or(0) + 1)]
        pub dependent: i32,
    }

    #[on_success(["id", "dependent"], |_ctx, _opts| {})]
    const _: () = ();
}

#[tokio::test]
async fn should_allow_constant_and_dependents_in_fields_array() {
    let input = allow_constant_and_dependents_on_success_schema::DataInput { lax: 5678 };

    let (created, ..) = allow_constant_and_dependents_on_success_schema::DataModel
        .create(input, ())
        .unwrap();

    assert_eq!(created.id, 1234);
    assert_eq!(created.dependent, 5679);
}

// -----------------------------------------------------------------------------
// Named const anchors for grouped options (GOAL.md §10)
// -----------------------------------------------------------------------------

#[test]
fn should_allow_named_consts_as_grouped_option_anchors() {
    // A named const (used here for its own stable identifier, matching
    // GOAL.md §10's `NAME_EMAIL_REQUIRED` example) must work exactly like the
    // anonymous `const _: () = ();` default.
    let (errors, ..) = named_const_option_schema::DataInputModel
        .create(
            named_const_option_schema::PartialDataInput {
                email: None,
                phone_number: None,
            },
            (),
        )
        .err().unwrap();

    assert!(errors.get("email").is_some());
    assert!(errors.get("phone_number").is_some());

    let (created, ..) = named_const_option_schema::DataInputModel
        .create(
            named_const_option_schema::PartialDataInput {
                email: Some(Some("a@b.com".to_string())),
                phone_number: None,
            },
            (),
        )
        .unwrap();

    assert_eq!(created.email.as_deref(), Some("a@b.com"));
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod named_const_option_schema {
    struct Fields {
        #[lax(None)]
        pub email: Option<String>,

        #[lax(None)]
        pub phone_number: Option<String>,
    }

    #[required(["email", "phone_number"], |ctx, _| {
        if ctx.input().email.is_some() || ctx.input().phone_number.is_some() {
            return None;
        }

        let mut errors = DataInputErrors::new();
        errors.set_email("either \"email\" or \"phone_number\" is required", None);
        errors.set_phone_number("either \"email\" or \"phone_number\" is required", None);
        Some(errors)
    })]
    const NAME_EMAIL_REQUIRED: () = ();
}
