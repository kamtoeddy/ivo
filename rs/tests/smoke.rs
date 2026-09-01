use ivo::ivo_schema;
use std::sync::atomic::Ordering;

use crate::{ON_FAILURE_COUNTER, ON_SUCCESS_COUNTER};

#[ivo_schema(input(User))]
mod user_schema {
    struct Fields {
        #[required]
        pub name: String,

        #[lax(String::from("user"))]
        pub role: String,
    }
}

#[ivo_schema(
    input(UserInput, derive(Debug, Clone, PartialEq)),
    output(User, derive(Debug, Clone, PartialEq))
)]
mod user_schema_dual {
    struct Fields {
        #[required]
        #[on_delete(async |_data, _opts| { println!("deleted"); })]
        pub name: String,

        #[constant(|| String::from("default-id"))]
        pub id: String,
    }
}

#[ivo_schema(input(UserWithValidation, derive(Debug, Clone)))]
mod user_validation_schema {
    struct Fields {
        #[required]
        #[validate(async |name, _ctx, _opts| {
            if name.is_empty() {
                Err((String::from("name must not be empty"), None))
            } else {
                Ok(Some(name))
            }
        })]
        pub name: String,
    }
}

#[test]
fn smoke_single_struct() {
    let user = user_schema::User {
        name: "test".to_string(),
        role: "admin".to_string(),
    };
    assert_eq!(user.name, "test");

    let mut partial = user_schema::PartialUser::new();
    partial.set_name("updated".to_string());
    assert!(!partial.is_empty());

    let _model = user_schema::UserModel;
}

#[test]
fn smoke_dual_struct() {
    let input = user_schema_dual::UserInput {
        name: "test".to_string(),
    };
    assert_eq!(input.name, "test");

    let output = user_schema_dual::User {
        name: "test".to_string(),
        id: String::from("default-id"),
    };
    assert_eq!(output.name, "test");
    assert_eq!(output.id, "default-id");
}

#[tokio::test]
async fn smoke_model_create_single_struct() {
    let input = user_schema::User {
        name: "test".to_string(),
        role: "user".to_string(),
    };
    let (data, ..) = user_schema::UserModel.create(input, ()).unwrap();
    assert_eq!(data.name, "test");
    assert_eq!(data.role, "user");
}

#[tokio::test]
async fn smoke_model_update_single_struct() {
    let existing = user_schema::User {
        name: "old".to_string(),
        role: "user".to_string(),
    };
    let mut updates = user_schema::PartialUser::new();
    updates.set_name("new".to_string());
    let (data, ..) = user_schema::UserModel
        .update(existing, updates, ())
        .unwrap();
    assert_eq!(data.name, Some("new".to_string()));
    assert_eq!(data.role, None);
}

#[tokio::test]
async fn smoke_model_create_dual_struct() {
    let input = user_schema_dual::UserInput {
        name: "test".to_string(),
    };
    let (data, ..) = user_schema_dual::UserModel.create(input, ()).unwrap();
    assert_eq!(data.name, "test");
    assert_eq!(data.id, "default-id");
}

#[tokio::test]
async fn smoke_model_delete_dual_struct() {
    let output = user_schema_dual::User {
        name: "test".to_string(),
        id: String::from("default-id"),
    };
    user_schema_dual::UserModel.delete(&output, ()).await;
}

#[tokio::test]
async fn smoke_model_validator_pass() {
    let input = user_validation_schema::UserWithValidation {
        name: "test".to_string(),
    };
    let (data, ..) = user_validation_schema::UserWithValidationModel
        .create(input, ())
        .await
        .unwrap();
    assert_eq!(data.name, "test");
}

#[tokio::test]
async fn smoke_model_validator_fail() {
    let input = user_validation_schema::UserWithValidation {
        name: String::new(),
    };
    let result = user_validation_schema::UserWithValidationModel
        .create(input, ())
        .await;
    assert!(result.is_err());
    let (errors, _ctx_options) = result.unwrap_err();
    assert!(errors.contains_key("name"));
}

