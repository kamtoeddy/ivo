use ivo::ivo_schema;

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod duplicate_virtual_alias {
    struct Fields {
        #[ivo_virtual("shared")]
        #[validate(|v: String, _, _| Ok(Some(v)))]
        pub virtual_a: String,

        #[ivo_virtual("shared")]
        #[validate(|v: String, _, _| Ok(Some(v)))]
        pub virtual_b: String,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod alias_same_as_own_field_name {
    struct Fields {
        #[ivo_virtual("virtual_field")]
        #[validate(|v: String, _, _| Ok(Some(v)))]
        pub virtual_field: String,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod alias_collides_with_required_field {
    struct Fields {
        #[required]
        pub name: String,

        #[ivo_virtual("name")]
        #[validate(|v: String, _, _| Ok(Some(v)))]
        pub virtual_field: String,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod alias_collides_with_unrelated_dependent_field {
    struct Fields {
        #[required]
        pub name: String,

        #[depends_on("name")]
        #[default(0)]
        #[resolve(|_, _| 0)]
        pub other: i32,

        #[ivo_virtual("other")]
        #[validate(|v: String, _, _| Ok(Some(v)))]
        pub virtual_field: String,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod alias_collides_with_default_created_at {
    struct Fields {
        #[required]
        pub name: String,

        #[created_at]
        pub created_at: String,

        #[ivo_virtual("created_at")]
        #[validate(|v: String, _, _| Ok(Some(v)))]
        pub virtual_field: String,
    }

    #[timestamps(|| String::from("timestamp"))]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod alias_collides_with_custom_created_at {
    struct Fields {
        #[required]
        pub name: String,

        #[created_at]
        pub custom_created_at: String,

        #[ivo_virtual("custom_created_at")]
        #[validate(|v: String, _, _| Ok(Some(v)))]
        pub virtual_field: String,
    }

    #[timestamps(|| String::from("timestamp"))]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod alias_collides_with_default_updated_at {
    struct Fields {
        #[required]
        pub name: String,

        #[updated_at]
        pub updated_at: String,

        #[ivo_virtual("updated_at")]
        #[validate(|v: String, _, _| Ok(Some(v)))]
        pub virtual_field: String,
    }

    #[timestamps(|| String::from("timestamp"))]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod alias_collides_with_custom_updated_at {
    struct Fields {
        #[required]
        pub name: String,

        #[updated_at]
        pub custom_updated_at: String,

        #[ivo_virtual("custom_updated_at")]
        #[validate(|v: String, _, _| Ok(Some(v)))]
        pub virtual_field: String,
    }

    #[timestamps(|| String::from("timestamp"))]
    const _: () = ();
}

fn main() {}
