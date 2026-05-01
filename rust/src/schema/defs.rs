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

//     let d = DependentField::default(2)
//         .depends_on(parents)
//         .resolver(resolver)
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