#[ivo_schema(
    input(UserInput, derive(Debug, Clone, PartialEq)),
    output(User, derive(Debug, Clone, PartialEq))
)]
mod user_sanitization_schema {
    struct Fields {
        #[required]
        pub name: String,

        #[ivo_virtual("raw_email")]
        #[sanitize(async |email, _ctx, _opts| { email.to_lowercase() })]
        pub email: String,

        #[depends_on("email")]
        #[default(|ctx, _opts| ctx.input().raw_email.clone().unwrap())]
        #[resolve(async |ctx, _opts| { ctx.input().raw_email.clone().unwrap() })]
        pub raw_email: String,
    }
}

#[tokio::test]
async fn smoke_model_sanitizer() {
    let input = user_sanitization_schema::UserInput {
        name: "test".to_string(),
        raw_email: "Test@Example.COM".to_string(),
    };
    let (data, ..) = user_sanitization_schema::UserModel
        .create(input, ())
        .await
        .unwrap();
    assert_eq!(data.name, "test");
    assert_eq!(data.raw_email, "test@example.com");
}

#[ivo_schema(
    input(UserInput, derive(Debug, Clone, PartialEq)),
    output(User, derive(Debug, Clone, PartialEq))
)]
mod user_dependent_schema {
    struct Fields {
        #[required]
        pub first_name: String,

        #[required]
        pub last_name: String,

        #[depends_on("first_name", "last_name")]
        #[default(|ctx, _opts| {
            format!("{} {}", ctx.values().first_name, ctx.values().last_name)
        })]
        #[resolve(async |ctx, _opts| {
            format!("{} {}", ctx.values().first_name, ctx.values().last_name)
        })]
        pub full_name: String,
    }
}

#[tokio::test]
async fn smoke_model_resolver() {
    let input = user_dependent_schema::UserInput {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
    };
    let (data, ..) = user_dependent_schema::UserModel
        .create(input, ())
        .await
        .unwrap();
    assert_eq!(data.first_name, "John");
    assert_eq!(data.last_name, "Doe");
    assert_eq!(data.full_name, "John Doe");
}

#[ivo_schema(
    input(UserInput, derive(Debug, Clone, PartialEq)),
    output(User, derive(Debug, Clone, PartialEq))
)]
mod user_virtual_alias_schema {
    struct Fields {
        #[required]
        pub name: String,

        #[ivo_virtual("raw_email")]
        #[sanitize(async |email, _ctx, _opts| { email.to_lowercase() })]
        pub email: String,

        #[depends_on("email")]
        #[default(|ctx, _opts| ctx.input().raw_email.clone().unwrap())]
        #[resolve(async |ctx, _opts| { ctx.input().raw_email.clone().unwrap() })]
        pub raw_email: String,
    }
}

#[tokio::test]
async fn smoke_virtual_alias() {
    let input = user_virtual_alias_schema::UserInput {
        name: "test".to_string(),
        raw_email: "Test@Example.COM".to_string(),
    };
    let (data, ..) = user_virtual_alias_schema::UserModel
        .create(input, ())
        .await
        .unwrap();
    assert_eq!(data.name, "test");
    assert_eq!(data.raw_email, "test@example.com");
}

#[ivo_schema(
    input(UserInput, derive(Debug, Clone, PartialEq)),
    output(User, derive(Debug, Clone, PartialEq))
)]
mod user_timestamps_schema {
    struct Fields {
        #[required]
        pub name: String,

        #[created_at]
        pub created_at: String,

        #[updated_at]
        pub updated_at: String,
    }

    #[timestamps(|| String::from("timestamp"))]
    const _: () = ();
}

#[tokio::test]
async fn smoke_timestamps() {
    let input = user_timestamps_schema::UserInput {
        name: "test".to_string(),
    };
    let (data, ..) = user_timestamps_schema::UserModel.create(input, ()).unwrap();
    assert_eq!(data.name, "test");
    assert_eq!(data.created_at, "timestamp");
    assert_eq!(data.updated_at, "timestamp");
}

