/// Messages sent to Discord bot by the GitHub bot.
pub enum Discord {
    /// Sent to trigger a status update whenever the github bot does an issue
    /// refresh.
    StatusUpdate(Option<String>),
}
