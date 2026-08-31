use ivo::ivo_schema;

// -----------------------------------------------------------------------------
// Field-level #[ignore_update] on virtual fields
// -----------------------------------------------------------------------------

#[test]
fn should_respect_field_level_ignore_update_on_virtual_fields() {
    let created = sync_ignore_update_schema::DataModel
        .create(
            sync_ignore_update_schema::PartialDataInput {
                lax: None,
                virtual_field: Some("virtual_value".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_ignore_update_schema::Data {
            lax: 10,
            dependent: 2
        }
    );

    let updated = sync_ignore_update_schema::DataModel
        .update(
            created.data.clone(),
            sync_ignore_update_schema::PartialDataInput {
                lax: Some(30),
                virtual_field: Some("new_virtual_value".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_ignore_update_schema::PartialData {
            lax: Some(30),
            dependent: None,
        }
    );

    let failed = sync_ignore_update_schema::DataModel
        .update(
            created.data.clone(),
            sync_ignore_update_schema::PartialDataInput {
                lax: None,
                virtual_field: Some("ignored_value".into()),
            },
            (),
        )
        .err()
        .unwrap();

    assert!(failed.errors.is_none());
}

async fn should_respect_field_level_ignore_update_on_virtual_fields_async() {
    let created = async_ignore_update_schema::DataModel
        .create(
            async_ignore_update_schema::PartialDataInput {
                lax: None,
                virtual_field: Some("virtual_value".into()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_ignore_update_schema::Data {
            lax: 10,
            dependent: 2
        }
    );

    let updated = async_ignore_update_schema::DataModel
        .update(
            created.data.clone(),
            async_ignore_update_schema::PartialDataInput {
                lax: Some(30),
                virtual_field: Some("new_virtual_value".into()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_ignore_update_schema::PartialData {
            lax: Some(30),
            dependent: None,
        }
    );

    let failed = async_ignore_update_schema::DataModel
        .update(
            created.data.clone(),
            async_ignore_update_schema::PartialDataInput {
                lax: None,
                virtual_field: Some("ignored_value".into()),
            },
            (),
        )
        .await
        .err()
        .unwrap();

    assert!(failed.errors.is_none());
}

async_test_matrix!(should_respect_field_level_ignore_update_on_virtual_fields_async);

// -----------------------------------------------------------------------------
// Field-level #[ignore_update] on an *aliased* virtual field: the ignore
// resolver, the field's own provided/ignored bookkeeping, and the resulting
// (non-)update must all key off the alias consistently -- same coverage as
// the non-aliased case above, just confirming the alias doesn't break it.
// -----------------------------------------------------------------------------

#[test]
fn should_respect_field_level_ignore_update_on_an_aliased_virtual_field() {
    let created = sync_ignore_update_alias_schema::DataModel
        .create(
            sync_ignore_update_alias_schema::PartialDataInput {
                lax: None,
                virtual_alias: Some("virtual_value".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_ignore_update_alias_schema::Data {
            lax: 10,
            dependent: 2
        }
    );

    let updated = sync_ignore_update_alias_schema::DataModel
        .update(
            created.data.clone(),
            sync_ignore_update_alias_schema::PartialDataInput {
                lax: Some(30),
                virtual_alias: Some("new_virtual_value".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_ignore_update_alias_schema::PartialData {
            lax: Some(30),
            dependent: None,
        }
    );

    let failed = sync_ignore_update_alias_schema::DataModel
        .update(
            created.data.clone(),
            sync_ignore_update_alias_schema::PartialDataInput {
                lax: None,
                virtual_alias: Some("ignored_value".into()),
            },
            (),
        )
        .err()
        .unwrap();

    assert!(failed.errors.is_none());
}

async fn should_respect_field_level_ignore_update_on_an_aliased_virtual_field_async() {
    let created = async_ignore_update_alias_schema::DataModel
        .create(
            async_ignore_update_alias_schema::PartialDataInput {
                lax: None,
                virtual_alias: Some("virtual_value".into()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_ignore_update_alias_schema::Data {
            lax: 10,
            dependent: 2
        }
    );

    let updated = async_ignore_update_alias_schema::DataModel
        .update(
            created.data.clone(),
            async_ignore_update_alias_schema::PartialDataInput {
                lax: Some(30),
                virtual_alias: Some("new_virtual_value".into()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_ignore_update_alias_schema::PartialData {
            lax: Some(30),
            dependent: None,
        }
    );

    let failed = async_ignore_update_alias_schema::DataModel
        .update(
            created.data.clone(),
            async_ignore_update_alias_schema::PartialDataInput {
                lax: None,
                virtual_alias: Some("ignored_value".into()),
            },
            (),
        )
        .await
        .err()
        .unwrap();

    assert!(failed.errors.is_none());
}

async_test_matrix!(should_respect_field_level_ignore_update_on_an_aliased_virtual_field_async);

// -----------------------------------------------------------------------------
// Grouped #[ignore([...], handler)] on virtual fields
// -----------------------------------------------------------------------------

#[test]
fn should_respect_grouped_ignore_rule_on_virtual_fields() {
    let default_dependent_value = 1;
    let default_lax_value = 10;

    let created = sync_grouped_ignore_schema::DataModel
        .create(
            sync_grouped_ignore_schema::PartialDataInput {
                lax: None,
                virtual_field: Some("virtual_value".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_grouped_ignore_schema::Data {
            dependent: default_dependent_value,
            lax: default_lax_value,
        }
    );

    let created = sync_grouped_ignore_schema::DataModel
        .create(
            sync_grouped_ignore_schema::PartialDataInput {
                lax: Some(20),
                virtual_field: Some("keep".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_grouped_ignore_schema::Data {
            dependent: 2,
            lax: 20,
        }
    );

    let updated = sync_grouped_ignore_schema::DataModel
        .update(
            created.data.clone(),
            sync_grouped_ignore_schema::PartialDataInput {
                lax: Some(30),
                virtual_field: Some("keep".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_grouped_ignore_schema::PartialData {
            dependent: Some(3),
            lax: Some(30),
        }
    );

    let failed = sync_grouped_ignore_schema::DataModel
        .update(
            created.data.clone(),
            sync_grouped_ignore_schema::PartialDataInput {
                lax: None,
                virtual_field: Some("virtual_value".into()),
            },
            (),
        )
        .err()
        .unwrap();

    assert!(failed.errors.is_none());
}

async fn should_respect_grouped_ignore_rule_on_virtual_fields_async() {
    let default_dependent_value = 1;
    let default_lax_value = 10;

    let created = async_grouped_ignore_schema::DataModel
        .create(
            async_grouped_ignore_schema::PartialDataInput {
                lax: None,
                virtual_field: Some("virtual_value".into()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_grouped_ignore_schema::Data {
            dependent: default_dependent_value,
            lax: default_lax_value,
        }
    );

    let created = async_grouped_ignore_schema::DataModel
        .create(
            async_grouped_ignore_schema::PartialDataInput {
                lax: Some(20),
                virtual_field: Some("keep".into()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_grouped_ignore_schema::Data {
            dependent: 2,
            lax: 20,
        }
    );

    let updated = async_grouped_ignore_schema::DataModel
        .update(
            created.data.clone(),
            async_grouped_ignore_schema::PartialDataInput {
                lax: Some(30),
                virtual_field: Some("keep".into()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_grouped_ignore_schema::PartialData {
            dependent: Some(3),
            lax: Some(30),
        }
    );

    let failed = async_grouped_ignore_schema::DataModel
        .update(
            created.data.clone(),
            async_grouped_ignore_schema::PartialDataInput {
                lax: None,
                virtual_field: Some("virtual_value".into()),
            },
            (),
        )
        .await
        .err()
        .unwrap();

    assert!(failed.errors.is_none());
}

async_test_matrix!(should_respect_grouped_ignore_rule_on_virtual_fields_async);

// -----------------------------------------------------------------------------
// Grouped #[ignore([...], handler)] on an *aliased* virtual field. Note the
// field list in the attribute itself (`#[ignore(["virtual_field", "lax"], ..)]`)
// still names the field's *internal* schema name, not the alias -- aliases
// only rename the field externally (in `PartialDataInput`/`ctx.input()`),
// they're never valid inside a grouped option's field list (see
// `tests/options/compile_fail/on_success.rs`'s
// `should_reject_if_an_alias_with_foreign_name_is_provided_to_the_fields_array`
// for the equivalent compile-time rejection on `#[on_success(...)]`).
// -----------------------------------------------------------------------------

#[test]
fn should_respect_grouped_ignore_rule_on_an_aliased_virtual_field() {
    let default_dependent_value = 1;
    let default_lax_value = 10;

    let created = sync_grouped_ignore_alias_schema::DataModel
        .create(
            sync_grouped_ignore_alias_schema::PartialDataInput {
                lax: None,
                virtual_alias: Some("virtual_value".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_grouped_ignore_alias_schema::Data {
            dependent: default_dependent_value,
            lax: default_lax_value,
        }
    );

    let created = sync_grouped_ignore_alias_schema::DataModel
        .create(
            sync_grouped_ignore_alias_schema::PartialDataInput {
                lax: Some(20),
                virtual_alias: Some("keep".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_grouped_ignore_alias_schema::Data {
            dependent: 2,
            lax: 20,
        }
    );

    let updated = sync_grouped_ignore_alias_schema::DataModel
        .update(
            created.data.clone(),
            sync_grouped_ignore_alias_schema::PartialDataInput {
                lax: Some(30),
                virtual_alias: Some("keep".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_grouped_ignore_alias_schema::PartialData {
            dependent: Some(3),
            lax: Some(30),
        }
    );

    let failed = sync_grouped_ignore_alias_schema::DataModel
        .update(
            created.data.clone(),
            sync_grouped_ignore_alias_schema::PartialDataInput {
                lax: None,
                virtual_alias: Some("virtual_value".into()),
            },
            (),
        )
        .err()
        .unwrap();

    assert!(failed.errors.is_none());
}

async fn should_respect_grouped_ignore_rule_on_an_aliased_virtual_field_async() {
    let default_dependent_value = 1;
    let default_lax_value = 10;

    let created = async_grouped_ignore_alias_schema::DataModel
        .create(
            async_grouped_ignore_alias_schema::PartialDataInput {
                lax: None,
                virtual_alias: Some("virtual_value".into()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_grouped_ignore_alias_schema::Data {
            dependent: default_dependent_value,
            lax: default_lax_value,
        }
    );

    let created = async_grouped_ignore_alias_schema::DataModel
        .create(
            async_grouped_ignore_alias_schema::PartialDataInput {
                lax: Some(20),
                virtual_alias: Some("keep".into()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_grouped_ignore_alias_schema::Data {
            dependent: 2,
            lax: 20,
        }
    );

    let updated = async_grouped_ignore_alias_schema::DataModel
        .update(
            created.data.clone(),
            async_grouped_ignore_alias_schema::PartialDataInput {
                lax: Some(30),
                virtual_alias: Some("keep".into()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_grouped_ignore_alias_schema::PartialData {
            dependent: Some(3),
            lax: Some(30),
        }
    );

    let failed = async_grouped_ignore_alias_schema::DataModel
        .update(
            created.data.clone(),
            async_grouped_ignore_alias_schema::PartialDataInput {
                lax: None,
                virtual_alias: Some("virtual_value".into()),
            },
            (),
        )
        .await
        .err()
        .unwrap();

    assert!(failed.errors.is_none());
}

async_test_matrix!(should_respect_grouped_ignore_rule_on_an_aliased_virtual_field_async);

// -----------------------------------------------------------------------------
// Grouped #[ignore_update([...], handler)] on virtual fields
// -----------------------------------------------------------------------------

#[test]
fn should_respect_grouped_ignore_update_rule_on_virtual_fields() {
    let default_lax_value = 10;

    let created = sync_grouped_ignore_update_schema::DataModel
        .create(
            sync_grouped_ignore_update_schema::PartialDataInput {
                lax: None,
                virtual_field: Some("virtual_value".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_grouped_ignore_update_schema::Data {
            dependent: 2,
            lax: default_lax_value,
        }
    );

    let updated = sync_grouped_ignore_update_schema::DataModel
        .update(
            created.data.clone(),
            sync_grouped_ignore_update_schema::PartialDataInput {
                lax: Some(30),
                virtual_field: Some("keep".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_grouped_ignore_update_schema::PartialData {
            dependent: Some(3),
            lax: Some(30),
        }
    );

    let failed = sync_grouped_ignore_update_schema::DataModel
        .update(
            created.data.clone(),
            sync_grouped_ignore_update_schema::PartialDataInput {
                lax: None,
                virtual_field: Some("virtual_value".into()),
            },
            (),
        )
        .err()
        .unwrap();

    assert!(failed.errors.is_none());
}

async fn should_respect_grouped_ignore_update_rule_on_virtual_fields_async() {
    let default_lax_value = 10;

    let created = async_grouped_ignore_update_schema::DataModel
        .create(
            async_grouped_ignore_update_schema::PartialDataInput {
                lax: None,
                virtual_field: Some("virtual_value".into()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_grouped_ignore_update_schema::Data {
            dependent: 2,
            lax: default_lax_value,
        }
    );

    let updated = async_grouped_ignore_update_schema::DataModel
        .update(
            created.data.clone(),
            async_grouped_ignore_update_schema::PartialDataInput {
                lax: Some(30),
                virtual_field: Some("keep".into()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_grouped_ignore_update_schema::PartialData {
            dependent: Some(3),
            lax: Some(30),
        }
    );

    let failed = async_grouped_ignore_update_schema::DataModel
        .update(
            created.data.clone(),
            async_grouped_ignore_update_schema::PartialDataInput {
                lax: None,
                virtual_field: Some("virtual_value".into()),
            },
            (),
        )
        .await
        .err()
        .unwrap();

    assert!(failed.errors.is_none());
}

async_test_matrix!(should_respect_grouped_ignore_update_rule_on_virtual_fields_async);

// -----------------------------------------------------------------------------
// Grouped #[ignore_update([...], handler)] on an *aliased* virtual field.
// -----------------------------------------------------------------------------

#[test]
fn should_respect_grouped_ignore_update_rule_on_an_aliased_virtual_field() {
    let default_lax_value = 10;

    let created = sync_grouped_ignore_update_alias_schema::DataModel
        .create(
            sync_grouped_ignore_update_alias_schema::PartialDataInput {
                lax: None,
                virtual_alias: Some("virtual_value".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_grouped_ignore_update_alias_schema::Data {
            dependent: 2,
            lax: default_lax_value,
        }
    );

    let updated = sync_grouped_ignore_update_alias_schema::DataModel
        .update(
            created.data.clone(),
            sync_grouped_ignore_update_alias_schema::PartialDataInput {
                lax: Some(30),
                virtual_alias: Some("keep".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_grouped_ignore_update_alias_schema::PartialData {
            dependent: Some(3),
            lax: Some(30),
        }
    );

    let failed = sync_grouped_ignore_update_alias_schema::DataModel
        .update(
            created.data.clone(),
            sync_grouped_ignore_update_alias_schema::PartialDataInput {
                lax: None,
                virtual_alias: Some("virtual_value".into()),
            },
            (),
        )
        .err()
        .unwrap();

    assert!(failed.errors.is_none());
}

async fn should_respect_grouped_ignore_update_rule_on_an_aliased_virtual_field_async() {
    let default_lax_value = 10;

    let created = async_grouped_ignore_update_alias_schema::DataModel
        .create(
            async_grouped_ignore_update_alias_schema::PartialDataInput {
                lax: None,
                virtual_alias: Some("virtual_value".into()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_grouped_ignore_update_alias_schema::Data {
            dependent: 2,
            lax: default_lax_value,
        }
    );

    let updated = async_grouped_ignore_update_alias_schema::DataModel
        .update(
            created.data.clone(),
            async_grouped_ignore_update_alias_schema::PartialDataInput {
                lax: Some(30),
                virtual_alias: Some("keep".into()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_grouped_ignore_update_alias_schema::PartialData {
            dependent: Some(3),
            lax: Some(30),
        }
    );

    let failed = async_grouped_ignore_update_alias_schema::DataModel
        .update(
            created.data.clone(),
            async_grouped_ignore_update_alias_schema::PartialDataInput {
                lax: None,
                virtual_alias: Some("virtual_value".into()),
            },
            (),
        )
        .await
        .err()
        .unwrap();

    assert!(failed.errors.is_none());
}

async_test_matrix!(should_respect_grouped_ignore_update_rule_on_an_aliased_virtual_field_async);

// -----------------------------------------------------------------------------
// Field-level #[ignore(handler)] on virtual fields. Bare `#[ignore]` isn't a
// valid attribute (it must be conditional -- `#[ignore(|_, _| ...)]` or the
// async equivalents), unlike `#[ignore_update]`/`#[ignore_init]`, which do
// have bare forms; the closures below use an unconditional `true` (same
// convention `rs/`'s original test used) purely to isolate "is the field
// ever actually ignored" from any input-dependent branching. Applies to both
// `create` and `update`, unlike `#[ignore_update]` (update only) and
// `#[ignore_init]` (create only, below).
// -----------------------------------------------------------------------------

#[test]
fn should_respect_field_level_ignore_on_virtual_fields() {
    let created = sync_ignore_schema::DataModel
        .create(
            sync_ignore_schema::PartialDataInput {
                lax: None,
                virtual_field: Some("virtual_value".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_ignore_schema::Data {
            lax: 10,
            dependent: 1,
        }
    );

    let created = sync_ignore_schema::DataModel
        .create(
            sync_ignore_schema::PartialDataInput {
                lax: Some(20),
                virtual_field: Some("virtual_value".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_ignore_schema::Data {
            lax: 20,
            dependent: 1,
        }
    );

    let updated = sync_ignore_schema::DataModel
        .update(
            created.data.clone(),
            sync_ignore_schema::PartialDataInput {
                lax: Some(30),
                virtual_field: Some("virtual_value".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_ignore_schema::PartialData {
            lax: Some(30),
            dependent: None,
        }
    );

    let failed = sync_ignore_schema::DataModel
        .update(
            created.data.clone(),
            sync_ignore_schema::PartialDataInput {
                lax: None,
                virtual_field: Some("virtual_value".into()),
            },
            (),
        )
        .err()
        .unwrap();

    assert!(failed.errors.is_none());
}

#[test]
fn should_respect_field_level_ignore_on_an_aliased_virtual_field() {
    let created = sync_ignore_alias_schema::DataModel
        .create(
            sync_ignore_alias_schema::PartialDataInput {
                lax: None,
                virtual_alias: Some("virtual_value".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_ignore_alias_schema::Data {
            lax: 10,
            dependent: 1,
        }
    );

    let created = sync_ignore_alias_schema::DataModel
        .create(
            sync_ignore_alias_schema::PartialDataInput {
                lax: Some(20),
                virtual_alias: Some("virtual_value".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_ignore_alias_schema::Data {
            lax: 20,
            dependent: 1,
        }
    );

    let updated = sync_ignore_alias_schema::DataModel
        .update(
            created.data.clone(),
            sync_ignore_alias_schema::PartialDataInput {
                lax: Some(30),
                virtual_alias: Some("virtual_value".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_ignore_alias_schema::PartialData {
            lax: Some(30),
            dependent: None,
        }
    );

    let failed = sync_ignore_alias_schema::DataModel
        .update(
            created.data.clone(),
            sync_ignore_alias_schema::PartialDataInput {
                lax: None,
                virtual_alias: Some("virtual_value".into()),
            },
            (),
        )
        .err()
        .unwrap();

    assert!(failed.errors.is_none());
}

#[test]
fn should_respect_field_level_ignore_on_a_virtual_field_whose_alias_collides_with_a_dependent_field_name(
) {
    let created = sync_ignore_dependent_alias_schema::DataModel
        .create(
            sync_ignore_dependent_alias_schema::PartialDataInput {
                lax: None,
                dependent: Some("virtual_value".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_ignore_dependent_alias_schema::Data {
            lax: 10,
            dependent: 1,
        }
    );

    let created = sync_ignore_dependent_alias_schema::DataModel
        .create(
            sync_ignore_dependent_alias_schema::PartialDataInput {
                lax: Some(20),
                dependent: Some("virtual_value".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_ignore_dependent_alias_schema::Data {
            lax: 20,
            dependent: 1,
        }
    );

    let updated = sync_ignore_dependent_alias_schema::DataModel
        .update(
            created.data.clone(),
            sync_ignore_dependent_alias_schema::PartialDataInput {
                lax: Some(30),
                dependent: Some("virtual_value".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_ignore_dependent_alias_schema::PartialData {
            lax: Some(30),
            dependent: None,
        }
    );

    let failed = sync_ignore_dependent_alias_schema::DataModel
        .update(
            created.data.clone(),
            sync_ignore_dependent_alias_schema::PartialDataInput {
                lax: None,
                dependent: Some("virtual_value".into()),
            },
            (),
        )
        .err()
        .unwrap();

    assert!(failed.errors.is_none());
}

// -----------------------------------------------------------------------------
// Field-level bare #[ignore_init] on virtual fields (no closure form -- see
// GOAL.md's field-config table: "Resolver form is rejected"). Ignores the
// field during `create` only; `update` is unaffected, unlike `#[ignore]`
// above (both) and `#[ignore_update]` (update only). The two `update` calls
// below are both made against the *same* base record (from the second
// `create`, `dependent == 1`), not chained, matching `rs/`'s original.
// -----------------------------------------------------------------------------

#[test]
fn should_respect_field_level_ignore_init_on_virtual_fields() {
    let created = sync_ignore_init_schema::DataModel
        .create(
            sync_ignore_init_schema::PartialDataInput {
                lax: None,
                virtual_field: Some("virtual_value".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_ignore_init_schema::Data {
            lax: 10,
            dependent: 1,
        }
    );

    let created = sync_ignore_init_schema::DataModel
        .create(
            sync_ignore_init_schema::PartialDataInput {
                lax: Some(20),
                virtual_field: Some("virtual_value".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_ignore_init_schema::Data {
            lax: 20,
            dependent: 1,
        }
    );

    let updated = sync_ignore_init_schema::DataModel
        .update(
            created.data.clone(),
            sync_ignore_init_schema::PartialDataInput {
                lax: Some(30),
                virtual_field: Some("virtual_value".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_ignore_init_schema::PartialData {
            lax: Some(30),
            dependent: Some(created.data.dependent + 1),
        }
    );

    let updated = sync_ignore_init_schema::DataModel
        .update(
            created.data.clone(),
            sync_ignore_init_schema::PartialDataInput {
                lax: None,
                virtual_field: Some("virtual_value".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_ignore_init_schema::PartialData {
            lax: None,
            dependent: Some(created.data.dependent + 1),
        }
    );
}

#[test]
fn should_respect_field_level_ignore_init_on_an_aliased_virtual_field() {
    let created = sync_ignore_init_alias_schema::DataModel
        .create(
            sync_ignore_init_alias_schema::PartialDataInput {
                lax: None,
                virtual_alias: Some("virtual_value".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_ignore_init_alias_schema::Data {
            lax: 10,
            dependent: 1,
        }
    );

    let created = sync_ignore_init_alias_schema::DataModel
        .create(
            sync_ignore_init_alias_schema::PartialDataInput {
                lax: Some(20),
                virtual_alias: Some("virtual_value".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_ignore_init_alias_schema::Data {
            lax: 20,
            dependent: 1,
        }
    );

    let updated = sync_ignore_init_alias_schema::DataModel
        .update(
            created.data.clone(),
            sync_ignore_init_alias_schema::PartialDataInput {
                lax: Some(30),
                virtual_alias: Some("virtual_value".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_ignore_init_alias_schema::PartialData {
            lax: Some(30),
            dependent: Some(created.data.dependent + 1),
        }
    );

    let updated = sync_ignore_init_alias_schema::DataModel
        .update(
            created.data.clone(),
            sync_ignore_init_alias_schema::PartialDataInput {
                lax: None,
                virtual_alias: Some("virtual_value".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_ignore_init_alias_schema::PartialData {
            lax: None,
            dependent: Some(created.data.dependent + 1),
        }
    );
}

#[test]
fn should_respect_field_level_ignore_init_on_a_virtual_field_whose_alias_collides_with_a_dependent_field_name(
) {
    let created = sync_ignore_init_dependent_alias_schema::DataModel
        .create(
            sync_ignore_init_dependent_alias_schema::PartialDataInput {
                lax: None,
                dependent: Some("virtual_value".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_ignore_init_dependent_alias_schema::Data {
            lax: 10,
            dependent: 1,
        }
    );

    let created = sync_ignore_init_dependent_alias_schema::DataModel
        .create(
            sync_ignore_init_dependent_alias_schema::PartialDataInput {
                lax: Some(20),
                dependent: Some("virtual_value".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_ignore_init_dependent_alias_schema::Data {
            lax: 20,
            dependent: 1,
        }
    );

    let updated = sync_ignore_init_dependent_alias_schema::DataModel
        .update(
            created.data.clone(),
            sync_ignore_init_dependent_alias_schema::PartialDataInput {
                lax: Some(30),
                dependent: Some("virtual_value".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_ignore_init_dependent_alias_schema::PartialData {
            lax: Some(30),
            dependent: Some(created.data.dependent + 1),
        }
    );

    let updated = sync_ignore_init_dependent_alias_schema::DataModel
        .update(
            created.data.clone(),
            sync_ignore_init_dependent_alias_schema::PartialDataInput {
                lax: None,
                dependent: Some("virtual_value".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_ignore_init_dependent_alias_schema::PartialData {
            lax: None,
            dependent: Some(created.data.dependent + 1),
        }
    );
}

// -----------------------------------------------------------------------------
// The `_with_alias_same_as_dependent` collision variant was covered above for
// the two brand-new scenario types, but was still missing for the three
// scenario types already covered earlier in this file (field-level
// `#[ignore_update]`, grouped `#[ignore]`, grouped `#[ignore_update]`) --
// closing that gap too, one variant each, matching the alias-collides
// schema shape used everywhere else in this file.
// -----------------------------------------------------------------------------

#[test]
fn should_respect_field_level_ignore_update_on_a_virtual_field_whose_alias_collides_with_a_dependent_field_name(
) {
    let created = sync_ignore_update_dependent_alias_schema::DataModel
        .create(
            sync_ignore_update_dependent_alias_schema::PartialDataInput {
                lax: None,
                dependent: Some("virtual_value".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_ignore_update_dependent_alias_schema::Data {
            lax: 10,
            dependent: 2,
        }
    );

    let updated = sync_ignore_update_dependent_alias_schema::DataModel
        .update(
            created.data.clone(),
            sync_ignore_update_dependent_alias_schema::PartialDataInput {
                lax: Some(30),
                dependent: Some("new_virtual_value".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_ignore_update_dependent_alias_schema::PartialData {
            lax: Some(30),
            dependent: None,
        }
    );

    let failed = sync_ignore_update_dependent_alias_schema::DataModel
        .update(
            created.data.clone(),
            sync_ignore_update_dependent_alias_schema::PartialDataInput {
                lax: None,
                dependent: Some("ignored_value".into()),
            },
            (),
        )
        .err()
        .unwrap();

    assert!(failed.errors.is_none());
}

#[test]
fn should_respect_grouped_ignore_rule_on_a_virtual_field_whose_alias_collides_with_a_dependent_field_name(
) {
    let default_dependent_value = 1;
    let default_lax_value = 10;

    let created = sync_grouped_ignore_dependent_alias_schema::DataModel
        .create(
            sync_grouped_ignore_dependent_alias_schema::PartialDataInput {
                lax: None,
                dependent: Some("virtual_value".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_grouped_ignore_dependent_alias_schema::Data {
            dependent: default_dependent_value,
            lax: default_lax_value,
        }
    );

    let created = sync_grouped_ignore_dependent_alias_schema::DataModel
        .create(
            sync_grouped_ignore_dependent_alias_schema::PartialDataInput {
                lax: Some(20),
                dependent: Some("keep".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_grouped_ignore_dependent_alias_schema::Data {
            dependent: 2,
            lax: 20,
        }
    );

    let updated = sync_grouped_ignore_dependent_alias_schema::DataModel
        .update(
            created.data.clone(),
            sync_grouped_ignore_dependent_alias_schema::PartialDataInput {
                lax: Some(30),
                dependent: Some("keep".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_grouped_ignore_dependent_alias_schema::PartialData {
            dependent: Some(3),
            lax: Some(30),
        }
    );

    let failed = sync_grouped_ignore_dependent_alias_schema::DataModel
        .update(
            created.data.clone(),
            sync_grouped_ignore_dependent_alias_schema::PartialDataInput {
                lax: None,
                dependent: Some("virtual_value".into()),
            },
            (),
        )
        .err()
        .unwrap();

    assert!(failed.errors.is_none());
}

#[test]
fn should_respect_grouped_ignore_update_rule_on_a_virtual_field_whose_alias_collides_with_a_dependent_field_name(
) {
    let default_lax_value = 10;

    let created = sync_grouped_ignore_update_dependent_alias_schema::DataModel
        .create(
            sync_grouped_ignore_update_dependent_alias_schema::PartialDataInput {
                lax: None,
                dependent: Some("virtual_value".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_grouped_ignore_update_dependent_alias_schema::Data {
            dependent: 2,
            lax: default_lax_value,
        }
    );

    let updated = sync_grouped_ignore_update_dependent_alias_schema::DataModel
        .update(
            created.data.clone(),
            sync_grouped_ignore_update_dependent_alias_schema::PartialDataInput {
                lax: Some(30),
                dependent: Some("keep".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_grouped_ignore_update_dependent_alias_schema::PartialData {
            dependent: Some(3),
            lax: Some(30),
        }
    );

    let failed = sync_grouped_ignore_update_dependent_alias_schema::DataModel
        .update(
            created.data.clone(),
            sync_grouped_ignore_update_dependent_alias_schema::PartialDataInput {
                lax: None,
                dependent: Some("virtual_value".into()),
            },
            (),
        )
        .err()
        .unwrap();

    assert!(failed.errors.is_none());
}

// -----------------------------------------------------------------------------
// Schema definitions
// -----------------------------------------------------------------------------

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_ignore_update_schema {
    struct Fields {
        #[lax(10)]
        pub lax: i32,

        #[ivo_virtual]
        #[validate(|_, _, _| Ok(None))]
        #[ignore_update(|_, _| true)]
        pub virtual_field: String,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_ignore_update_schema {
    struct Fields {
        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[ivo_virtual]
        #[validate(async |_, _, _| Ok(None))]
        #[ignore_update(|_, _| true)]
        pub virtual_field: String,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_grouped_ignore_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(|_, _, _| Ok(None))]
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }

    #[ignore(["virtual_field", "lax"], |ctx, _| {
        ctx.input()
            .virtual_field
            .as_ref()
            .map(|v| v == "virtual_value")
            .unwrap_or(false)
    })]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_grouped_ignore_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(async |_, _, _| Ok(None))]
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }

    #[ignore(["virtual_field", "lax"], async |ctx, _| {
        ctx.input()
            .virtual_field
            .as_ref()
            .map(|v| v == "virtual_value")
            .unwrap_or(false)
    })]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_grouped_ignore_update_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(|_, _, _| Ok(None))]
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }

    #[ignore_update(["virtual_field", "lax"], |ctx, _| {
        ctx.input()
            .virtual_field
            .as_ref()
            .map(|v| v == "virtual_value")
            .unwrap_or(false)
    })]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_grouped_ignore_update_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(async |_, _, _| Ok(None))]
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }

    #[ignore_update(["virtual_field", "lax"], async |ctx, _| {
        ctx.input()
            .virtual_field
            .as_ref()
            .map(|v| v == "virtual_value")
            .unwrap_or(false)
    })]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_ignore_update_alias_schema {
    struct Fields {
        #[lax(10)]
        pub lax: i32,

        #[ivo_virtual("virtual_alias")]
        #[validate(|_, _, _| Ok(None))]
        #[ignore_update(|_, _| true)]
        pub virtual_field: String,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_ignore_update_alias_schema {
    struct Fields {
        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[ivo_virtual("virtual_alias")]
        #[validate(async |_, _, _| Ok(None))]
        #[ignore_update(|_, _| true)]
        pub virtual_field: String,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_grouped_ignore_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(|_, _, _| Ok(None))]
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }

    #[ignore(["virtual_field", "lax"], |ctx, _| {
        ctx.input()
            .virtual_alias
            .as_ref()
            .map(|v| v == "virtual_value")
            .unwrap_or(false)
    })]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_grouped_ignore_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(async |_, _, _| Ok(None))]
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }

    #[ignore(["virtual_field", "lax"], async |ctx, _| {
        ctx.input()
            .virtual_alias
            .as_ref()
            .map(|v| v == "virtual_value")
            .unwrap_or(false)
    })]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_grouped_ignore_update_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(|_, _, _| Ok(None))]
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }

    #[ignore_update(["virtual_field", "lax"], |ctx, _| {
        ctx.input()
            .virtual_alias
            .as_ref()
            .map(|v| v == "virtual_value")
            .unwrap_or(false)
    })]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_grouped_ignore_update_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(async |_, _, _| Ok(None))]
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }

    #[ignore_update(["virtual_field", "lax"], async |ctx, _| {
        ctx.input()
            .virtual_alias
            .as_ref()
            .map(|v| v == "virtual_value")
            .unwrap_or(false)
    })]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_ignore_schema {
    struct Fields {
        #[lax(10)]
        pub lax: i32,

        #[ivo_virtual]
        #[validate(|_, _, _| Ok(None))]
        #[ignore(|_, _| true)]
        pub virtual_field: String,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_ignore_alias_schema {
    struct Fields {
        #[lax(10)]
        pub lax: i32,

        #[ivo_virtual("virtual_alias")]
        #[validate(|_, _, _| Ok(None))]
        #[ignore(|_, _| true)]
        pub virtual_field: String,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_ignore_dependent_alias_schema {
    struct Fields {
        #[lax(10)]
        pub lax: i32,

        #[ivo_virtual("dependent")]
        #[validate(|_, _, _| Ok(None))]
        #[ignore(|_, _| true)]
        pub virtual_field: String,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_ignore_init_schema {
    struct Fields {
        #[lax(10)]
        pub lax: i32,

        #[ivo_virtual]
        #[validate(|_, _, _| Ok(None))]
        #[ignore_init]
        pub virtual_field: String,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_ignore_init_alias_schema {
    struct Fields {
        #[lax(10)]
        pub lax: i32,

        #[ivo_virtual("virtual_alias")]
        #[validate(|_, _, _| Ok(None))]
        #[ignore_init]
        pub virtual_field: String,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_ignore_init_dependent_alias_schema {
    struct Fields {
        #[lax(10)]
        pub lax: i32,

        #[ivo_virtual("dependent")]
        #[validate(|_, _, _| Ok(None))]
        #[ignore_init]
        pub virtual_field: String,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_ignore_update_dependent_alias_schema {
    struct Fields {
        #[lax(10)]
        pub lax: i32,

        #[ivo_virtual("dependent")]
        #[validate(|_, _, _| Ok(None))]
        #[ignore_update(|_, _| true)]
        pub virtual_field: String,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_grouped_ignore_dependent_alias_schema {
    struct Fields {
        #[ivo_virtual("dependent")]
        #[validate(|_, _, _| Ok(None))]
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }

    #[ignore(["virtual_field", "lax"], |ctx, _| {
        ctx.input()
            .dependent
            .as_ref()
            .map(|v| v == "virtual_value")
            .unwrap_or(false)
    })]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_grouped_ignore_update_dependent_alias_schema {
    struct Fields {
        #[ivo_virtual("dependent")]
        #[validate(|_, _, _| Ok(None))]
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }

    #[ignore_update(["virtual_field", "lax"], |ctx, _| {
        ctx.input()
            .dependent
            .as_ref()
            .map(|v| v == "virtual_value")
            .unwrap_or(false)
    })]
    const _: () = ();
}
