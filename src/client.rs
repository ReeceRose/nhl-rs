/// A builder used for constructing a [`Client`]. Constructed using [`ClientBuilder::new`].
#[derive(Debug, Default)]
pub struct ClientBuilder {
    language: Option<String>,
}

impl ClientBuilder {
    /// Creates a new [`ClientBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the language code for the client.
    ///
    /// # Example
    /// ```rust
    /// use nhl_rs::ClientBuilder;
    ///
    /// const LANGUAGE_CODE: &str = "fr";
    ///
    /// let client = ClientBuilder::new().language(LANGUAGE_CODE.to_string()).build();
    /// ```
    pub fn language(mut self, language: String) -> Self {
        self.language = Some(language);
        self
    }

    /// Builds the [`Client`]. This consumes the builder.
    ///
    /// # Example
    /// ```rust
    /// use roboat::ClientBuilder;
    ///
    /// let client = ClientBuilder::new().build();
    /// ```
    pub fn build(self) -> Client {
        Client {
            language: self.language.unwrap_or("en".to_string()),
            base_url: "https://api-web.nhle.com".to_string(),
            stats_base_url: "https://api.nhle.com/stats/rest".to_string(),
            ..Default::default()
        }
    }
}

/// A client used for making requests to the NHL API.
///
/// The client stores the language code, the URLs, and an HTTPS client to send web requests.
///
/// Constructed using a [`ClientBuilder`].
///
/// # Construction Examples
///
/// ## Without setting language (defaults to English)
/// ```
/// use nhl_rs::ClientBuilder;
///
/// let client = ClientBuilder::new().build();
/// ```
///
/// ## With a language.
/// ```
/// use nhl_rs::ClientBuilder;
///
/// const LANGUAGE_CODE: &str = "fr";
///
/// let client = ClientBuilder::new().language(LANGUAGE_CODE.to_string()).build();
/// ```
///
#[derive(Debug, Default)]
pub struct Client {
    pub(crate) language: String,
    pub(crate) base_url: String,
    pub(crate) stats_base_url: String,
}
