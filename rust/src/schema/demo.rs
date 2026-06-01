use crate::{
    schema::properties::{
        base::IvoProperty, constants::ConstantField, dependents::DependentField,
        enumerated::EnumeratedField, lax::LaxField, required::RequiredField,
        virtuals::VirtualField,
    },
    traits::IvoSchemaStruct,
};

impl<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions> IvoProperty<T, I, O, CtxOptions> {
    pub fn constant() -> ConstantField {
        ConstantField
    }

    pub fn dependent() -> DependentField {
        DependentField
    }

    pub fn enumerated() -> EnumeratedField {
        EnumeratedField
    }

    pub fn lax() -> LaxField {
        LaxField
    }

    pub fn required() -> RequiredField {
        RequiredField
    }

    pub fn virtual_field() -> VirtualField {
        VirtualField
    }
}

// fn main() {
//     let r = RequiredField::validate(|v, _| Err(("lol", None)))
//         .re_validate(|v, c| Ok(true))
//         .readonly()
//         .on_failure(|_| async {})
//         .on_success(|_| async {})
//         .on_delete(|_, __| async {})
//         .build();

//     let l = LaxField::default("&str")
//         .validate(|v, _| Ok("true"))
//         .readonly()
//         .on_delete(|_, __| async {})
//         .on_failure(|_| async {})
//         .on_success(|_| async {})
//         .build();

//     let c = ConstantField::value("&str")
//         .on_success(|_| async {})
//         .on_delete(|_, __| async {})
//         .build();

//     let v = VirtualField::alias("lol")
//         .validate(|v, _| Ok(true))
//         .re_validate_async(|v, c| async { Ok(true) })
//         .required_if(|_| async { (true, "lol") })
//         .sanitize(|s| false)
//         .on_failure(|_| async {})
//         .on_success(|_| async {})
//         .build();

//     let v = VirtualField::validate_async(|v, _| async {
//         if true {
//             Ok(true)
//         } else {
//             Err(("lol", None))
//         }
//     })
//     .re_validate(|v, c| Ok(true))
//     .alias("lol")
//     .required_if(|_| async { (true, "lol") })
//     .sanitize(|s| false)
//     .on_failure(|_| async {})
//     .on_success(|_| async {})
//     .build();

//     let v = VirtualField::validate(|v, _| Ok(true))
//         .re_validate(|v, c| Ok(true))
//         .alias("lol")
//         .required_if(|_| async { (true, "lol") })
//         .sanitize(|s| false)
//         .on_failure(Box::new(|_| async {}))
//         .on_success(|_| async {})
//         .build();

//     let v = VirtualField::validate(|v, _| Ok(true))
//         .alias("lol")
//         .re_validate(|v, c| Ok(true))
//         .required_if(|_| async { (true, "lol") })
//         .sanitize(|_| false)
//         // .ignore_if(|_| false)
//         .allow_update_if(|_| false)
//         .allow_init_if(|_| false)
//         // .ignore_init()
//         // .ignore_update()
//         .on_failure(|_| async {})
//         .on_failure(|_| async {})
//         .on_success(|_| async { println!("on success 1") })
//         .on_success(|_| async { println!("on success 2") })
//         .build();
// }