#[ivo_schema(input(User, derive(Debug, Clone, PartialEq)))]
mod user_lax_default_schema {
    struct Fields {
        #[required]
        pub name: String,

        #[lax(String::from("user"))]
        pub role: String,
    }
}

#[ivo_schema(input(User, derive(Debug, Clone, PartialEq)))]
mod user_grouped_ignore_schema {
    struct Fields {
        #[lax(String::from("default_a"))]
        pub a: String,

        #[lax(String::from("default_b"))]
        pub b: String,
    }

    #[ignore(["a", "b"], async |_ctx, _opts| { true })]
    const _: () = ();
}

#[ivo_schema(input(User, derive(Debug, Clone, PartialEq)))]
mod user_grouped_required_schema {
    struct Fields {
        #[lax(String::from(""))]
        pub a: String,

        #[lax(String::from(""))]
        pub b: String,
    }

    #[required(["a", "b"], async |_ctx, _opts| {
        let mut errors = UserErrors::new();
        errors.set_a("field is required", None);
        errors.set_b("field is required", None);
        Some(errors)
    })]
    const _: () = ();
}

#[tokio::test]
async fn smoke_model_lax_default() {
    let mut input = user_lax_default_schema::PartialUser::new();
    input.set_name("test".to_string());
    let (data, ..) = user_lax_default_schema::UserModel
        .create(input, ())
        .unwrap();
    assert_eq!(data.name, "test");
    assert_eq!(data.role, "user");
}

#[tokio::test]
async fn smoke_model_grouped_ignore() {
    let mut input = user_grouped_ignore_schema::PartialUser::new();
    input.set_a("provided_a".to_string());
    input.set_b("provided_b".to_string());
    let (data, ..) = user_grouped_ignore_schema::UserModel
        .create(input, ())
        .await
        .unwrap();
    assert_eq!(data.a, "default_a");
    assert_eq!(data.b, "default_b");
}

#[tokio::test]
async fn smoke_model_grouped_required() {
    let input = user_grouped_required_schema::PartialUser::new();
    let result = user_grouped_required_schema::UserModel
        .create(input, ())
        .await;
    assert!(result.is_err());
    let (errors, _ctx_options) = result.unwrap_err();
    assert!(errors.contains_key("a"));
    assert!(errors.contains_key("b"));
    assert_eq!(errors.get("a").unwrap().reason, "field is required");
    assert_eq!(errors.get("b").unwrap().reason, "field is required");
}

#[ivo_schema(input(User, derive(Debug, Clone, PartialEq)))]
mod user_field_ignore_schema {
    struct Fields {
        #[lax(String::from("default_a"))]
        #[ignore(|_ctx, _opts| true)]
        pub a: String,

        #[lax(String::from(""))]
        pub b: String,
    }
}

#[ivo_schema(input(User, derive(Debug, Clone, PartialEq)))]
mod user_field_ignore_init_schema {
    struct Fields {
        #[lax(String::from("default_a"))]
        #[ignore_init]
        pub a: String,

        #[lax(String::from(""))]
        pub b: String,
    }
}

#[ivo_schema(input(User, derive(Debug, Clone, PartialEq)))]
mod user_field_required_schema {
    struct Fields {
        #[lax(String::from(""))]
        #[required(|_ctx, _opts| Some(String::from("a is required")))]
        pub a: String,

        #[lax(String::from(""))]
        pub b: String,
    }
}

#[ivo_schema(input(User, derive(Debug, Clone, PartialEq)))]
mod user_field_ignore_update_schema {
    struct Fields {
        #[required]
        pub name: String,

        #[lax(String::from(""))]
        #[ignore_update(|ctx, _opts| ctx.values().role == *"admin")]
        pub role: String,
    }
}

