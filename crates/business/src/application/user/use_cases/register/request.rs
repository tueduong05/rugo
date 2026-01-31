use validator::Validate;

#[derive(Validate)]
pub struct RegisterRequest {
    #[validate(
        length(min = 3, max = 20, message = "Username must be between 3 and 20 characters"),
        regex(path = *crate::domain::user::value_objects::username::USERNAME_REGEX, message = "Username contains invalid characters")
    )]
    pub username: String,

    #[validate(
        length(max = 256, message = "Email is too long"),
        regex(path = *crate::domain::user::value_objects::email::EMAIL_REGEX, message = "Invalid email format")
    )]
    pub email: String,

    pub password: String,
}
