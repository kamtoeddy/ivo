use ivo::ivo_schema;

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
        pub name: String,

        #[constant(|| String::from("default-id"))]
        pub id: String,
    }
}

#[ivo_schema(input(UserWithValidation, derive(Debug, Clone)))]
mod user_validation_schema {
    struct Fields {
        #[required]
        #[validate(|name, _ctx, _opts| async move {
            if name.is_empty() {
                Err(::ivo::FieldError {
                    reason: String::from("name must not be empty"),
                    metadata: None,
                })
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
    let created = user_schema::UserModel.create(input, &()).await.unwrap();
    assert_eq!(created.name, "test");
    assert_eq!(created.role, "user");
}

#[tokio::test]
async fn smoke_model_update_single_struct() {
    let existing = user_schema::User {
        name: "old".to_string(),
        role: "user".to_string(),
    };
    let mut updates = user_schema::PartialUser::new();
    updates.set_name("new".to_string());
    let updated = user_schema::UserModel
        .update(existing, updates, &())
        .await
        .unwrap();
    assert_eq!(updated.name, "new");
    assert_eq!(updated.role, "user");
}

#[tokio::test]
async fn smoke_model_create_dual_struct() {
    let input = user_schema_dual::UserInput {
        name: "test".to_string(),
    };
    let created = user_schema_dual::UserModel
        .create(input, &())
        .await
        .unwrap();
    assert_eq!(created.name, "test");
    assert_eq!(created.id, "default-id");
}

#[tokio::test]
async fn smoke_model_delete_dual_struct() {
    let output = user_schema_dual::User {
        name: "test".to_string(),
        id: String::from("default-id"),
    };
    user_schema_dual::UserModel
        .delete(&output, &())
        .await
        .unwrap();
}

#[tokio::test]
async fn smoke_model_validator_pass() {
    let input = user_validation_schema::UserWithValidation {
        name: "test".to_string(),
    };
    let created = user_validation_schema::UserWithValidationModel
        .create(input, &())
        .await
        .unwrap();
    assert_eq!(created.name, "test");
}

#[tokio::test]
async fn smoke_model_validator_fail() {
    let input = user_validation_schema::UserWithValidation {
        name: String::new(),
    };
    let result = user_validation_schema::UserWithValidationModel
        .create(input, &())
        .await;
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.contains_key("name"));
}

#[ivo_schema(input(UserWithSanitization, derive(Debug, Clone)))]
mod user_sanitization_schema {
    struct Fields {
        #[required]
        #[sanitize(|email, _ctx, _opts| async move { email.to_lowercase() })]
        pub email: String,
    }
}

#[tokio::test]
async fn smoke_model_sanitizer() {
    let input = user_sanitization_schema::UserWithSanitization {
        email: "Test@Example.COM".to_string(),
    };
    let created = user_sanitization_schema::UserWithSanitizationModel
        .create(input, &())
        .await
        .unwrap();
    assert_eq!(created.email, "test@example.com");
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

        #[depends_on(first_name, last_name)]
        #[resolve(|ctx, _opts| async move {
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
    let created = user_dependent_schema::UserModel
        .create(input, &())
        .await
        .unwrap();
    assert_eq!(created.first_name, "John");
    assert_eq!(created.last_name, "Doe");
    assert_eq!(created.full_name, "John Doe");
}

#[ivo_schema(
    input(UserInput, derive(Debug, Clone, PartialEq)),
    output(User, derive(Debug, Clone, PartialEq))
)]
mod user_virtual_alias_schema {
    struct Fields {
        #[required]
        pub name: String,

        #[ivo_virtual(alias = "raw_email")]
        pub email: String,

        #[depends_on(email)]
        #[resolve(|ctx, _opts| async move { ctx.input().raw_email.clone().unwrap() })]
        pub raw_email: String,
    }
}

#[tokio::test]
async fn smoke_virtual_alias() {
    let input = user_virtual_alias_schema::UserInput {
        name: "test".to_string(),
        raw_email: "Test@Example.COM".to_string(),
    };
    let created = user_virtual_alias_schema::UserModel
        .create(input, &())
        .await
        .unwrap();
    assert_eq!(created.name, "test");
    assert_eq!(created.raw_email, "Test@Example.COM");
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
    let created = user_timestamps_schema::UserModel
        .create(input, &())
        .await
        .unwrap();
    assert_eq!(created.name, "test");
    assert_eq!(created.created_at, "timestamp");
    assert_eq!(created.updated_at, "timestamp");
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

    #[ignore(["a", "b"], |_ctx, _opts| async move { true })]
    const _: () = ();
}