#[tokio::test]
async fn smoke_model_field_ignore() {
    let mut input = user_field_ignore_schema::PartialUser::new();
    input.set_a("provided_a".to_string());
    input.set_b("provided_b".to_string());
    let (data, ..) = user_field_ignore_schema::UserModel
        .create(input, ())
        .unwrap();
    assert_eq!(data.a, "default_a");
    assert_eq!(data.b, "provided_b");
}

#[tokio::test]
async fn smoke_model_field_ignore_init() {
    let mut input = user_field_ignore_init_schema::PartialUser::new();
    input.set_a("provided_a".to_string());
    input.set_b("provided_b".to_string());
    let (data, ..) = user_field_ignore_init_schema::UserModel
        .create(input, ())
        .unwrap();
    assert_eq!(data.a, "default_a");
    assert_eq!(data.b, "provided_b");
}

#[tokio::test]
async fn smoke_model_field_required() {
    let mut input = user_field_required_schema::PartialUser::new();
    input.set_b("b".to_string());
    let result = user_field_required_schema::UserModel.create(input, ());
    assert!(result.is_err());
    let (errors, _ctx_options) = result.unwrap_err();
    assert!(errors.contains_key("a"));
}

#[tokio::test]
async fn smoke_model_field_ignore_update() {
    let existing = user_field_ignore_update_schema::User {
        name: "test".to_string(),
        role: "admin".to_string(),
    };
    let mut updates = user_field_ignore_update_schema::PartialUser::new();
    updates.set_role("user".to_string());
    let (failed, _ctx_options) = user_field_ignore_update_schema::UserModel
        .update(existing, updates, ())
        .err()
        .unwrap();
    assert!(failed.is_none());
}

#[ivo_schema(input(User, derive(Debug, Clone, PartialEq)))]
mod user_re_validate_schema {
    struct Fields {
        #[required]
        #[validate(async |name, _ctx, _opts| { Ok(Some(name)) })]
        #[re_validate(async |name, _ctx, _opts| {
            if name == "bad" {
                Err((String::from("name cannot be bad"), None))
            } else {
                Ok(Some(name))
            }
        })]
        pub name: String,
    }
}

#[ivo_schema(input(User, derive(Debug, Clone, PartialEq)))]
mod user_readonly_schema {
    struct Fields {
        #[required]
        pub name: String,

        #[required]
        #[readonly]
        #[validate(async |id, _ctx, _opts| { Ok(Some(id)) })]
        pub id: String,
    }
}

#[ivo_schema(
    input(UserInput, derive(Debug, Clone, PartialEq)),
    output(User, derive(Debug, Clone, PartialEq))
)]
mod user_dependent_default_schema {
    struct Fields {
        #[required]
        pub name: String,

        #[depends_on("name")]
        #[default(|_ctx, _opts| String::from("default-status"))]
        #[resolve(async |_ctx, _opts| { String::from("default-status") })]
        pub status: String,
    }
}

#[tokio::test]
async fn smoke_model_re_validate_fail() {
    let input = user_re_validate_schema::User {
        name: "bad".to_string(),
    };
    let result = user_re_validate_schema::UserModel.create(input, ()).await;
    assert!(result.is_err());
    let (errors, _ctx_options) = result.unwrap_err();
    assert!(errors.contains_key("name"));
}

#[tokio::test]
async fn smoke_model_re_validate_pass() {
    let input = user_re_validate_schema::User {
        name: "good".to_string(),
    };
    let (data, ..) = user_re_validate_schema::UserModel
        .create(input, ())
        .await
        .unwrap();
    assert_eq!(data.name, "good");
}

#[tokio::test]
async fn smoke_model_readonly_update() {
    let existing = user_readonly_schema::User {
        name: "old".to_string(),
        id: "1".to_string(),
    };
    let mut updates = user_readonly_schema::PartialUser::new();
    updates.set_name("new".to_string());
    updates.set_id("2".to_string());
    let (data, ..) = user_readonly_schema::UserModel
        .update(existing, updates, ())
        .await
        .unwrap();
    assert_eq!(data.name, Some("new".to_string()));
    assert_eq!(data.id, None);
}

