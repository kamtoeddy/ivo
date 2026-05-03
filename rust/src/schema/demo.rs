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
//     let c = ConstantField::value("&str")
//         .on_success(Box::new(|_| {}))
//         .on_delete(Box::new(|_| {}))
//         .build();

//     let c = ConstantField::value(String::from("String"))
//         .on_success(Box::new(|_| {}))
//         .on_delete(Box::new(|_| {}))
//         .build();

//     let c = ConstantField::value(Some(String::from("Option<String>")))
//         .on_success(Box::new(|_| {}))
//         .on_delete_fns(vec![Box::new(|_| {})])
//         .build();

//     let c = ConstantField::computed(Box::new(|s| "computed &str"))
//         .on_delete(Box::new(|_| {}))
//         .on_success_fns(vec![Box::new(|_| {}), Box::new(|_| {})])
//         .build();

//     let resolver = || String::from("full name");

//     let d = DependentField::default(String::from("Hello"))
//         .depends_on(&["first_name", "last_name"])
//         .resolver(Box::new(|_| resolver()))
//         .on_delete(Box::new(|_| {}))
//         .on_success(Box::new(|_| {}))
//         .build();

//     let d = DependentField::default_fn(Box::new(|_| true))
//         .depends_on(&["first_name", "last_name"])
//         .resolver(Box::new(|_| false))
//         .readonly()
//         .on_delete_fns(vec![Box::new(|_| {}), Box::new(|_| {})])
//         .on_success(Box::new(|_| {}))
//         .build();

//     let v = VirtualField::alias("lol")
//         .validator(Box::new(|v, _| Ok(true)))
//         .re_validator(Box::new(|v, c| Ok(true)))
//         .required(Box::new(|_| (true, "lol")))
//         .sanitizer(Box::new(|s| false))
//         .on_failure(Box::new(|_| {}))
//         .on_success(Box::new(|_| {}))
//         .build();

//     let v = VirtualField::validator(Box::new(|v, _| Ok(true)))
//         .re_validator(Box::new(|v, c| Ok(true)))
//         .alias("lol")
//         .required(Box::new(|_| (true, "lol")))
//         .sanitizer(Box::new(|s| false))
//         .on_failure(Box::new(|_| {}))
//         .on_success(Box::new(|_| {}))
//         .build();

//     let v = VirtualField::validator(Box::new(|v, _| Ok(true)))
//         .alias("lol")
//         .re_validator(Box::new(|v, c| Ok(true)))
//         .required(Box::new(|_| (true, "lol")))
//         .sanitizer(Box::new(|s| false))
//         .on_failure(Box::new(|_| {}))
//         .on_success(Box::new(|_| {}))
//         .build();
// }
