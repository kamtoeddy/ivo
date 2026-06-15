use ivo::{UpdateError, types::MethodsOfIvoStruct};
use std::time::Instant;

mod domain;

use crate::{
    places::domain::{Coodinates, PLACE_MODEL, PartialPlaceInput, Place, PlacesCtxOptions},
    utils::styled_text::Stylable,
};

pub async fn run_places_demo() {
    let timer = Instant::now();

    let input = PartialPlaceInput {
        // coordinates: None,
        coordinates: Some(Coodinates {
            lat: 4.756841301293143,
            lon: 11.235494655828541,
        }),
    };

    let r = PLACE_MODEL.create(&input, PlacesCtxOptions::new()).await;

    match r {
        Ok((data, handle_success)) => {
            println!("{:?}\n", data);

            handle_success().await;
        }
        Err((payload, handle_failure)) => {
            println!("\nFailed to create: {:?}\n", payload);

            handle_failure().await;
        }
    };

    println!(
        "{} {}\n",
        "\nCreate duration:".font_bold(),
        format!("{:?}", timer.elapsed()).colored_blue()
    );

    let timer = Instant::now();

    let place = Place {
        id: 1,
        coordinates: Coodinates {
            lat: 4.756841301293143,
            lon: 11.235494655828541,
        },
        name: Some("Centre Administratif, Bafia, Mbam-et-Inoubou, Centre, Cameroun".into()),
    };

    println!("{:?}\n", place);

    let updates = PartialPlaceInput {
        // coordinates: None,
        // coordinates: Some(Coodinates {
        //     lat: 10.756841301293143,
        //     lon: 11.235494655828541,
        // }),
        coordinates: Some(Coodinates {
            lat: 4.756841301293143,
            lon: 11.235494655828541,
        }),
    };

    let r = PLACE_MODEL
        .update(&place, &updates, PlacesCtxOptions::new())
        .await;

    match r {
        Ok((data, handle_success)) => {
            println!("updates: {:?}\n", data);
            println!(
                "old + updates: {:?}\n",
                place.ivo_internal_clone_with_ref(&data)
            );

            handle_success().await;
        }
        Err((error, handle_failure)) => {
            match error {
                UpdateError::NothingToUpdate => println!("Nothing to update\n"),
                UpdateError::ValidationError(payload) => {
                    println!("Failed to update: {:?}\n", payload)
                }
            };

            handle_failure().await;
        }
    };

    println!(
        "{} {}\n",
        "\nUpdate duration:".font_bold(),
        format!("{:?}", timer.elapsed()).colored_blue()
    );
}
