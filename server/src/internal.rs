use users;

/// Returns current (username, group_name)
pub fn get_user_group() -> Result<(String, String), String> {
    Ok((
        users::get_current_username()
            .ok_or_else(|| String::from("unable to get username"))?
            .to_str()
            .ok_or_else(|| String::from("unable to encode username"))?
            .to_string(),
        users::get_current_groupname()
            .ok_or_else(|| String::from("unable to get group"))?
            .to_str()
            .ok_or_else(|| String::from("unable to encode group"))?
            .to_string(),
    ))
}
