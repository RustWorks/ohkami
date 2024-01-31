use std::borrow::Cow;

use ohkami::{Ohkami, Route, Memory};
use crate::config::{pool, JWTPayload};
use crate::{fangs::Auth, errors::RealWorldError};
use crate::models::response::ProfileResponse;
use crate::db::UserEntity;


pub fn profiles_ohkami() -> Ohkami {
    Ohkami::with(Auth::default(), (
        "/:username"
            .GET(get_profile),
        "/:username/follow"
            .POST(follow)
            .DELETE(unfollow),
    ))
}

async fn get_profile(
    username:    &str,
    jwt_payload: Memory<'_, JWTPayload>,
) -> Result<ProfileResponse, RealWorldError> {
    let the_user = UserEntity::get_by_name(username).await?;

    let following_the_user = sqlx::query!(r#"
        SELECT EXISTS (
            SELECT id
            FROM users_follow_users AS ufu
            WHERE
                ufu.follower_id = $1 AND
                ufu.followee_id = $2
        )
    "#, jwt_payload.user_id, the_user.id)
        .fetch_one(pool()).await
        .map_err(RealWorldError::DB)?
        .exists.unwrap();

    Ok(the_user.into_profile_response_with(following_the_user))
}

async fn follow(
    username:    &str,
    jwt_payload: Memory<'_, JWTPayload>,
) -> Result<ProfileResponse, RealWorldError> {
    let by_existing_user = sqlx::query!(r#"
        SELECT EXISTS (
            SELECT id
            FROM users AS u
            WHERE
                u.id = $1
        )
    "#, jwt_payload.user_id)
        .fetch_one(pool()).await
        .map_err(RealWorldError::DB)?
        .exists.unwrap();
    if !by_existing_user {
        return Err(RealWorldError::Unauthorized(Cow::Owned(format!(
            "User of id '{}' doesn't exist",
            jwt_payload.user_id
        ))))
    }

    let followee = UserEntity::get_by_name(username).await?;

    sqlx::query!(r#"
        INSERT INTO
        users_follow_users (followee_id, follower_id)
        VALUES             ($1,          $2)
    "#, followee.id, jwt_payload.user_id)
        .execute(pool()).await
        .map_err(RealWorldError::DB)?;

    Ok(followee.into_profile_response_with(true))
}

async fn unfollow(
    username:    &str,
    jwt_payload: Memory<'_, JWTPayload>,
) -> Result<ProfileResponse, RealWorldError> {
    let followee = UserEntity::get_by_name(username).await?;

    let deletion_count = sqlx::query!(r#"
        DELETE FROM users_follow_users AS ufu
        WHERE
            ufu.followee_id = $1 AND
            ufu.follower_id = $2
    "#, followee.id, jwt_payload.user_id)
        .execute(pool()).await
        .map_err(RealWorldError::DB)?
        .rows_affected();
    if deletion_count != 1 {
        tracing::error!("\
            Found {deletion_count} deletion of following \
            {} by {}
        ", followee.id, jwt_payload.user_id);
        return Err(RealWorldError::FoundUnexpectedly(Cow::Borrowed(
            "Found more than one following"
        )))
    }

    Ok(followee.into_profile_response_with(false))
}