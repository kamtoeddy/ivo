use ivo::ivo_schema;

fn default_name() -> String {
    String::from("default")
}

fn default_dependent() -> i32 {
    1
}

#[ivo_schema(input(DataInput, ivo_attrs(derive(Debug, Clone, PartialEq))))]
mod readonly_lax_with_fn_path_default {
    struct Fields {
        #[lax(super::default_name)]
        #[readonly]
        pub name: String,
    }
}

#[ivo_schema(
    input(DataInput, ivo_attrs(derive(Debug, Clone, PartialEq))),
    output(Data, ivo_attrs(derive(Debug, Clone, PartialEq)))
)]
mod readonly_dependent_with_fn_path_default {
    struct Fields {
        #[depends_on("lax")]
        #[default(super::default_dependent)]
        #[resolve(|_, _| 2)]
        #[readonly]
        pub dependent: i32,

        #[lax(String::new())]
        pub lax: String,
    }
}

fn main() {}