#[tokio::test]
async fn smoke_model_dependent_default() {
    let input = user_dependent_default_schema::UserInput {
        name: "test".to_string(),
    };
    let (data, ..) = user_dependent_default_schema::UserModel
        .create(input, ())
        .await
        .unwrap();
    assert_eq!(data.name, "test");
    assert_eq!(data.status, "default-status");
}

#[ivo_schema(input(User, derive(Debug, Clone, PartialEq)))]
mod user_required_error_schema {
    struct Fields {
        #[required]
        #[required_error("name is mandatory")]
        pub name: String,
    }
}

#[ivo_schema(
    input(UserInput, derive(Debug, Clone, PartialEq)),
    output(User, derive(Debug, Clone, PartialEq))
)]
mod user_constant_static_schema {
    struct Fields {
        #[required]
        pub name: String,

        #[constant(1234)]
        pub id: i32,
    }
}

#[ivo_schema(
    input(UserInput, derive(Debug, Clone, PartialEq)),
    output(User, derive(Debug, Clone, PartialEq))
)]
mod user_constant_resolver_schema {
    struct Fields {
        #[required]
        pub name: String,

        #[constant(async |_ctx, _opts| { 5678 })]
        pub id: i32,
    }
}

#[ivo_schema(input(User, derive(Debug, Clone, PartialEq)))]
mod user_grouped_ignore_update_schema {
    struct Fields {
        #[required]
        pub name: String,

        #[lax(String::from(""))]
        pub role: String,
    }

    #[ignore_update(|_ctx, _opts| true)]
    const _: () = ();
}

#[tokio::test]
async fn smoke_model_required_error_static() {
    let input = user_required_error_schema::PartialUser::new();
    let result = user_required_error_schema::UserModel.create(input, ());
    assert!(result.is_err());
    let (errors, _ctx_options) = result.unwrap_err();
    let err = errors.get("name").unwrap();
    assert_eq!(err.reason, "name is mandatory");
}

#[tokio::test]
async fn smoke_model_constant_static() {
    let input = user_constant_static_schema::UserInput {
        name: "test".to_string(),
    };
    let (data, ..) = user_constant_static_schema::UserModel
        .create(input, ())
        .unwrap();
    assert_eq!(data.id, 1234);
}

#[tokio::test]
async fn smoke_model_constant_resolver() {
    let input = user_constant_resolver_schema::UserInput {
        name: "test".to_string(),
    };
    let (data, ..) = user_constant_resolver_schema::UserModel
        .create(input, ())
        .await
        .unwrap();
    assert_eq!(data.id, 5678);
}

#[tokio::test]
async fn smoke_model_grouped_ignore_update() {
    let existing = user_grouped_ignore_update_schema::User {
        name: "test".to_string(),
        role: "admin".to_string(),
    };
    let mut updates = user_grouped_ignore_update_schema::PartialUser::new();
    updates.set_role("user".to_string());
    let (failed, _ctx_options) = user_grouped_ignore_update_schema::UserModel
        .update(existing, updates, ())
        .err()
        .unwrap();
    assert!(failed.is_none());
}

#[ivo_schema(input(User, derive(Debug, Clone, PartialEq), derive_partial(Debug)))]
mod user_derive_partial_schema {
    struct Fields {
        #[required]
        pub name: String,

        #[lax(String::from(""))]
        pub role: String,
    }
}

#[test]
fn smoke_derive_partial() {
    let mut partial = user_derive_partial_schema::PartialUser::new();
    partial.set_name("test".to_string());
    let debug = format!("{:?}", partial);
    assert!(debug.contains("test"));
}

#[ivo_schema(
    input(UserInput, derive(Debug, Clone, PartialEq)),
    output(User, derive(Debug, Clone, PartialEq))
)]
mod user_on_delete_field_schema {
    struct Fields {
        #[required]
        pub name: String,

        #[constant(|| String::from("id"))]
        #[on_delete(async |_data, _opts| { panic!("field on_delete invoked") })]
        pub id: String,
    }
}

