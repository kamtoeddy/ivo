use std::{collections::HashMap, future::ready, sync::LazyLock};

use ivo::{FieldError, IvoContext, IvoErrorTool, IvoField, IvoStruct, Model, Schema};
use serde::Deserialize;

const LOCATION_SERVICE_URL: &str = "https://misc-api.kamtoeddy.com/geo/details-with-tz";

#[derive(Debug, PartialEq, Clone)]
pub struct Coodinates {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Clone, IvoStruct)]
pub struct Place {
    pub id: i32,
    pub coordinates: Coodinates,
    pub name: Option<String>,
}

#[derive(Clone, Debug, IvoStruct)]
pub struct PlaceInput {
    pub coordinates: Coodinates,
}

#[derive(Clone)]
pub struct PlacesCtxOptions;

impl PlacesCtxOptions {
    pub fn new() -> Self {
        Self
    }
}

type Ctx = IvoContext<PlaceInput, Place>;

#[derive(Debug, Deserialize)]
struct LocationDetails {
    display_name: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ResolvedLocationDetails {
    details: LocationDetails,
    tz: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ResolvedLocationResults {
    data: Option<ResolvedLocationDetails>,
    error: Option<String>,
}

pub static PLACE_MODEL: LazyLock<Model<PlaceInput, Place, PlacesCtxOptions, (), PlacesErrorTool>> =
    LazyLock::new(|| PLACE_SCHEMA.model());

pub static PLACE_SCHEMA: LazyLock<
    Schema<PlaceInput, Place, PlacesCtxOptions, (), PlacesErrorTool>,
> = LazyLock::new(|| {
    Schema::new(
        |f| {
            f.set(
                "id",
                IvoField::CONSTANT
                    .computed(|_, _| ready(1234))
                    .on_success(|ctx: Ctx, _| {
                        println!("[id]: on success: {:?}", ctx.values().id);

                        ready(())
                    }),
            )
            .set(
                "coordinates",
                IvoField::REQUIRED
                    .required_error("please provide \"coordinates\"")
                    .validate(|c: Coodinates, _, _| {
                        if c.lat.is_nan() || c.lon.is_nan() {
                            return ready(Err(("InvalidNumber".into(), None)));
                        }

                        let mut errors = vec![];

                        if !(-90.0..=90.0).contains(&c.lat) {
                            errors.push("LatitudeOutOfRange: [-90, 90]".into());
                        }

                        if !(-180.0..=180.0).contains(&c.lon) {
                            errors.push("LongitudeOutOfRange: [-180, 180]".into());
                        }

                        if !errors.is_empty() {
                            return ready(Err(("Out of range error".into(), Some(errors))));
                        }

                        ready(Ok(Some(c)))
                    })
                    .on_delete(|_, _| {
                        println!("[coordinates]: on delete 1 handled");

                        ready(())
                    })
                    .on_delete(|_, _| {
                        println!("[coordinates]: on delete 2 handled");

                        ready(())
                    })
                    .on_failure(|_, _| {
                        println!("[coordinates]: on failure handled");

                        ready(())
                    })
                    .on_success(|ctx: Ctx, _| {
                        println!(
                            "[coordinates]: on success uname with slug_id: {:?}",
                            ctx.values().coordinates
                        );

                        ready(())
                    }),
            )
            .set(
                "name",
                IvoField::DEPENDENT
                    .default(None)
                    .depends_on(["coordinates"])
                    .resolve(async |ctx: Ctx, _| {
                        let v = ctx.values().coordinates.unwrap();

                        match reqwest::get(format!(
                            "{LOCATION_SERVICE_URL}?lat={}&lon={}",
                            v.lat, v.lon
                        ))
                        .await
                        .map(|r| r.json::<ResolvedLocationResults>())
                        {
                            Ok(resp) => {
                                let resp = resp.await;

                                if let Ok(ResolvedLocationResults { data: Some(d), .. }) = resp {
                                    // println!("data: {:?}\n", d);
                                    return d.details.display_name;
                                } else {
                                    let error = resp.err().unwrap();

                                    println!("resp: {:?}", error);
                                }
                            }
                            Err(e) => {
                                println!("Err: {e:?}")
                            }
                        }

                        ctx.values().name.unwrap()
                    })
                    .on_success(|_, _| {
                        println!("[name]: on success",);
                        ready(())
                    }),
            )
        },
        |o| {
            o.on_delete(|_, _| {
                println!("[options.on_delete]: fn 1");

                ready(())
            })
            .on_delete(|_, _| {
                println!("[options.on_delete]: fn 2");

                ready(())
            })
        },
    )
});

type PlacesErrorToolFieldMetadata = Vec<String>;

pub struct PlacesErrorTool {
    errors: HashMap<String, Vec<String>>,
}

impl IvoErrorTool for PlacesErrorTool {
    type FieldMetadata = PlacesErrorToolFieldMetadata;
    type ErrorPayload = HashMap<String, Vec<String>>;

    fn add(&mut self, field_name: &str, error: FieldError<Self::FieldMetadata>) -> &mut Self {
        self.errors
            .entry(field_name.to_owned())
            .and_modify(|e| append_error(e, &error))
            .or_insert_with(|| {
                let mut errors = vec![];

                append_error(&mut errors, &error);

                errors
            });

        self
    }

    fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    fn new() -> Self {
        Self {
            errors: HashMap::new(),
        }
    }

    fn payload(self) -> Self::ErrorPayload {
        self.errors
    }
}

fn append_error(errors: &mut Vec<String>, error: &FieldError<PlacesErrorToolFieldMetadata>) {
    errors.push(error.reason.clone());

    if let Some(ref metadata) = error.metadata {
        for err in metadata {
            errors.push(err.clone());
        }
    }
}
