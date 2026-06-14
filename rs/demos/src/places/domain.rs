use std::{collections::HashMap, future::ready, sync::LazyLock, time::Instant};

use ivo::{
    FieldError, IvoErrorTool, IvoField, IvoStruct, Model, Schema, SharedCtxOptions,
    SharedIvoContext, SharedRwCtxOptions,
};
use serde::Deserialize;

use crate::utils::styled_text::Stylable;

const LOCATION_SERVICE_URL: &'static str = "https://misc-api.kamtoeddy.com/geo/details-with-tz";

#[derive(Debug, PartialEq, Clone)]
pub struct Coodinates {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Clone, PartialEq, IvoStruct)]
pub struct Place {
    pub id: i32,
    pub coordinates: Coodinates,
    pub name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, IvoStruct)]
pub struct PlaceInput {
    pub coordinates: Coodinates,
}

#[derive(Clone)]
pub struct PlacesCtxOptions;

impl<'a> PlacesCtxOptions {
    pub fn new() -> Self {
        Self
    }

    fn find_user_by_coordinates(
        &self,
        _coordinates: &String,
    ) -> impl Future<Output = Option<Place>> + use<'a> {
        ready(None)
    }
}

type Ctx = SharedIvoContext<PlaceInput, Place>;
type CtxOptions = SharedCtxOptions<PlacesCtxOptions>;
type RwCtxOptions = SharedRwCtxOptions<PlacesCtxOptions>;

#[derive(Debug, Deserialize)]
struct LocationDetails {
    display_name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ResolvedLocationDetails {
    details: LocationDetails,
    tz: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ResolvedLocationResults {
    data: Option<ResolvedLocationDetails>,
    error: Option<String>,
}

pub static PLACE_MODEL: LazyLock<Model<PlaceInput, Place, PlacesCtxOptions, PlacesErrorTool>> =
    LazyLock::new(|| PLACE_SCHEMA.get_model());

pub static PLACE_SCHEMA: LazyLock<Schema<PlaceInput, Place, PlacesCtxOptions, PlacesErrorTool>> =
    LazyLock::new(|| {
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
                        .validate(|Coodinates { lat, lon }: Coodinates, _, _| {
                            if lat.is_nan() || lon.is_nan() {
                                return ready(Err(("InvalidNumber".into(), None)));
                            }

                            let mut errors = vec![];

                            if !(-90.0..=90.0).contains(&lat) {
                                errors.push("LatitudeOutOfRange: [-90, 90]".into());
                            }

                            if !(-180.0..=180.0).contains(&lon) {
                                errors.push("LongitudeOutOfRange: [-180, 180]".into());
                            }

                            if !errors.is_empty() {
                                return ready(Err(("Out of range error".into(), Some(errors))));
                            }

                            ready(Ok(Coodinates { lat, lon }))
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
                            let mut timer = Instant::now();

                            match reqwest::get(format!(
                                "{LOCATION_SERVICE_URL}?lat={}&lon={}",
                                v.lat, v.lon
                            ))
                            .await
                            .map(|r| {
                                println!(
                                    "{} {}\n",
                                    "\nFetch location details:".font_bold(),
                                    format!("{:?}", timer.elapsed()).colored_red()
                                );
                                timer = Instant::now();
                                r.json::<ResolvedLocationResults>()
                            }) {
                                Ok(resp) => {
                                    let resp = resp.await;

                                    if resp.is_ok() {
                                        let data = resp.unwrap();

                                        println!("data: {:?}\n", data);

                                        println!(
                                            "{} {}\n",
                                            "\nParse location details:".font_bold(),
                                            format!("{:?}", timer.elapsed()).colored_blue()
                                        );
                                        if let Some(d) = data.data {
                                            return d.details.display_name;
                                        }
                                    } else {
                                        let error = resp.err().unwrap();

                                        println!("resp: {:?}", error);
                                    }
                                }
                                Err(e) => {
                                    println!("Err: {e:?}")
                                }
                            }

                            Some(String::from(""))
                        })
                        .on_success(|_, _| {
                            println!("[name]: on success",);
                            ready(())
                        }),
                )
                .created_at(|| "Date.now()", None)
                .updated_at(|| "Date.now()", Some("updated_on"), true)
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

pub struct PlacesErrorTool {
    errors: HashMap<String, Vec<String>>,
}

impl IvoErrorTool for PlacesErrorTool {
    type FieldMetadata = Vec<String>;
    type ErrorPayload = HashMap<String, Vec<String>>;

    fn add(&mut self, field_name: &str, error: FieldError<Self::FieldMetadata>) -> &mut Self {
        self.errors.entry(field_name.to_owned()).and_modify(|e| {
            e.push(error.reason);

            if error.metadata.is_some() {
                for err in error.metadata.unwrap() {
                    e.push(err);
                }
            }

            ()
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