#[ivo_schema(input(User, derive(Debug, Clone, PartialEq)))]
mod user_on_delete_grouped_schema {
    struct Fields {
        #[required]
        pub name: String,
    }

    #[on_delete(async |_data, _opts| { panic!("grouped on_delete invoked") })]
    const _: () = ();
}

#[tokio::test]
#[should_panic(expected = "field on_delete invoked")]
async fn smoke_model_on_delete_field() {
    let output = user_on_delete_field_schema::User {
        name: "deleted".to_string(),
        id: "id".to_string(),
    };
    user_on_delete_field_schema::UserModel
        .delete(&output, ())
        .await;
}

#[tokio::test]
#[should_panic(expected = "grouped on_delete invoked")]
async fn smoke_model_on_delete_grouped() {
    let output = user_on_delete_grouped_schema::User {
        name: "deleted".to_string(),
    };
    user_on_delete_grouped_schema::UserModel
        .delete(&output, ())
        .await;
}

#[ivo_schema(input(User, derive(Debug, Clone, PartialEq)))]
mod user_readonly_lax_schema {
    struct Fields {
        #[required]
        pub name: String,

        #[lax(String::from("user"))]
        #[readonly]
        pub role: String,
    }
}

#[tokio::test]
async fn smoke_model_readonly_lax_default() {
    let existing = user_readonly_lax_schema::User {
        name: "test".to_string(),
        role: "user".to_string(),
    };
    let mut updates = user_readonly_lax_schema::PartialUser::new();
    updates.set_role("admin".to_string());
    let (data, ..) = user_readonly_lax_schema::UserModel
        .update(existing, updates, ())
        .unwrap();
    assert_eq!(data.role, Some("admin".to_string()));

    let existing = user_readonly_lax_schema::User {
        name: "test".to_string(),
        role: "admin".to_string(),
    };
    let mut updates = user_readonly_lax_schema::PartialUser::new();
    updates.set_role("super".to_string());
    let (failed, _ctx_options) = user_readonly_lax_schema::UserModel
        .update(existing, updates, ())
        .err()
        .unwrap();
    assert!(failed.is_none());
}

#[ivo_schema(
    input(UserInput, derive(Debug, Clone, PartialEq)),
    output(User, derive(Debug, Clone, PartialEq))
)]
mod user_passthrough_schema {
    struct Fields {
        #[required]
        #[input(doc = "input name")]
        #[output(doc = "output name")]
        pub name: String,

        #[constant(|| String::from("id"))]
        #[output(doc = "output id only")]
        pub id: String,
    }
}

#[test]
fn smoke_passthrough_attrs() {
    let input = user_passthrough_schema::UserInput {
        name: "test".to_string(),
    };
    let output = user_passthrough_schema::User {
        name: "test".to_string(),
        id: "id".to_string(),
    };
    assert_eq!(input.name, output.name);
}

#[ivo_schema(
    input(UserInput, derive(Debug, Clone, PartialEq)),
    output(User, derive(Debug, Clone, PartialEq))
)]
mod user_partial_passthrough_schema {
    struct Fields {
        #[required]
        #[partial(doc = "partial name")]
        #[input_partial(allow(dead_code))]
        #[output_partial(allow(unused_mut))]
        pub name: String,

        #[constant(|| String::from("id"))]
        pub id: String,
    }
}

#[test]
fn smoke_partial_passthrough_attrs() {
    let mut input_partial = user_partial_passthrough_schema::PartialUserInput::new();
    input_partial.set_name("test".to_string());
    assert!(!input_partial.is_empty());

    let mut output_partial = user_partial_passthrough_schema::PartialUser::new();
    output_partial.set_name("test".to_string());
    assert!(!output_partial.is_empty());
}

