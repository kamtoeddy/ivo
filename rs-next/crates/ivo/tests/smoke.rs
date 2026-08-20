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
    let input = user_schema_dual::UserInput {
        name: "test".to_string(),
    };
    user_schema_dual::UserModel
        .delete(input, &())
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
