use std::{collections::HashMap, future::ready, sync::LazyLock};

use crate::async_test_matrix;
use ivo::{IvoErrorPayload, IvoErrorSanitizer, IvoField, IvoInputStruct, IvoStruct, Model};

async fn should_respect_custom_error_tool() {
    let r = PLACE_MODEL
        .create(
            &PartialPlace {
                coordinates: Some(Coodinates {
                    lat: f64::NAN,
                    lon: f64::NAN,
                }),
            },
            None,
        )
        .await;

    match r {
        Err((p, _, _)) => {
            let errors = p.get("coordinates").unwrap();

            assert_eq!(errors.len(), 1);
            assert!(errors.contains(&customized_string("InvalidNumber")));
        }
        _ => unreachable!("expected a validation error"),
    }

    let r = PLACE_MODEL
        .create(
            &PartialPlace {
                coordinates: Some(Coodinates {
                    lat: 400.0,
                    lon: -200.0,
                }),
            },
            None,
        )
        .await;

    match r {
        Err((p, _, _)) => {
            let errors = p.get("coordinates").unwrap();

            assert_eq!(errors.len(), 3);
            assert!(errors.contains(&customized_string("Out of range error")));
            assert!(errors.contains(&customized_string("LatitudeOutOfRange: [-90, 90]")));
            assert!(errors.contains(&customized_string("LongitudeOutOfRange: [-180, 180]")));
        }
        _ => unreachable!("expected a validation error"),
    }

    let data = Place {
        coordinates: Coodinates {
            lat: 3.0,
            lon: 45.1,
        },
    };

    let r = PLACE_MODEL
        .update(
            &data,
            &PartialPlace {
                coordinates: Some(Coodinates {
                    lat: f64::NAN,
                    lon: f64::NAN,
                }),
            },
            None,
        )
        .await;

    match r {
        Err((Some(payload), _, _)) => {
            let errors = payload.get("coordinates").unwrap();

            assert_eq!(errors.len(), 1);
            assert!(errors.contains(&customized_string("InvalidNumber")));
        }
        _ => unreachable!("expected a validation error"),
    }

    let r = PLACE_MODEL
        .update(
            &data,
            &PartialPlace {
                coordinates: Some(Coodinates {
                    lat: -400.0,
                    lon: 200.0,
                }),
            },
            None,
        )
        .await;

    match r {
        Err((Some(payload), _, _)) => {
            let errors = payload.get("coordinates").unwrap();

            assert_eq!(errors.len(), 3);
            assert!(errors.contains(&customized_string("Out of range error")));
            assert!(errors.contains(&customized_string("LatitudeOutOfRange: [-90, 90]")));
            assert!(errors.contains(&customized_string("LongitudeOutOfRange: [-180, 180]")));
        }
        _ => unreachable!("expected a validation error"),
    }

    let updated_coords = Coodinates {
        lat: data.coordinates.lat + 1.1,
        lon: data.coordinates.lon,
    };

    let (updates, _, _) = PLACE_MODEL
        .update(
            &data,
            &PartialPlace {
                coordinates: Some(updated_coords.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialPlace {
            coordinates: Some(updated_coords)
        }
    );

    let data = data.clone_with_updates(&updates);

    let (err, _, _) = PLACE_MODEL
        .update(
            &data,
            &PartialPlace {
                coordinates: Some(data.coordinates.clone()),
            },
            None,
        )
        .await
        .err()
        .unwrap();

    assert!(err.is_none());
}

async_test_matrix!(should_respect_custom_error_tool);

#[derive(Debug, PartialEq, Clone)]
struct Coodinates {
    lat: f64,
    lon: f64,
}

#[derive(Debug, Clone, IvoInputStruct)]
struct Place {
    coordinates: Coodinates,
}

type PlacesCtxOptions = Option<()>;
type PlacesTimestamp = ();

static PLACE_MODEL: LazyLock<
    Model<Place, Place, PlacesCtxOptions, PlacesTimestamp, PlacesErrorSanitizer>,
> = LazyLock::new(|| {
    Model::new(
        |f| {
            f.field(
                "coordinates",
                IvoField::REQUIRED.validate(|c: Coodinates, _, _| {
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
                }),
            )
        },
        |o| o,
    )
});

type PlacesErrorSanitizerMetadata = Vec<String>;

struct PlacesErrorSanitizer;

impl IvoErrorSanitizer for PlacesErrorSanitizer {
    type Metadata = PlacesErrorSanitizerMetadata;
    type Payload = HashMap<String, Vec<String>>;

    fn sanitize<PlacesCtxOptions>(
        payload: IvoErrorPayload<Self::Metadata>,
        _: &PlacesCtxOptions,
    ) -> Self::Payload {
        let mut errors = HashMap::new();

        for (field_name, error) in payload {
            let mut field_errors = vec![customized_string(&error.reason)];

            if let Some(ref metadata) = error.metadata {
                for err in metadata {
                    field_errors.push(customized_string(err));
                }
            }

            errors.insert(field_name, field_errors);
        }

        errors
    }
}

fn customized_string(s: &str) -> String {
    format!("customized: {s}")
}
