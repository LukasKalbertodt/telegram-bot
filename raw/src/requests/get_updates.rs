use crate::requests::*;
use crate::types::*;

/// Use this method to receive incoming updates using long polling.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Default)]
#[must_use = "requests do nothing unless sent"]
pub struct GetUpdates {
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<Integer>, // TODO(knsd): Values between 1—100 are accepted
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<Integer>, // TODO(knsd): Should be positive
    allowed_updates: Vec<AllowedUpdate>, // TODO(knsd) BitSet? HashSet? BTreeSet?
}

impl Request for GetUpdates {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<Vec<Update>>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("getUpdates"), self)
    }
}

impl GetUpdates {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn offset(&mut self, offset: Integer) -> &mut Self {
        self.offset = Some(offset);
        self
    }

    pub fn limit(&mut self, limit: Integer) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    pub fn timeout(&mut self, timeout: Integer) -> &mut Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn allowed_updates(&mut self, updates: &[AllowedUpdate]) -> &mut Self {
        self.allowed_updates = updates.to_vec();
        self
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub enum AllowedUpdate {
    #[serde(rename = "message")]
    Message,
    #[serde(rename = "edited_message")]
    EditedMessage,
    #[serde(rename = "channel_post")]
    ChannelPost,
    #[serde(rename = "edited_channel_post")]
    EditedChannelPost,
    #[serde(rename = "inline_query")]
    InlineQuery,
    #[serde(rename = "chosen_inline_query")]
    ChosenInlineResult,
    #[serde(rename = "callback_query")]
    CallbackQuery,
    #[serde(rename = "shipping_query")]
    ShippingQuery,
    #[serde(rename = "pre_checkout_query")]
    PreCheckoutQuery,
}
