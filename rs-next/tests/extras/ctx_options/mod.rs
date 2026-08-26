use ivo::ivo_schema;

mod constants;
mod dependents;
mod lax;
mod required;
mod virtuals;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct ProductID(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct SupplierID(u64);

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Product {
    id: ProductID,
    name: String,
    sku: String,
    price: u32,
    supplier: SupplierID,
}

#[derive(Debug, Clone, PartialEq)]
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

async fn should_properly_update_ctx_options() {
    let supplier_num = 2;

    let created = product_schema::ProductModel
        .create(
            product_schema::PartialProductInput {
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
        created.ctx_options.read().await.warnings[0],
        format!("warning: supplier {supplier_num} is not currently active!")
    );

    let supplier_num = 3;

    let updated = product_schema::ProductModel
        .update(
            created.data.clone(),
            product_schema::PartialProductInput {
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
        updated.ctx_options.read().await.warnings[0],
        format!("warning: supplier {supplier_num} is not currently active!")
    );
}

async_test_matrix!(should_properly_update_ctx_options);

#[ivo_schema(
    input(ProductInput, derive(Debug, Clone, PartialEq)),
    output(Product, derive(Debug, Clone, PartialEq)),
    ctx_options(ProductCtxOptions)
)]
mod product_schema {
    use super::{ProductCtxOptions, ProductID, SupplierID, SupplierStatus};

    struct Fields {
        #[constant(ProductID(1))]
        pub id: ProductID,

        #[required]
        pub name: String,

        #[required]
        pub sku: String,

        #[required]
        pub price: u32,

        #[required]
        #[validate(async |id, _, opts| {
            let mut ctx_options = opts.write().await;

            if let Some(supplier) = ctx_options.get_supplier_by_id(&id).await {
                if matches!(
                    supplier.status,
                    SupplierStatus::PhaseOut | SupplierStatus::Suspended
                ) {
                    let n = id.0;
                    ctx_options.add_warning(&format!(
                        "warning: supplier {n} is not currently active!"
                    ));
                }

                return Ok(None);
            }

            Err(("Supplier not found".into(), None))
        })]
        pub supplier: SupplierID,
    }
}

use std::{array, collections::HashMap};

static SUPPLIERS_DB: std::sync::LazyLock<HashMap<SupplierID, Supplier>> =
    std::sync::LazyLock::new(|| {
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