#[ivo_schema(input(User, derive(Debug, Clone, PartialEq)))]
mod user_grouped_required_schema {
    struct Fields {
        #[lax]
        pub a: String,

        #[lax]
        pub b: String,
    }

    #[required(["a", "b"], |_ctx, _opts| async move { true })]
    const _: () = ();
}

#[tokio::test]
async fn smoke_model_lax_default() {
    let mut input = user_lax_default_schema::PartialUser::new();
    input.set_name("test".to_string());
    let created = user_lax_default_schema::UserModel
        .create(input, &())
        .await
        .unwrap();
    assert_eq!(created.name, "test");
    assert_eq!(created.role, "user");
}

#[tokio::test]
async fn smoke_model_grouped_ignore() {
    let mut input = user_grouped_ignore_schema::PartialUser::new();
    input.set_a("provided_a".to_string());
    input.set_b("provided_b".to_string());
    let created = user_grouped_ignore_schema::UserModel
        .create(input, &())
        .await
        .unwrap();
    assert_eq!(created.a, "default_a");
    assert_eq!(created.b, "default_b");
}

#[tokio::test]
async fn smoke_model_grouped_required() {
    let mut input = user_grouped_required_schema::PartialUser::new();
    input.set_b("b".to_string());
    let result = user_grouped_required_schema::UserModel
        .create(input, &())
        .await;
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.contains_key("a"));
}

#[ivo_schema(input(User, derive(Debug, Clone, PartialEq)))]
mod user_field_ignore_schema {
    struct Fields {
        #[lax(String::from("default_a"))]
        #[ignore(|_ctx, _opts| true)]
        pub a: String,

        #[lax]
        pub b: String,
    }
}

#[ivo_schema(input(User, derive(Debug, Clone, PartialEq)))]
mod user_field_ignore_init_schema {
    struct Fields {
        #[lax(String::from("default_a"))]
        #[ignore_init]
        pub a: String,

        #[lax]
        pub b: String,
    }
}

#[ivo_schema(input(User, derive(Debug, Clone, PartialEq)))]
mod user_field_required_schema {
    struct Fields {
        #[lax]
        #[required(|_ctx, _opts| Some(String::from("a is required")))]
        pub a: String,

        #[lax]
        pub b: String,
    }
}

#[ivo_schema(input(User, derive(Debug, Clone, PartialEq)))]
mod user_field_ignore_update_schema {
    struct Fields {
        #[required]
        pub name: String,

        #[lax]
        #[ignore_update(|ctx, _opts| ctx.values().role == *"admin")]
        pub role: String,
    }
}

#[tokio::test]
async fn smoke_model_field_ignore() {
    let mut input = user_field_ignore_schema::PartialUser::new();
    input.set_a("provided_a".to_string());
    input.set_b("provided_b".to_string());
    let created = user_field_ignore_schema::UserModel
        .create(input, &())
        .await
        .unwrap();
    assert_eq!(created.a, "default_a");
    assert_eq!(created.b, "provided_b");
}

#[tokio::test]
async fn smoke_model_field_ignore_init() {
    let mut input = user_field_ignore_init_schema::PartialUser::new();
    input.set_a("provided_a".to_string());
    input.set_b("provided_b".to_string());
    let created = user_field_ignore_init_schema::UserModel
        .create(input, &())
        .await
        .unwrap();
    assert_eq!(created.a, "default_a");
    assert_eq!(created.b, "provided_b");
}

#[tokio::test]
async fn smoke_model_field_required() {
    let mut input = user_field_required_schema::PartialUser::new();
    input.set_b("b".to_string());
    let result = user_field_required_schema::UserModel
        .create(input, &())
        .await;
    assert!(result.is_err());
    let errors = result.unwrap_err();
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
    let updated = user_field_ignore_update_schema::UserModel
        .update(existing, updates, &())
        .await
        .unwrap();
    assert_eq!(updated.role, "admin");
}

