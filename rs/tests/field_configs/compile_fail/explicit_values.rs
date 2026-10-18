use ivo::ivo_schema;

#[ivo_schema(input(DataInput))]
mod lax_without_default {
    struct Fields {
        #[lax]
        pub lax: i32,
    }
}

#[ivo_schema(input(DataInput))]
mod lax_with_empty_default {
    struct Fields {
        #[lax()]
        pub lax: i32,
    }
}

#[ivo_schema(input(DataInput), output(Data))]
mod constant_without_value {
    struct Fields {
        #[required]
        pub required: String,

        #[constant]
        pub constant: i32,
    }
}

#[ivo_schema(input(DataInput), output(Data))]
mod dependent_without_default {
    struct Fields {
        #[required]
        pub required: String,

        #[depends_on("required")]
        #[resolve(|_, _| 1)]
        pub dependent: i32,
    }
}

#[ivo_schema(input(DataInput), output(Data))]
mod timestamp_without_resolver {
    struct Fields {
        #[required]
        pub required: String,

        #[created_at]
        pub created_at: u64,
    }
}

fn main() {}
