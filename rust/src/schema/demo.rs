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

    pub fn virtual_prop() -> VirtualField {
        VirtualField
    }
}

// fn main() {
//     let c = ConstantField::value(Some(String::from("hello")))
//         .on_delete(handler)
//         .on_success(handler)
//         .build();

//     let resolver = |_| String::from("full name");

//     let d = DependentField::default(String::from("Hello"))
//         .depends_on(&["first_name", "last_name"])
//         .resolver(Box::new(|s| resolver(s)))
//         .on_delete(handler)
//         .on_success(handler)
//         .build();

//     let required = |_| (true, "lol");

//     let v = VirtualField::validator(Box::new(|v, c| Ok(true)))
//         .re_validator(Box::new(|v, c| Ok(true)))
//         .alias("lol")
//         .required(Box::new(|c| required(c)))
//         .sanitizer(Box::new(|s| false))
//         .on_failure(Box::new(|s| {}))
//         .on_success(Box::new(|s| {}))
//         .build();
// }
