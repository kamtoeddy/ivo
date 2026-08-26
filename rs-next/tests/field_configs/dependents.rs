use ivo::ivo_schema;

#[test]
fn should_allow_dependency_on_normal_lax_or_required_fields() {
    // These schemas compile only if dependent fields are permitted to depend on
    // lax and/or required fields.
    let _ = dependent_on_lax_schema::DataModel;
    let _ = dependent_on_required_schema::DataModel;
    let _ = dependent_on_both_schema::DataModel;
}

#[test]
fn should_allow_dependency_on_other_dependent_fields() {
    let _ = dependent_on_dependent_schema::DataModel;
    let _ = dependent_on_dependent_and_required_schema::DataModel;
}

#[test]
fn should_allow_dependency_on_virtual_fields() {
    let _ = dependent_on_virtual_schema::DataModel;
    let _ = dependent_on_virtual_and_required_schema::DataModel;
    let _ = dependent_on_dependent_and_virtual_schema::DataModel;
    let _ = chained_dependent_on_virtual_schema::DataModel;
}

#[test]
fn should_allow_dependency_on_virtual_fields_with_aliases() {
    let _ = virtual_alias_schema::DataModel;
    let _ = virtual_alias_matching_dependent_schema::DataModel;
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod dependent_on_lax_schema {
    struct Fields {
        #[depends_on(lax)]
        #[default(2)]
        #[resolve(|_, _| 4)]
        pub dependent: i32,

        #[lax(String::from("default"))]
        pub lax: String,

        #[required]
        #[validate(|v, _, _| Ok(Some(v)))]
        pub required: String,

        #[updated_at]
        pub updated_at: String,
    }

    #[timestamps(|| String::from("timestamp"))]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod dependent_on_required_schema {
    struct Fields {
        #[depends_on(required)]
        #[default(2)]
        #[resolve(|_, _| 4)]
        pub dependent: i32,

        #[lax(String::from("default"))]
        pub lax: String,

        #[required]
        #[validate(|v, _, _| Ok(Some(v)))]
        pub required: String,

        #[updated_at]
        pub updated_at: String,
    }

    #[timestamps(|| String::from("timestamp"))]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod dependent_on_both_schema {
    struct Fields {
        #[depends_on(lax, required)]
        #[default(2)]
        #[resolve(|_, _| 4)]
        pub dependent: i32,

        #[lax(String::from("default"))]
        pub lax: String,

        #[required]
        #[validate(|v, _, _| Ok(Some(v)))]
        pub required: String,

        #[updated_at]
        pub updated_at: String,
    }

    #[timestamps(|| String::from("timestamp"))]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod dependent_on_dependent_schema {
    struct Fields {
        #[depends_on(dependent1)]
        #[default(2)]
        #[resolve(|_, _| 4)]
        pub dependent: i32,

        #[depends_on(lax)]
        #[default(2)]
        #[resolve(|_, _| 4)]
        pub dependent1: i32,

        #[lax(String::from("default"))]
        pub lax: String,

        #[required]
        #[validate(|v, _, _| Ok(Some(v)))]
        pub required: String,

        #[updated_at]
        pub updated_at: String,
    }

    #[timestamps(|| String::from("timestamp"))]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod dependent_on_dependent_and_required_schema {
    struct Fields {
        #[depends_on(dependent1, required)]
        #[default(2)]
        #[resolve(|_, _| 4)]
        pub dependent: i32,

        #[depends_on(lax)]
        #[default(2)]
        #[resolve(|_, _| 4)]
        pub dependent1: i32,

        #[lax(String::from("default"))]
        pub lax: String,

        #[required]
        #[validate(|v, _, _| Ok(Some(v)))]
        pub required: String,

        #[updated_at]
        pub updated_at: String,
    }

    #[timestamps(|| String::from("timestamp"))]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod dependent_on_virtual_schema {
    struct Fields {
        #[depends_on(virtual_field)]
        #[default(2)]
        #[resolve(|_, _| 4)]
        pub dependent: i32,

        #[depends_on(lax)]
        #[default(2)]
        #[resolve(|_, _| 4)]
        pub dependent1: i32,

        #[lax(String::from("default"))]
        pub lax: String,

        #[required]
        #[validate(|v, _, _| Ok(Some(v)))]
        pub required: String,

        #[ivo_virtual]
        #[validate(|v, _, _| Ok(Some(v)))]
        pub virtual_field: String,

        #[updated_at]
        pub updated_at: String,
    }

    #[timestamps(|| String::from("timestamp"))]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod dependent_on_virtual_and_required_schema {
    struct Fields {
        #[depends_on(required, virtual_field)]
        #[default(2)]
        #[resolve(|_, _| 4)]
        pub dependent: i32,

        #[depends_on(lax)]
        #[default(2)]
        #[resolve(|_, _| 4)]
        pub dependent1: i32,

        #[lax(String::from("default"))]
        pub lax: String,

        #[required]
        #[validate(|v, _, _| Ok(Some(v)))]
        pub required: String,

        #[ivo_virtual]
        #[validate(|v, _, _| Ok(Some(v)))]
        pub virtual_field: String,

        #[updated_at]
        pub updated_at: String,
    }

    #[timestamps(|| String::from("timestamp"))]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod dependent_on_dependent_and_virtual_schema {
    struct Fields {
        #[depends_on(dependent1, virtual_field)]
        #[default(2)]
        #[resolve(|_, _| 4)]
        pub dependent: i32,

        #[depends_on(lax)]
        #[default(2)]
        #[resolve(|_, _| 4)]
        pub dependent1: i32,

        #[lax(String::from("default"))]
        pub lax: String,

        #[required]
        #[validate(|v, _, _| Ok(Some(v)))]
        pub required: String,

        #[ivo_virtual]
        #[validate(|v, _, _| Ok(Some(v)))]
        pub virtual_field: String,

        #[updated_at]
        pub updated_at: String,
    }

    #[timestamps(|| String::from("timestamp"))]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod chained_dependent_on_virtual_schema {
    struct Fields {
        #[depends_on(virtual_field)]
        #[default(2)]
        #[resolve(|_, _| 4)]
        pub dependent: i32,

        #[depends_on(lax, virtual_field)]
        #[default(2)]
        #[resolve(|_, _| 4)]
        pub dependent1: i32,

        #[lax(String::from("default"))]
        pub lax: String,

        #[required]
        #[validate(|v, _, _| Ok(Some(v)))]
        pub required: String,

        #[ivo_virtual]
        #[validate(|v, _, _| Ok(Some(v)))]
        pub virtual_field: String,

        #[updated_at]
        pub updated_at: String,
    }

    #[timestamps(|| String::from("timestamp"))]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod virtual_alias_schema {
    struct Fields {
        #[depends_on(virtual_field)]
        #[default(2)]
        #[resolve(|_, _| 4)]
        pub dependent: i32,

        #[lax(String::from("default"))]
        pub lax: String,

        #[required]
        #[validate(|v, _, _| Ok(Some(v)))]
        pub required: String,

        #[ivo_virtual(alias = "alias_name")]
        #[validate(|v, _, _| Ok(Some(v)))]
        pub virtual_field: String,

        #[updated_at]
        pub updated_at: String,
    }

    #[timestamps(|| String::from("timestamp"))]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod virtual_alias_matching_dependent_schema {
    struct Fields {
        #[depends_on(virtual_field)]
        #[default(2)]
        #[resolve(|_, _| 4)]
        pub dependent: i32,

        #[lax(String::from("default"))]
        pub lax: String,

        #[required]
        #[validate(|v, _, _| Ok(Some(v)))]
        pub required: String,

        #[ivo_virtual(alias = "dependent")]
        #[validate(|v, _, _| Ok(Some(v)))]
        pub virtual_field: String,

        #[updated_at]
        pub updated_at: String,
    }

    #[timestamps(|| String::from("timestamp"))]
    const _: () = ();
}
