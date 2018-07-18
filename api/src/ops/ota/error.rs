/// Use this for most non-specific errors.
///
/// Refer to `doc/user.md` for more details.
///
/// # Examples
///
/// ```no_run
/// # #![feature(plugin)]
/// # #![plugin(rocket_codegen)]
/// # extern crate sudoku_backend;
/// # #[macro_use]
/// # extern crate rocket;
/// # use rocket::response::content::Json;
/// # use sudoku_backend::ops::GenericError;
/// # fn work() -> Result<String, &'static str> {
/// #     Err("henlo")
/// # }
/// #[get("/endpoint")]
/// fn endpoint() -> Result<String, Json<GenericError>> {
///     work().map_err(|e| Json(GenericError {
///         reason: format!("couldn't finish work: {}", e),
///     }))
/// }
///
/// fn main() {
///     rocket::ignite()
///         .mount("/", routes![endpoint])
///         .launch();
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct GenericError {
    /// In all-lowercase past-tense finishing-punctuation-free form.
    ///
    /// For example: "failed to apply diff", "user with that name exists".
    pub reason: String,
}

/// Security ⇒ no data.
///
/// Refer to `doc/user.md` for more details.
///
/// # Examples
///
/// ```no_run
/// # #![feature(plugin)]
/// # #![plugin(rocket_codegen)]
/// # extern crate sudoku_backend;
/// # #[macro_use]
/// # extern crate rocket;
/// # use rocket::response::content::Json;
/// # use sudoku_backend::ops::LoginError;
/// # fn do_log_in() -> Option<String> {
/// #     None
/// # }
/// #[get("/login")]
/// fn login() -> Result<String, Json<LoginError>> {
///     // Specificities of the log-in process are outside the scope of the document
///     do_log_in().ok_or(Json(LoginError {}))
/// }
///
/// fn main() {
///     rocket::ignite()
///         .mount("/", routes![login])
///         .launch();
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct LoginError {}
