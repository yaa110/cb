use users;

pub struct UserGroup {
    pub user: String,
    pub group: String,
}

impl UserGroup {
    /// Returns current UserGroup
    pub fn get() -> Result<UserGroup, String> {
        let user = users::get_current_username()
            .ok_or_else(|| String::from("unable to get username"))?
            .to_str()
            .ok_or_else(|| String::from("unable to encode username"))?
            .to_string();
        let group = users::get_current_groupname()
            .ok_or_else(|| String::from("unable to get group name"))?
            .to_str()
            .ok_or_else(|| String::from("unable to encode group name"))?
            .to_string();
        Ok(UserGroup {
            user,
            group,
        })
    }
}
