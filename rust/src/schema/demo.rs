use crate::schema::properties::{
    base::IvoProperty, constants::ConstantField, dependents::DependentField, enums::EnumField,
    lax::LaxField, virtuals::VirtualField,
};

impl<I, O> IvoProperty<I, O> {
    pub fn constant() -> ConstantField {
        ConstantField
    }

    pub fn dependent() -> DependentField {
        DependentField
    }

    pub fn enum_field() -> EnumField {
        EnumField
    }

    pub fn lax() -> LaxField {
        LaxField
    }

    pub fn virtual_field() -> VirtualField {
        VirtualField
    }
}

// fn main() {
//     let l = LaxField::default("&str")
//         .validate(Box::new(|v, _| Ok("true")))
//         .on_delete(Box::new(|_| {}))
//         .on_failure(Box::new(|_| {}))
//         .on_success(Box::new(|_| {}))
//         .build();

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
//         .resolve(Box::new(|_| resolver()))
//         .on_delete(Box::new(|_| {}))
//         .on_success(Box::new(|_| {}))
//         .build();

//     let d = DependentField::default_fn(Box::new(|_| true))
//         .depends_on(&["first_name", "last_name"])
//         .resolve(Box::new(|_| false))
//         .readonly()
//         .on_delete_fns(vec![Box::new(|_| {}), Box::new(|_| {})])
//         .on_success(Box::new(|_| {}))
//         .build();

//     let v = VirtualField::alias("lol")
//         .validate(Box::new(|v, _| Ok(true)))
//         .re_validate(Box::new(|v, c| Ok(true)))
//         .required_if(Box::new(|_| (true, "lol")))
//         .sanitize(Box::new(|s| false))
//         .on_failure(Box::new(|_| {}))
//         .on_success(Box::new(|_| {}))
//         .build();

//     let v = VirtualField::validate(Box::new(|v, _| Ok(true)))
//         .re_validate(Box::new(|v, c| Ok(true)))
//         .alias("lol")
//         .required_if(Box::new(|_| (true, "lol")))
//         .sanitize(Box::new(|s| false))
//         .on_failure(Box::new(|_| {}))
//         .on_success(Box::new(|_| {}))
//         .build();

//     let v = VirtualField::validate(Box::new(|v, _| Ok(true)))
//         .alias("lol")
//         .re_validate(Box::new(|v, c| Ok(true)))
//         .required_if(Box::new(|_| (true, "lol")))
//         .sanitize(Box::new(|_| false))
//         // .ignore_if(Box::new(|_| false))
//         .allow_update_if(Box::new(|_| false))
//         .allow_init_if(Box::new(|_| false))
//         // .ignore_init()
//         // .ignore_update()
//         .on_failure(Box::new(|_| {}))
//         .on_success(Box::new(|_| {}))
//         .build();
// }
