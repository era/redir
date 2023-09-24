use diesel::prelude::*;

type DbError = Box<dyn std::error::Error + Send + Sync>;

use crate::models;

pub fn find_link_by_id(
    conn: &mut SqliteConnection,
    url_id: &str,
) -> Result<Option<models::Links>, DbError> {
    use crate::schema::links::dsl::*;

    let saved_url = links
        .filter(id.eq(url_id))
        .first::<models::Links>(conn)
        .optional()?;

    Ok(saved_url)
}

pub fn find_link_by_url(
    conn: &mut SqliteConnection,
    url_to_search: &str,
) -> Result<Option<models::Links>, DbError> {
    use crate::schema::links::dsl::*;

    let found_url = links
        .filter(url.eq(url_to_search))
        .first::<models::Links>(conn)
        .optional()?;

    Ok(found_url)
}

pub fn insert_new_url(
    conn: &mut SqliteConnection,
    user_url: String,
    url_id: Option<String>,
) -> Result<models::Links, DbError> {
    use crate::schema::links::dsl::*;

    if let Some(existing_url) = find_link_by_url(conn, &user_url)? {
        return Ok(existing_url);
    }

    let encoded_id = match url_id {
        Some(encoded_id) => {
            if let Some(existing_url) = find_link_by_id(conn, &encoded_id)? {
                return Ok(existing_url);
            }
            encoded_id
        }
        None => base62::encode(bytes_to_u128(md5::compute(&user_url).as_slice()).unwrap()).to_string()
    };


    let new_url = models::Links {
        id: encoded_id,
        url: user_url,
        count: 0,
    };

    diesel::insert_into(links).values(&new_url).execute(conn)?;

    Ok(new_url)
}

fn bytes_to_u128(bytes: &[u8]) -> Option<u128> {
    // Ensure that the input slice has exactly 16 bytes, as u128 is 16 bytes.
    if bytes.len() != 16 {
        return None;
    }

    let mut result = 0u128;

    // Iterate through each byte in the input slice.
    for &byte in bytes {
        // Shift the current value left by 8 bits (one byte) and add the new byte value.
        result = (result << 8) | u128::from(byte);
    }

    Some(result)
}
