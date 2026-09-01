use std::collections::HashMap;

use ivo::{ivo_schema, IvoErrorPayload, IvoErrorSanitizer};

async fn should_respect_custom_error_sanitizer() {
    let r = place_schema::PlaceInputModel.create(
        place_schema::PartialPlaceInput {
            coordinates: Some(Coordinates {
                lat: f64::NAN,
                lon: f64::NAN,
            }),
        },
        None,
    );

    match r {
        Err((errors, ..)) => {
            let errors = errors.get("coordinates").unwrap();

            assert_eq!(errors.len(), 1);
            assert!(errors.contains(&customize("InvalidNumber")));
        }
        _ => unreachable!("expected a validation error"),
    }

    let r = place_schema::PlaceInputModel.create(
        place_schema::PartialPlaceInput {
            coordinates: Some(Coordinates {
                lat: 400.0,
                lon: -200.0,
            }),
        },
        None,
    );

    match r {
        Err((errors, ..)) => {
            let errors = errors.get("coordinates").unwrap();

            assert_eq!(errors.len(), 3);
            assert!(errors.contains(&customize("Out of range error")));
            assert!(errors.contains(&customize("LatitudeOutOfRange: [-90, 90]")));
            assert!(errors.contains(&customize("LongitudeOutOfRange: [-180, 180]")));
        }
        _ => unreachable!("expected a validation error"),
    }

    let data = place_schema::PlaceInput {
        coordinates: Coordinates {
            lat: 3.0,
            lon: 45.1,
        },
    };

    let r = place_schema::PlaceInputModel.update(
        data.clone(),
        place_schema::PartialPlaceInput {
            coordinates: Some(Coordinates {
                lat: f64::NAN,
                lon: f64::NAN,
            }),
        },
        None,
    );

    match r {
        Err((errors, ..)) => {
            let errors = errors.as_ref().unwrap().get("coordinates").unwrap();

            assert_eq!(errors.len(), 1);
            assert!(errors.contains(&customize("InvalidNumber")));
        }
        _ => unreachable!("expected a validation error"),
    }

    let r = place_schema::PlaceInputModel.update(
        data.clone(),
        place_schema::PartialPlaceInput {
            coordinates: Some(Coordinates {
                lat: -400.0,
                lon: 200.0,
            }),
        },
        None,
    );

    match r {
        Err((errors, ..)) => {
            let errors = errors.as_ref().unwrap().get("coordinates").unwrap();

            assert_eq!(errors.len(), 3);
            assert!(errors.contains(&customize("Out of range error")));
            assert!(errors.contains(&customize("LatitudeOutOfRange: [-90, 90]")));
            assert!(errors.contains(&customize("LongitudeOutOfRange: [-180, 180]")));
        }
        _ => unreachable!("expected a validation error"),
    }

    let updated_coords = Coordinates {
        lat: data.coordinates.lat + 1.1,
        lon: data.coordinates.lon,
    };

    let (updated, ..) = place_schema::PlaceInputModel
        .update(
            data.clone(),
            place_schema::PartialPlaceInput {
                coordinates: Some(updated_coords.clone()),
            },
            None,
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated,
        place_schema::PartialPlaceInput {
            coordinates: Some(updated_coords)
        }
    );

    let data = data.clone_with_updates(&updated);

    let (failed, ..) = place_schema::PlaceInputModel
        .update(
            data.clone(),
            place_schema::PartialPlaceInput {
                coordinates: Some(data.coordinates.clone()),
            },
            None,
        )
        .err()
        .unwrap();

    assert!(failed.is_none());
}

async_test_matrix!(should_respect_custom_error_sanitizer);

#[derive(Debug, Default, PartialEq, Clone)]
struct Coordinates {
    lat: f64,
    lon: f64,
}

type PlacesCtxOptions = Option<(String, Coordinates)>;

#[ivo_schema(
    input(PlaceInput, derive(Debug, Clone, PartialEq)),
    ctx_options(PlacesCtxOptions),
    error_sanitizer(ErrorSanitizer)
)]
mod place_schema {
    use super::{Coordinates, ErrorSanitizer, PlacesCtxOptions};

    struct Fields {
        #[required]
        #[validate(|c: Coordinates, _, _| {
            if c.lat.is_nan() || c.lon.is_nan() {
                return Err(("InvalidNumber".into(), None));
            }

            let mut errors = vec![];

            if !(-90.0..=90.0).contains(&c.lat) {
                errors.push("LatitudeOutOfRange: [-90, 90]".into());
            }

            if !(-180.0..=180.0).contains(&c.lon) {
                errors.push("LongitudeOutOfRange: [-180, 180]".into());
            }

            if !errors.is_empty() {
                return Err(("Out of range error".into(), Some(errors)));
            }

            Ok(None)
        })]
        pub coordinates: Coordinates,
    }
}

struct ErrorSanitizer;

impl IvoErrorSanitizer<PlacesCtxOptions> for ErrorSanitizer {
    type Metadata = Vec<String>;
    type Payload = HashMap<String, Vec<String>>;

    fn sanitize(payload: IvoErrorPayload<Self::Metadata>, o: &PlacesCtxOptions) -> Self::Payload {
        let mut errors = HashMap::new();

        for (field_name, error) in payload {
            let mut field_errors = vec![customize(&error.reason)];

            if let Some(metadata) = error.metadata {
                for err in metadata {
                    field_errors.push(customize(&err));
                }
            }

            errors.insert(field_name, field_errors);
        }

        if let Some((s, coordinates)) = o {
            println!("{s:?} {coordinates:?}")
        }

        errors
    }
}

fn customize(s: &str) -> String {
    format!("customized: {s}")
}
