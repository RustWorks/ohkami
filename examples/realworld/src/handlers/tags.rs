use ohkami::{Ohkami, Route};
use crate::config::pool;
use crate::errors::RealWorldError;
use crate::models::{Tag, response::ListOfTagsResponse};


pub fn tags_ohkami() -> Ohkami {
    Ohkami::new((
        "/".GET(get),
    ))
}

async fn get() -> Result<ListOfTagsResponse<'static>, RealWorldError> {
    let tags = sqlx::query!(r#"
        SELECT name
        FROM tags
    "#).fetch_all(pool()).await
        .map_err(RealWorldError::DB)?.into_iter()
        .map(|n| Tag::new(n.name)).collect();

    Ok(ListOfTagsResponse { tags })
}