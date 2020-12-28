use std::cmp::max;
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use std::time::Duration;

use futures::Stream;

use telegram_bot_raw::{AllowedUpdate, GetUpdates, Integer, Update};

use crate::api::Api;
use crate::errors::Error;

const TELEGRAM_LONG_POLL_TIMEOUT_SECONDS: u64 = 5;
const TELEGRAM_LONG_POLL_LIMIT_MESSAGES: Integer = 100;
const TELEGRAM_LONG_POLL_ERROR_DELAY_MILLISECONDS: u64 = 500;

/// This type represents stream of Telegram API updates and uses
/// long polling method under the hood.
#[must_use = "streams do nothing unless polled"]
#[allow(clippy::type_complexity)]
pub struct UpdatesStream {
    api: Api,
    last_update: Integer,
    buffer: VecDeque<Update>,
    current_request:
        Option<Pin<Box<dyn Future<Output = Result<Option<Vec<Update>>, Error>> + Send>>>,
    timeout: Duration,
    allowed_updates: Vec<AllowedUpdate>,
    limit: Integer,
    error_delay: Duration,
    next_poll_id: usize,
}

impl Stream for UpdatesStream {
    type Item = Result<Update, Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let ref_mut = self.get_mut();
        let poll_id = ref_mut.next_poll_id;
        ref_mut.next_poll_id += 1;
        let span = tracing::trace_span!("stream", poll_id = poll_id);
        let _enter = span.enter();

        tracing::trace!("start stream polling");

        if let Some(value) = ref_mut.buffer.pop_front() {
            tracing::trace!(update = ?value, "returning buffered update");
            return Poll::Ready(Some(Ok(value)));
        }
        tracing::trace!("processing request");

        let result = match ref_mut.current_request {
            None => {
                tracing::trace!("there is no current request");
                Ok(false)
            }
            Some(ref mut current_request) => {
                let cc = current_request.as_mut();
                let polled_update = cc.poll(cx);
                match polled_update {
                    Poll::Pending => {
                        tracing::trace!("request is pending");
                        return Poll::Pending;
                    }
                    Poll::Ready(Ok(None)) => {
                        tracing::trace!("request timed out");
                        Ok(false)
                    }
                    Poll::Ready(Ok(Some(ref updates))) if updates.is_empty() => {
                        tracing::trace!("request resolved to empty update list");
                        Ok(false)
                    }
                    Poll::Ready(Ok(Some(updates))) => {
                        for update in updates {
                            tracing::trace!(update = ?update, "processing update");
                            ref_mut.last_update = max(update.id, ref_mut.last_update);
                            tracing::trace!(last_update = ref_mut.last_update);
                            ref_mut.buffer.push_back(update)
                        }

                        Ok(true)
                    }
                    Poll::Ready(Err(err)) => {
                        tracing::error!(error = %err, "request error");
                        Err(err)
                    }
                }
            }
        };

        match result {
            Err(err) => {
                let timeout = ref_mut.timeout + Duration::from_secs(1);
                let mut get_updates = GetUpdates::new();
                get_updates
                    .offset(ref_mut.last_update + 1)
                    .timeout(ref_mut.error_delay.as_secs() as Integer)
                    .limit(ref_mut.limit)
                    .allowed_updates(&ref_mut.allowed_updates);
                tracing::trace!(request = ?get_updates, timeout=?timeout, "preparing new request");

                let request = ref_mut.api.send_timeout(get_updates, timeout);
                ref_mut.current_request = Some(Box::pin(request));
                Poll::Ready(Some(Err(err)))
            }
            Ok(false) => {
                let timeout = ref_mut.timeout + Duration::from_secs(1);
                let mut get_updates = GetUpdates::new();
                get_updates
                    .offset(ref_mut.last_update + 1)
                    .timeout(ref_mut.error_delay.as_secs() as Integer)
                    .limit(ref_mut.limit)
                    .allowed_updates(&ref_mut.allowed_updates);
                tracing::trace!(request = ?get_updates, timeout=?timeout, "preparing new request");

                let request = ref_mut.api.send_timeout(get_updates, timeout);
                ref_mut.current_request = Some(Box::pin(request));

                tracing::trace!("executing recursive call");
                Pin::new(ref_mut).poll_next(cx)
            }
            Ok(true) => {
                tracing::trace!("dropping request");
                ref_mut.current_request = None;
                tracing::trace!("executing recursive call");
                Pin::new(ref_mut).poll_next(cx)
            }
        }
    }
}

impl UpdatesStream {
    ///  create a new `UpdatesStream` instance.
    pub fn new(api: &Api) -> Self {
        UpdatesStream {
            api: api.clone(),
            last_update: 0,
            buffer: VecDeque::new(),
            current_request: None,
            timeout: Duration::from_secs(TELEGRAM_LONG_POLL_TIMEOUT_SECONDS),
            allowed_updates: Vec::new(),
            limit: TELEGRAM_LONG_POLL_LIMIT_MESSAGES,
            error_delay: Duration::from_millis(TELEGRAM_LONG_POLL_ERROR_DELAY_MILLISECONDS),
            next_poll_id: 0,
        }
    }

    /// Set timeout for long polling requests, this corresponds with `timeout` field
    /// in [getUpdates](https://core.telegram.org/bots/api#getupdates) method,
    /// also this stream sets an additional request timeout for `timeout + 1 second`
    /// in case of invalid Telegram API server behaviour.
    ///
    /// Default timeout is 5 seconds.
    pub fn timeout(&mut self, timeout: Duration) -> &mut Self {
        self.timeout = timeout;
        self
    }

    /// Set allowed updates to receive, this corresponds with `allowed_updates` field
    /// in [getUpdates](https://core.telegram.org/bots/api#getupdates) method.
    /// List the types of updates you want your bot to receive. For example,
    /// specify [“message”, “edited_channel_post”, “callback_query”] to only receive updates of these types.
    /// See Update for a complete list of available update types. Specify an empty list to receive all
    /// updates regardless of type (default). If not specified, the previous setting will be used.
    ///
    /// Please note that this parameter doesn't affect updates created before the call to the getUpdates,
    /// so unwanted updates may be received for a short period of time.
    pub fn allowed_updates(&mut self, allowed_updates: &[AllowedUpdate]) -> &mut Self {
        self.allowed_updates = allowed_updates.to_vec();
        self
    }

    /// Set limits the number of updates to be retrieved, this corresponds with `limit` field
    /// in [getUpdates](https://core.telegram.org/bots/api#getupdates) method.
    /// Values between 1—100 are accepted.
    ///
    /// Defaults to 100.
    pub fn limit(&mut self, limit: Integer) -> &mut Self {
        self.limit = limit;
        self
    }

    /// Set a delay between erroneous request and next request.
    /// This delay prevents busy looping in some cases.
    ///
    /// Default delay is 500 ms.
    pub fn error_delay(&mut self, delay: Duration) -> &mut Self {
        self.error_delay = delay;
        self
    }
}
