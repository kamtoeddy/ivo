#![expect(dead_code)]

use ivo::{IvoField, IvoRwCtxOptions, IvoStruct, Model, Schema};
use std::{array, collections::HashMap, future::ready, sync::LazyLock};

use crate::async_test_matrix;

mod constants;
mod dependents;
mod lax;

// [x] constants
// [x] dependents
// [x] lax
// [ ] required
// [ ] virtuals
//
// [ ] validator
// [ ] revalidator
// [ ] sanitizer
// [ ] dependent_resolver
// [ ] post_validator

async fn should_properly_update_ctx_options() {
    let supplier_num = 2;

    let (data, ctx_options, _) = PRODUCT_MODEL
        .create(
            &PartialProductInput {
                name: Some("product_name".into()),
                price: Some(1_000),
                sku: Some("product_sku".into()),
                supplier: Some(SupplierID(supplier_num)),
            },
            ProductCtxOptions::new(),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        ctx_options.warnings[0],
        format!("warning: supplier {supplier_num} is not currently active!")
    );

    let supplier_num = 3;

    let (_, ctx_options, _) = PRODUCT_MODEL
        .update(
            &data,
            &PartialProductInput {
                name: None,
                price: None,
                sku: None,
                supplier: Some(SupplierID(supplier_num)),
            },
            ProductCtxOptions::new(),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        ctx_options.warnings[0],
        format!("warning: supplier {supplier_num} is not currently active!")
    );
}

async_test_matrix!(should_properly_update_ctx_options);

#[derive(Clone)]
struct ProductCtxOptions {
    warnings: Vec<String>,
}

impl ProductCtxOptions {
    fn new() -> Self {
        Self { warnings: vec![] }
    }

    fn add_warning(&mut self, warning: &str) {
        self.warnings.push(warning.to_owned());
    }

    async fn get_supplier_by_id(&self, id: &SupplierID) -> Option<Supplier> {
        SUPPLIERS_DB.get(id).cloned()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ProductID(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct SupplierID(u64);

#[derive(Debug, Clone, PartialEq, IvoStruct)]
pub struct Product {
    id: ProductID,
    name: String,
    sku: String,
    price: u32,
    supplier: SupplierID,
}

#[derive(Debug, Clone, PartialEq, IvoStruct)]
pub struct ProductInput {
    name: String,
    sku: String,
    price: u32,
    supplier: SupplierID,
}

#[derive(Debug, Clone)]
pub enum SupplierStatus {
    Active,
    PhaseOut,
    Suspended,
}

#[derive(Debug, Clone)]
pub struct Supplier {
    id: SupplierID,
    name: String,
    company_name: String,
    contact_email: String,
    status: SupplierStatus,
}

static PRODUCT_MODEL: LazyLock<Model<ProductInput, Product, ProductCtxOptions>> =
    LazyLock::new(|| PRODUCT_SCHEMA.model());

static PRODUCT_SCHEMA: LazyLock<Schema<ProductInput, Product, ProductCtxOptions>> =
    LazyLock::new(|| {
        Schema::new(
            |f| {
                f.set("id", IvoField::CONSTANT.value(ProductID(1)))
                    .set(
                        "name",
                        IvoField::REQUIRED.validate(|_: String, _, _| ready(Ok(None))),
                    )
                    .set(
                        "sku",
                        IvoField::REQUIRED.validate(|_: String, _, _| ready(Ok(None))),
                    )
                    .set(
                        "price",
                        IvoField::REQUIRED.validate(|_: u32, _, _| ready(Ok(None))),
                    )
                    .set(
                        "supplier",
                        IvoField::REQUIRED
                            .validate(|_: SupplierID, _, _| ready(Ok(None)))
                            .re_validate(
                                async |id: SupplierID, _, o: IvoRwCtxOptions<ProductCtxOptions>| {
                                    let mut ctx_options = o.write().await;

                                    if let Some(supplier) =
                                        ctx_options.get_supplier_by_id(&id).await
                                    {
                                        if matches!(
                                            supplier.status,
                                            SupplierStatus::PhaseOut | SupplierStatus::Suspended
                                        ) {
                                            let n = id.0;

                                            ctx_options.add_warning(&format!(
                                                "warning: supplier {n} is not currently active!"
                                            ))
                                        }

                                        return Ok(None);
                                    }

                                    Err(("Supplier not found".into(), None))
                                },
                            ),
                    )
            },
            |o| o,
        )
    });

static SUPPLIERS_DB: LazyLock<HashMap<SupplierID, Supplier>> = LazyLock::new(|| {
    let arr: [_; 5] = array::from_fn(|i| {
        let num = i + 1;

        let id = SupplierID(num as u64);
        let name = format!("supplier-{num}");
        let company_name = format!("company_{num}");

        (
            id,
            Supplier {
                id,
                name: name.clone(),
                company_name: company_name.clone(),
                contact_email: format!("{name}@{company_name}.com"),
                status: SupplierStatus::PhaseOut,
            },
        )
    });

    HashMap::from(arr)
});