#[tokio::test]
async fn smoke_model_on_success_trigger() {
    ON_SUCCESS_COUNTER.store(0, Ordering::SeqCst);

    #[ivo_schema(input(User, derive(Debug, Clone, PartialEq)))]
    mod user_on_success_schema {
        struct Fields {
            #[required]
            #[on_success(async |_ctx, _opts| {
                crate::ON_SUCCESS_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            })]
            pub name: String,
        }
    }

    let input = user_on_success_schema::User {
        name: "test".to_string(),
    };
    let (data, _ctx_options, handle_success) = user_on_success_schema::UserModel.create(input, ())
        .ok()
        .unwrap();
    assert_eq!(data.name, "test");
    handle_success().await;
    assert_eq!(ON_SUCCESS_COUNTER.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn smoke_model_on_failure_trigger() {
    ON_FAILURE_COUNTER.store(0, Ordering::SeqCst);

    #[ivo_schema(input(User, derive(Debug, Clone, PartialEq)))]
    mod user_on_failure_schema {
        struct Fields {
            #[required]
            #[validate(async |name, _ctx, _opts| {
                if name.is_empty() {
                    Err((String::from("name must not be empty"), None))
                } else {
                    Ok(Some(name))
                }
            })]
            #[on_failure(async |_ctx, _opts| {
                crate::ON_FAILURE_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            })]
            pub name: String,
        }
    }

    let input = user_on_failure_schema::User {
        name: String::new(),
    };
    let (errors, _ctx_options, handle_failure) = user_on_failure_schema::UserModel
        .create(input, ())
        .await
        .err()
        .unwrap();
    assert!(errors.contains_key("name"));
    handle_failure().await;
    assert_eq!(ON_FAILURE_COUNTER.load(Ordering::SeqCst), 1);
}

#[ivo_schema(input(User, derive(Debug, Clone, PartialEq)))]
mod user_post_validate_pass_schema {
    struct Fields {
        #[required]
        pub password: String,

        #[required]
        pub password_confirmation: String,
    }

    #[post_validate(["password", "password_confirmation"], validate = async |_ctx, _opts| {
        if _ctx.input().password != _ctx.input().password_confirmation {
            let mut errors = UserErrors::new();
            errors.set_password("passwords do not match", None);
            Err(errors)
        } else {
            Ok(None)
        }
    })]
    const _: () = ();
}

#[ivo_schema(input(User, derive(Debug, Clone, PartialEq)))]
mod user_post_validate_fail_schema {
    struct Fields {
        #[required]
        pub password: String,

        #[required]
        pub password_confirmation: String,
    }

    #[post_validate(["password", "password_confirmation"], validate = async |_ctx, _opts| {
        if _ctx.input().password != _ctx.input().password_confirmation {
            let mut errors = UserErrors::new();
            errors.set_password("passwords do not match", None);
            Err(errors)
        } else {
            Ok(None)
        }
    })]
    const _: () = ();
}

#[ivo_schema(input(User, derive(Debug, Clone, PartialEq)))]
mod user_post_validate_sync_schema {
    struct Fields {
        #[required]
        pub a: String,

        #[required]
        pub b: String,
    }

    #[post_validate(["a", "b"], validate = |_ctx, _opts| {
        let mut updates = PartialUser::new();
        updates.set_a("updated_a".to_string());
        updates.set_b("updated_b".to_string());
        Ok(Some(updates))
    })]
    const _: () = ();
}

#[tokio::test]
async fn smoke_model_post_validate_pass() {
    let input = user_post_validate_pass_schema::User {
        password: "secret".to_string(),
        password_confirmation: "secret".to_string(),
    };
    let (data, ..) = user_post_validate_pass_schema::UserModel
        .create(input, ())
        .await
        .unwrap();
    assert_eq!(data.password, "secret");
    assert_eq!(data.password_confirmation, "secret");
}

#[tokio::test]
async fn smoke_model_post_validate_fail() {
    let input = user_post_validate_fail_schema::User {
        password: "secret".to_string(),
        password_confirmation: "different".to_string(),
    };
    let (errors, _ctx_options) = user_post_validate_fail_schema::UserModel
        .create(input, ())
        .await
        .unwrap_err();
    assert!(errors.contains_key("password"));
}

