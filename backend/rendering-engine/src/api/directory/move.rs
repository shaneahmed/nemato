use crate::api::common::*;

#[derive(Deserialize)]
pub struct Body {
    parent_id: u32,
}

pub async fn r#move(
    Extension(mut logger): Extension<Logger<'_>>,
    Path(id): Path<u32>,
    Json(Body { parent_id }): Json<Body>,
) -> Response {
    if PRIVILEDGED.contains(&id) {
        return logger.error(
            StatusCode::FORBIDDEN,
            Error::RequestIntegrity,
            "DM-E00",
            "Cannot move priviledged directories.",
            None,
        );
    }

    logger.report(
        Check::RequestIntegrity,
        "Specified parent directory is not a priviledged directory.",
    );

    if STORES.contains(&id) {
        return logger.error(
            StatusCode::FORBIDDEN,
            Error::RequestIntegrity,
            "DM-E01",
            "Cannot move stores.",
            None,
        );
    }

    logger.report(
        Check::RequestIntegrity,
        "Specified directory is not a store.",
    );

    if parent_id == ROOT_ID {
        return logger.error(
            StatusCode::FORBIDDEN,
            Error::RequestIntegrity,
            "DM-E02",
            "Cannot move directories into the root directory.",
            None,
        );
    }

    logger.report(
        Check::RequestIntegrity,
        "Specified parent directory is not the root directory.",
    );

    // Retrieve target directory path.
    let target_directory_path = match crate::db::directory::path(id) {
        Ok(path) => {
            logger.report(
                Check::ResourceExistence,
                "Target directory exists in the database and its path was successfully retrieved.",
            );

            path
        }
        Err(e) => {
            return logger.error(
                StatusCode::NOT_FOUND,
                Error::DatabaseQuery,
                "DM-E03",
                "Target directory does not exist in the database.",
                Some(e),
            );
        }
    };

    // Retrieve destination directory path.
    let dest_directory_path = match crate::db::directory::path(parent_id) {
        Ok(path) => {
            logger.report(
                Check::ResourceExistence,
                "Destination directory exists in the database and its path was successfully retrieved.",
            );

            path
        }
        Err(e) => {
            return logger.error(
                StatusCode::NOT_FOUND,
                Error::DatabaseQuery,
                "DM-E04",
                "Destination directory does not exist in the database.",
                Some(e),
            );
        }
    };

    // Check destination is not inside target.
    if dest_directory_path.starts_with(&target_directory_path) {
        return logger.error(
            StatusCode::FORBIDDEN,
            Error::RequestIntegrity,
            "DM-E05",
            "Cannot move directory into itself.",
            None,
        );
    }

    logger.report(
        Check::RequestIntegrity,
        "Destination directory is not inside target directory.",
    );

    // Move the directory in the filesystem.
    match crate::io::r#move(&target_directory_path, &dest_directory_path) {
        Ok(()) => {
            logger.log("Directory moved in the filesystem.");
        }
        Err(e) => {
            return logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::ResourceMove,
                "DM-E06",
                "Failed to move directory in the filesystem.",
                Some(e),
            );
        }
    }

    // Move the directory in the database.
    match crate::db::directory::r#move(id, parent_id, &MoveMode::Regular) {
        Ok(()) => {
            logger.log("Directory moved in the database.");
        }
        Err(e) => {
            return logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::ResourceMove,
                "DM-E07",
                "Failed to move directory in the database.",
                Some(e),
            );
        }
    }

    let registry = match crate::db::general::get_registry() {
        Ok(registry) => {
            logger.log("Registry retrieved from the database.");

            registry
        }
        Err(e) => {
            return logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::DatabaseQuery,
                "DM-E08",
                "Failed to retrieve registry from the database.",
                Some(e),
            )
        }
    };

    logger.success(StatusCode::OK, "Directory moved successfully.");

    (StatusCode::OK, Json(registry)).into_response()
}
