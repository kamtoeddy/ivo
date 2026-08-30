use ivo::ivo_schema;

// should_reject_if_the_fields_array_contains_any_duplicates
#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod on_success_duplicates {
    struct Fields {
        #[lax(1234)]
        pub lax: i32,

        #[lax(5678)]
        pub lax_1: i32,
    }

    #[on_success(["lax", "lax"], |_ctx, _opts| {})]
    const _: () = ();
}

// should_reject_if_the_fields_array_contains_any_string_that_is_not_a_field_on_schema
#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod on_success_invalid_field {
    struct Fields {
        #[lax(1234)]
        pub lax: i32,

        #[lax(5678)]
        pub lax_1: i32,
    }

    #[on_success(["lax", "invalid_field"], |_ctx, _opts| {})]
    const _: () = ();
}

// should_reject_if_an_alias_with_foreign_name_is_provided_to_the_fields_array
#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod on_success_alias_foreign_name {
    struct Fields {
        #[lax(1234)]
        pub lax: i32,

        #[lax(5678)]
        pub lax_1: i32,

        #[depends_on("lax", "virtual_field")]
        #[default(1)]
        #[resolve(|_, _| 2)]
        pub dependent: i32,

        #[ivo_virtual("alias")]
        #[validate(|_, _, _| Ok(None))]
        pub virtual_field: i32,
    }

    #[on_success(["lax", "lax_1", "alias"], |_ctx, _opts| {})]
    const _: () = ();
}

// should_reject_an_empty_fields_array: `#[on_success([...], handler)]` still
// requires at least one field; "always fire" is spelled only via the bare,
// arrayless `#[on_success(handler)]` entity-level form.
#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod on_success_empty_fields_array {
    struct Fields {
        #[lax(1234)]
        pub lax: i32,
    }

    #[on_success([], |_ctx, _opts| {})]
    const _: () = ();
}

fn main() {}
