use ivo::ivo_schema;

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod bare_ident_depends_on {
    struct Fields {
        #[required]
        pub name: String,

        #[depends_on(name)]
        #[default(String::new())]
        #[resolve(|_, _| String::new())]
        pub dependent: String,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod bare_ident_ivo_virtual_alias {
    struct Fields {
        #[ivo_virtual(alias)]
        #[validate(|v: String, _, _| Ok(Some(v)))]
        pub virtual_field: String,

        #[depends_on("virtual_field")]
        #[default(String::new())]
        #[resolve(|_, _| String::new())]
        pub dependent: String,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod empty_parents {
    struct Fields {
        #[required]
        pub name: String,

        #[depends_on()]
        #[default(String::new())]
        #[resolve(|_, _| String::new())]
        pub dependent: String,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod self_dependency {
    struct Fields {
        #[required]
        pub name: String,

        #[depends_on("dependent")]
        #[default(String::new())]
        #[resolve(|_, _| String::new())]
        pub dependent: String,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod unknown_parent {
    struct Fields {
        #[required]
        pub name: String,

        #[depends_on("ghost")]
        #[default(String::new())]
        #[resolve(|_, _| String::new())]
        pub dependent: String,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod duplicate_parent {
    struct Fields {
        #[required]
        pub name: String,

        #[depends_on("name", "name")]
        #[default(String::new())]
        #[resolve(|_, _| String::new())]
        pub dependent: String,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod dependency_on_timestamp {
    struct Fields {
        #[required]
        pub name: String,

        #[created_at]
        pub created_at: String,

        #[depends_on("created_at")]
        #[default(String::new())]
        #[resolve(|_, _| String::new())]
        pub dependent: String,
    }

    #[timestamps(|| String::from("timestamp"))]
    const _: () = ();
}

fn main() {}