#[tokio::test]
async fn smoke_model_post_validate_sync() {
    let input = user_post_validate_sync_schema::User {
        a: "a".to_string(),
        b: "b".to_string(),
    };
    let (data, ..) = user_post_validate_sync_schema::UserModel
        .create(input, ())
        .unwrap();
    assert_eq!(data.a, "updated_a");
    assert_eq!(data.b, "updated_b");
}

#[ivo_schema(input(User, derive(Debug, Clone, PartialEq)))]
mod user_post_validate_pre_schema {
    struct Fields {
        #[required]
        pub a: String,

        #[required]
        pub b: String,
    }

    #[post_validate(
        ["a", "b"],
        pre_validate = async |_ctx, _opts| {
            let mut updates = PartialUser::new();
            updates.set_a("pre_a".to_string());
            updates.set_b("pre_b".to_string());
            Ok(Some(updates))
        },
        validate = async |_ctx, _opts| {
            if _ctx.input().a != Some("pre_a".to_string()) {
                let mut errors = UserErrors::new();
                errors.set_a("a was not updated by pre_validate", None);
                Err(errors)
            } else {
                Ok(None)
            }
        }
    )]
    const _: () = ();
}

#[tokio::test]
async fn smoke_model_post_validate_pre_validate() {
    let input = user_post_validate_pre_schema::User {
        a: "orig".to_string(),
        b: "orig".to_string(),
    };
    let (data, ..) = user_post_validate_pre_schema::UserModel
        .create(input, ())
        .await
        .unwrap();
    assert_eq!(data.a, "pre_a");
    assert_eq!(data.b, "pre_b");
}

#[ivo_schema(input(User, derive(Debug, Clone, PartialEq)))]
mod user_post_validate_update_fail_schema {
    struct Fields {
        #[required]
        pub a: String,

        #[required]
        pub b: String,
    }

    #[post_validate(
        ["a", "b"],
        validate = async |_ctx, _opts| {
            if _ctx.input().a == Some("bad".to_string()) {
                let mut errors = UserErrors::new();
                errors.set_a("bad update value", None);
                Err(errors)
            } else {
                Ok(None)
            }
        }
    )]
    const _: () = ();
}

#[ivo_schema(input(User, derive(Debug, Clone, PartialEq)))]
mod user_post_validate_update_update_schema {
    struct Fields {
        #[required]
        pub a: String,

        #[required]
        pub b: String,
    }

    #[post_validate(
        ["a", "b"],
        validate = async |_ctx, _opts| {
            let mut updates = PartialUser::new();
            updates.set_a("updated".to_string());
            updates.set_b("updated_b".to_string());
            Ok(Some(updates))
        }
    )]
    const _: () = ();
}

#[tokio::test]
async fn smoke_model_post_validate_update_fail() {
    let existing = user_post_validate_update_fail_schema::User {
        a: "old".to_string(),
        b: "old_b".to_string(),
    };
    let mut updates = user_post_validate_update_fail_schema::PartialUser::new();
    updates.set_a("bad".to_string());
    updates.set_b("old_b".to_string());
    let (errors, _ctx_options) = user_post_validate_update_fail_schema::UserModel
        .update(existing, updates, ())
        .await
        .unwrap_err();
    assert!(errors.as_ref().unwrap().contains_key("a"));
}

#[tokio::test]
async fn smoke_model_post_validate_update_update() {
    let existing = user_post_validate_update_update_schema::User {
        a: "old".to_string(),
        b: "old_b".to_string(),
    };
    let mut updates = user_post_validate_update_update_schema::PartialUser::new();
    updates.set_a("ignored".to_string());
    updates.set_b("old_b".to_string());
    let (data, ..) = user_post_validate_update_update_schema::UserModel
        .update(existing, updates, ())
        .await
        .unwrap();
    assert_eq!(data.a, Some("updated".to_string()));
    assert_eq!(data.b, Some("updated_b".to_string()));
}
