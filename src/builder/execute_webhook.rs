use std::collections::HashMap;
#[cfg(not(feature = "http"))]
use std::marker::PhantomData;

use serde_json::Value;

#[cfg(feature = "http")]
use crate::http::AttachmentType;
use crate::model::channel::MessageFlags;

/// A builder to create the inner content of a [`Webhook`]'s execution.
///
/// This is a structured way of cleanly creating the inner execution payload,
/// to reduce potential argument counts.
///
/// Refer to the documentation for [`execute_webhook`] on restrictions with
/// execution payloads and its fields.
///
/// # Examples
///
/// Creating two embeds, and then sending them as part of the delivery
/// payload of [`Webhook::execute`]:
///
/// ```rust,no_run
/// use serenity::http::Http;
/// use serenity::model::channel::Embed;
/// use serenity::utils::Colour;
///
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// # let http = Http::default();
/// let id = 245037420704169985;
/// let token = "ig5AO-wdVWpCBtUUMxmgsWryqgsW3DChbKYOINftJ4DCrUbnkedoYZD0VOH1QLr-S3sV";
/// let webhook = http.get_webhook_with_token(id, token).await?;
///
/// let website = Embed::fake(|e| {
///     e.title("The Rust Language Website")
///         .description("Rust is a systems programming language.")
///         .colour(Colour::from_rgb(222, 165, 132))
/// });
///
/// let resources = Embed::fake(|e| {
///     e.title("Rust Resources")
///         .description("A few resources to help with learning Rust")
///         .colour(0xDEA584)
///         .field("The Rust Book", "A comprehensive resource for Rust.", false)
///         .field("Rust by Example", "A collection of Rust examples", false)
/// });
///
/// webhook
///     .execute(&http, false, |w| {
///         w.content("Here's some information on Rust:").embeds(vec![website, resources])
///     })
///     .await?;
/// #     Ok(())
/// # }
/// ```
///
/// [`Webhook`]: crate::model::webhook::Webhook
/// [`Webhook::execute`]: crate::model::webhook::Webhook::execute
/// [`execute_webhook`]: crate::http::client::Http::execute_webhook
#[derive(Clone, Debug)]
pub struct ExecuteWebhook<'a>(
    pub HashMap<&'static str, Value>,
    #[cfg(feature = "http")] pub Vec<AttachmentType<'a>>,
    #[cfg(not(feature = "http"))] PhantomData<&'a ()>,
);

impl<'a> ExecuteWebhook<'a> {
    /// Override the default avatar of the webhook with an image URL.
    ///
    /// # Examples
    ///
    /// Overriding the default avatar:
    ///
    /// ```rust,no_run
    /// # use serenity::http::Http;
    /// #
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let http = Http::default();
    /// # let webhook = http.get_webhook_with_token(0, "").await?;
    /// #
    /// let avatar_url = "https://i.imgur.com/KTs6whd.jpg";
    ///
    /// webhook.execute(&http, false, |w| w.avatar_url(avatar_url).content("Here's a webhook")).await?;
    /// #     Ok(())
    /// # }
    /// ```
    pub fn avatar_url<S: ToString>(&mut self, avatar_url: S) -> &mut Self {
        self.0.insert("avatar_url", Value::String(avatar_url.to_string()));
        self
    }

    /// Set the content of the message.
    ///
    /// Note that when setting at least one embed via [`Self::embeds`], this may be
    /// omitted.
    ///
    /// # Examples
    ///
    /// Sending a webhook with a content of `"foo"`:
    ///
    /// ```rust,no_run
    /// # use serenity::http::Http;
    /// #
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let http = Http::default();
    /// # let webhook = http.get_webhook_with_token(0, "").await?;
    /// #
    /// let execution = webhook.execute(&http, false, |w| w.content("foo")).await;
    ///
    /// if let Err(why) = execution {
    ///     println!("Err sending webhook: {:?}", why);
    /// }
    /// #     Ok(())
    /// # }
    /// ```
    pub fn content<S: ToString>(&mut self, content: S) -> &mut Self {
        self.0.insert("content", Value::String(content.to_string()));
        self
    }

