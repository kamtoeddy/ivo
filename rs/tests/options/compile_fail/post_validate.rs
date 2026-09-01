use ivo::ivo_schema;

// should_reject_if_fields_array_is_empty
#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod post_validate_empty {
    struct Fields {
        #[lax(1234)]
        pub lax: i32,

        #[lax(5678)]
        pub lax_1: i32,
    }

    #[post_validate([], validate = |_, _| Ok(None))]
    const _: () = ();
}

// should_reject_if_fields_array_has_just_one_field
#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod post_validate_one_field {
    struct Fields {
        #[lax(1234)]
        pub lax: i32,

        #[lax(5678)]
        pub lax_1: i32,
    }

    #[post_validate(["lax"], validate = |_, _| Ok(None))]
    const _: () = ();
}

// should_reject_if_the_fields_array_contains_any_duplicates
#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod post_validate_duplicates {
    struct Fields {
        #[lax(1234)]
        pub lax: i32,

        #[lax(5678)]
        pub lax_1: i32,
    }

    #[post_validate(["lax", "lax"], validate = |_, _| Ok(None))]
    const _: () = ();
}

// should_reject_if_the_fields_array_contains_any_string_that_is_not_a_field_on_schema
#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod post_validate_invalid_field {
    struct Fields {
        #[lax(1234)]
        pub lax: i32,

        #[lax(5678)]
        pub lax_1: i32,
    }

    #[post_validate(["lax", "invalid_field"], validate = |_, _| Ok(None))]
    const _: () = ();
}

// should_reject_if_a_constant_is_provided_to_the_fields_array
#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod post_validate_constant {
    struct Fields {
        #[constant(1234)]
        pub id: i32,

        #[lax(1234)]
        pub lax: i32,

        #[lax(5678)]
        pub lax_1: i32,
    }

    #[post_validate(["lax", "id"], validate = |_, _| Ok(None))]
    const _: () = ();
}

// should_reject_if_a_dependent_field_is_provided_to_the_fields_array
#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod post_validate_dependent {
    struct Fields {
        #[lax(1234)]
        pub lax: i32,

        #[lax(5678)]
        pub lax_1: i32,

        #[depends_on("lax", "lax_1")]
        #[default(1)]
        #[resolve(|_, _| 2)]
        pub dependent: i32,
    }

    #[post_validate(["lax", "lax_1", "dependent"], validate = |_, _| Ok(None))]
    const _: () = ();
}

// should_reject_if_an_alias_similar_to_a_dependent_field_is_provided_to_the_fields_array
#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod post_validate_alias_similar_to_dependent {
    struct Fields {
        #[lax(1234)]
        pub lax: i32,

        #[lax(5678)]
        pub lax_1: i32,

        #[depends_on("lax", "virtual_field")]
        #[default(1)]
        #[resolve(|_, _| 2)]
        pub dependent: i32,

        #[ivo_virtual("dependent")]
        #[validate(|_, _, _| Ok(None))]
        pub virtual_field: i32,
    }

    #[post_validate(["lax", "lax_1", "dependent"], validate = |_, _| Ok(None))]
    const _: () = ();
}

// should_reject_if_an_alias_with_foreign_name_is_provided_to_the_fields_array
#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod post_validate_alias_foreign_name {
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

    #[post_validate(["lax", "lax_1", "alias"], validate = |_, _| Ok(None))]
    const _: () = ();
}

// should_reject_if_created_at_timestamp_with_default_name_is_provided_to_the_fields_array
#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod post_validate_created_at_default {
    struct Fields {
        #[lax(1234)]
        pub lax: i32,

        #[lax(5678)]
        pub lax_1: i32,

        #[created_at]
        pub created_at: String,
    }

    #[timestamps(|| String::from("timestamp"))]
    const _: () = ();

    #[post_validate(["lax", "lax_1", "created_at"], validate = |_, _| Ok(None))]
    const _: () = ();
}

// should_reject_if_created_at_timestamp_with_custom_name_is_provided_to_the_fields_array
#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod post_validate_created_at_custom {
    struct Fields {
        #[lax(1234)]
        pub lax: i32,

        #[lax(5678)]
        pub lax_1: i32,

        #[created_at]
        pub custom_created_at: String,
    }

    #[timestamps(|| String::from("timestamp"))]
    const _: () = ();

    #[post_validate(["lax", "lax_1", "custom_created_at"], validate = |_, _| Ok(None))]
    const _: () = ();
}

// should_reject_if_updated_at_timestamp_with_default_name_is_provided_to_the_fields_array
#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod post_validate_updated_at_default {
    struct Fields {
        #[lax(1234)]
        pub lax: i32,

        #[lax(5678)]
        pub lax_1: i32,

        #[updated_at]
        pub updated_at: String,
    }

    #[timestamps(|| String::from("timestamp"))]
    const _: () = ();

    #[post_validate(["lax", "lax_1", "updated_at"], validate = |_, _| Ok(None))]
    const _: () = ();
}

// should_reject_if_updated_at_timestamp_with_custom_name_is_provided_to_the_fields_array
#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod post_validate_updated_at_custom {
    struct Fields {
        #[lax(1234)]
        pub lax: i32,

        #[lax(5678)]
        pub lax_1: i32,

        #[updated_at]
        pub custom_updated_at: String,
    }

    #[timestamps(|| String::from("timestamp"))]
    const _: () = ();

    #[post_validate(["lax", "lax_1", "custom_updated_at"], validate = |_, _| Ok(None))]
    const _: () = ();
}

fn main() {}