#[ivo_schema(input(User, derive(Debug, Clone, PartialEq)))]
mod user_re_validate_schema {
    struct Fields {
        #[required]
        #[validate(|name, _ctx, _opts| async move { Ok(Some(name)) })]
        #[re_validate(|name, _ctx, _opts| async move {
            if name == "bad" {
                Err(::ivo::FieldError {
                    reason: String::from("name cannot be bad"),
                    metadata: None,
                })
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
        #[validate(|id, _ctx, _opts| async move { Ok(Some(id)) })]
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

        #[depends_on(name)]
        #[default(|| String::from("default-status"))]
        pub status: String,
    }
}

#[tokio::test]
async fn smoke_model_re_validate_fail() {
    let input = user_re_validate_schema::User {
        name: "bad".to_string(),
    };
    let result = user_re_validate_schema::UserModel.create(input, &()).await;
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.contains_key("name"));
}

#[tokio::test]
async fn smoke_model_re_validate_pass() {
    let input = user_re_validate_schema::User {
        name: "good".to_string(),
    };
    let created = user_re_validate_schema::UserModel
        .create(input, &())
        .await
        .unwrap();
    assert_eq!(created.name, "good");
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
    let updated = user_readonly_schema::UserModel
        .update(existing, updates, &())
        .await
        .unwrap();
    assert_eq!(updated.name, "new");
    assert_eq!(updated.id, "1");
}

#[tokio::test]
async fn smoke_model_dependent_default() {
    let input = user_dependent_default_schema::UserInput {
        name: "test".to_string(),
    };
    let created = user_dependent_default_schema::UserModel
        .create(input, &())
        .await
        .unwrap();
    assert_eq!(created.name, "test");
    assert_eq!(created.status, "default-status");
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

        #[constant(|_ctx, _opts| async move { 5678 })]
        pub id: i32,
    }
}

#[ivo_schema(input(User, derive(Debug, Clone, PartialEq)))]
mod user_grouped_ignore_update_schema {
    struct Fields {
        #[required]
        pub name: String,

        #[lax]
        pub role: String,
    }

    #[ignore_update([], |_ctx, _opts| true)]
    const _: () = ();
}

#[tokio::test]
async fn smoke_model_required_error_static() {
    let input = user_required_error_schema::PartialUser::new();
    let result = user_required_error_schema::UserModel
        .create(input, &())
        .await;
    assert!(result.is_err());
    let errors = result.unwrap_err();
    let err = errors.get("name").unwrap();
    assert_eq!(err.reason, "name is mandatory");
}

#[tokio::test]
async fn smoke_model_constant_static() {
    let input = user_constant_static_schema::UserInput {
        name: "test".to_string(),
    };
    let created = user_constant_static_schema::UserModel
        .create(input, &())
        .await
        .unwrap();
    assert_eq!(created.id, 1234);
}

#[tokio::test]
async fn smoke_model_constant_resolver() {
    let input = user_constant_resolver_schema::UserInput {
        name: "test".to_string(),
    };
    let created = user_constant_resolver_schema::UserModel
        .create(input, &())
        .await
        .unwrap();
    assert_eq!(created.id, 5678);
}

#[tokio::test]
async fn smoke_model_grouped_ignore_update() {
    let existing = user_grouped_ignore_update_schema::User {
        name: "test".to_string(),
        role: "admin".to_string(),
    };
    let mut updates = user_grouped_ignore_update_schema::PartialUser::new();
    updates.set_role("user".to_string());
    let updated = user_grouped_ignore_update_schema::UserModel
        .update(existing, updates, &())
        .await
        .unwrap();
    assert_eq!(updated.role, "admin");
}

#[ivo_schema(input(User, derive(Debug, Clone, PartialEq), derive_partial(Debug)))]
mod user_derive_partial_schema {
    struct Fields {
        #[required]
        pub name: String,

        #[lax]
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
mod user_on_delete_schema {
    struct Fields {
        #[required]
        pub name: String,

        #[constant(|| String::from("id"))]
        #[on_delete(|ctx, _opts| async move { assert_eq!(ctx.input().id, "id") })]
        pub id: String,
    }

    #[on_delete(|ctx, _opts| async move { assert_eq!(ctx.input().name, "deleted") })]
    const _: () = ();
}

#[tokio::test]
async fn smoke_model_on_delete() {
    let output = user_on_delete_schema::User {
        name: "deleted".to_string(),
        id: "id".to_string(),
    };
    user_on_delete_schema::UserModel
        .delete(&output, &())
        .await
        .unwrap();
}
