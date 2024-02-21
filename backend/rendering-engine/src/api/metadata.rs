use crate::api::common::*;

pub async fn metadata(
    Extension(AppState {
        pool,
        current_image,
        ..
    }): Extension<AppState>,
    Json(id): Json<u32>,
) -> Response {
    #[cfg(feature = "log")]
    log::<()>(
        StatusCode::ACCEPTED,
        &format!("Received request for metadata of image: {}.", id),
        None,
    );

    // Get image with id from the database.
    let Ok(image) = crate::db::get(id, &pool).await else {
        return log_respond::<()>(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!(
                "Image with id {} does not exist in the database or doesn't have metadata.",
                id
            ),
            None,
        );
    };

    let metadata = image.metadata.clone();

    // Insert into in-memory state.
    *current_image.lock().unwrap() = Some(image);

    Json(metadata).into_response()
}
