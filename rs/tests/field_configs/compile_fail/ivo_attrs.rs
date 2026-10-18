use ivo::ivo_schema;

#[ivo_schema(input(DataInput), output(Data))]
mod removed_input_field_attr {
    struct Fields {
        #[required]
        #[input(doc = "x")]
        pub required: String,

        #[constant(1)]
        pub constant: i32,
    }
}

#[ivo_schema(input(DataInput), output(Data))]
mod removed_output_field_attr {
    struct Fields {
        #[required]
        #[output(doc = "x")]
        pub required: String,

        #[constant(1)]
        pub constant: i32,
    }
}

#[ivo_schema(input(DataInput), output(Data))]
mod removed_partial_field_attr {
    struct Fields {
        #[required]
        #[partial(doc = "x")]
        pub required: String,

        #[constant(1)]
        pub constant: i32,
    }
}

#[ivo_schema(input(DataInput), output(Data))]
mod removed_input_partial_field_attr {
    struct Fields {
        #[required]
        #[input_partial(doc = "x")]
        pub required: String,

        #[constant(1)]
        pub constant: i32,
    }
}

#[ivo_schema(input(DataInput), output(Data))]
mod removed_output_partial_field_attr {
    struct Fields {
        #[required]
        #[output_partial(doc = "x")]
        pub required: String,

        #[constant(1)]
        pub constant: i32,
    }
}

#[ivo_schema(input(DataInput), output(Data))]
mod input_attrs_on_output_only_field {
    struct Fields {
        #[required]
        pub required: String,

        #[constant(1)]
        #[ivo_input_attrs(doc = "x")]
        pub constant: i32,
    }
}

#[ivo_schema(input(DataInput), output(Data))]
mod input_partial_attrs_on_output_only_field {
    struct Fields {
        #[required]
        pub required: String,

        #[constant(1)]
        #[ivo_input_partial_attrs(doc = "x")]
        pub constant: i32,
    }
}

#[ivo_schema(input(DataInput), output(Data))]
mod output_attrs_on_virtual_field {
    struct Fields {
        #[ivo_virtual]
        #[ivo_output_attrs(doc = "x")]
        pub virtual_field: String,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|_, _| 2)]
        pub dependent: i32,
    }
}

#[ivo_schema(input(DataInput))]
mod output_partial_attrs_without_output {
    struct Fields {
        #[required]
        #[ivo_output_partial_attrs(doc = "x")]
        pub required: String,
    }
}

#[ivo_schema(input(DataInput))]
mod ivo_attrs_without_attributes {
    struct Fields {
        #[required]
        #[ivo_attrs]
        pub required: String,
    }
}

#[ivo_schema(input(DataInput, ivo_attrs(derive(Debug), serde = )))]
mod malformed_struct_ivo_attrs {
    struct Fields {
        #[required]
        pub required: String,
    }
}

fn main() {}
