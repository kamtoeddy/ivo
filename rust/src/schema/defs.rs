use crate::schema::properties::{
    base::IvoProperty, constants::ConstantField, dependents::DependentField, virtuals::VirtualField,
};

impl<I, O> IvoProperty<I, O> {
    pub fn constant() -> ConstantField {
        ConstantField
    }

    pub fn dependent() -> DependentField {
        DependentField
    }

    pub fn virtual_prop() -> Self {
        todo!()
    }
}

// fn main() {
//     let c = ConstantField::value(Some(String::from("hello")))
//         .on_delete(handler)
//         .on_success(handler)
//         .build();

//     let d = DependentField::default(String::from("Hello"))
//         .depends_on(vec!["first_name", "last_name"])
//         .resolver(Box::new(|s| String::from("full name")))
//         .on_delete(handler)
//         .on_success(handler)
//         .build();

//     let v = VirtualField::validate(validator)
//         .re_validate(re_validator)
//         .alias("lol")
//         .sanitizer(sanitizer)
//         .on_failure(handler)
//         .on_success(handler)
//         .build();
// }