    /// Appends a file to the webhook message.
    #[cfg(feature = "http")]
    pub fn add_file<T: Into<AttachmentType<'a>>>(&mut self, file: T) -> &mut Self {
        self.1.push(file.into());
        self
    }

    /// Appends a list of files to the webhook message.
    #[cfg(feature = "http")]
    pub fn add_files<T: Into<AttachmentType<'a>>, It: IntoIterator<Item = T>>(
        &mut self,
        files: It,
    ) -> &mut Self {
        self.1.extend(files.into_iter().map(|f| f.into()));
        self
    }

    /// Sets a list of files to include in the webhook message.
    ///
    /// Calling this multiple times will overwrite the file list.
    /// To append files, call [`Self::add_file`] or [`Self::add_files`] instead.
    #[cfg(feature = "http")]
    pub fn files<T: Into<AttachmentType<'a>>, It: IntoIterator<Item = T>>(
        &mut self,
        files: It,
    ) -> &mut Self {
        self.1 = files.into_iter().map(|f| f.into()).collect();
        self
    }

    /// Set the embeds associated with the message.
    ///
    /// This should be used in combination with [`Embed::fake`], creating one
    /// or more fake embeds to send to the API.
    ///
    /// # Examples
    ///
    /// Refer to the [struct-level documentation] for an example on how to use
    /// embeds.
    ///
    /// [`Embed::fake`]: crate::model::channel::Embed::fake
    /// [`Webhook::execute`]: crate::model::webhook::Webhook::execute
    /// [struct-level documentation]: #examples
    pub fn embeds(&mut self, embeds: Vec<Value>) -> &mut Self {
        self.0.insert("embeds", Value::Array(embeds));
        self
    }

    /// Whether the message is a text-to-speech message.
    ///
    /// # Examples
    ///
    /// Sending a webhook with text-to-speech enabled:
    ///
    /// ```rust,no_run
    /// # use serenity::http::Http;
    /// #
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let http = Http::default();
    /// # let webhook = http.get_webhook_with_token(0, "").await?;
    /// #
    /// let execution = webhook.execute(&http, false, |w| w.content("hello").tts(true)).await;
    ///
    /// if let Err(why) = execution {
    ///     println!("Err sending webhook: {:?}", why);
    /// }
    /// #     Ok(())
    /// # }
    /// ```
    pub fn tts(&mut self, tts: bool) -> &mut Self {
        self.0.insert("tts", Value::Bool(tts));
        self
    }

    /// Override the default username of the webhook.
    ///
    /// # Examples
    ///
    /// Overriding the username to `"hakase"`:
    ///
    /// ```rust,no_run
    /// # use serenity::http::Http;
    /// #
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let http = Http::default();
    /// # let webhook = http.get_webhook_with_token(0, "").await?;
    /// #
    /// let execution = webhook.execute(&http, false, |w| w.content("hello").username("hakase")).await;
    ///
    /// if let Err(why) = execution {
    ///     println!("Err sending webhook: {:?}", why);
    /// }
    /// #     Ok(())
    /// # }
    /// ```
    pub fn username<S: ToString>(&mut self, username: S) -> &mut Self {
        self.0.insert("username", Value::String(username.to_string()));
        self
    }

    /// Sets the flags for the message.
    ///
    /// # Examples
    ///
    /// Suppressing an embed on the message.
    ///
    /// ```rust,no_run
    /// # use serenity::http::Http;
    /// # use serenity::model::channel::MessageFlags;
    /// #
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let http = Http::default();
    /// # let webhook = http.get_webhook_with_token(0, "").await?;
    /// #
    /// let execution = webhook
    ///     .execute(&http, false, |w| {
    ///         w.content("https://docs.rs/serenity/latest/serenity/")
    ///             .flags(MessageFlags::SUPPRESS_EMBEDS)
    ///     })
    ///     .await;
    ///
    /// if let Err(why) = execution {
    ///     println!("Err sending webhook: {:?}", why);
    /// }
    /// #     Ok(())
    /// # }
    /// ```
    pub fn flags(&mut self, flags: MessageFlags) -> &mut Self {
        self.0.insert("flags", Value::Number(serde_json::Number::from(flags.bits)));
        self
    }
}

impl<'a> Default for ExecuteWebhook<'a> {
    /// Returns a default set of values for a [`Webhook`] execution.
    ///
    /// The only default value is [`Self::tts`] being set to `false`.
    ///
    /// # Examples
    ///
    /// Creating an [`ExecuteWebhook`] builder:
    ///
    /// ```rust
    /// use serenity::builder::ExecuteWebhook;
    ///
    /// let executor = ExecuteWebhook::default();
    /// ```
    ///
    /// [`Webhook`]: crate::model::webhook::Webhook
    fn default() -> ExecuteWebhook<'a> {
        let mut map = HashMap::new();
        map.insert("tts", Value::Bool(false));

        ExecuteWebhook(map, Default::default())
    }
}
