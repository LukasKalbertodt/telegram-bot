/// Create a reply markup.
///
/// # Examples
/// ```
/// # #[macro_use] extern crate telegram_bot;
/// # fn main() {
/// let reply_keyboard = reply_markup!(reply_keyboard, selective, one_time, resize,
///     ["button", "button"],
///     ["button" contact], // Request contact
///     ["button" location] // Request location
/// );
///
/// let remove_keyboard = reply_markup!(remove_keyboard);
/// let selective_remove_keyboard = reply_markup!(remove_keyboard, selective);
///
/// let force_reply = reply_markup!(force_reply);
/// let selective_force_reply = reply_markup!(force_reply, selective);
///
/// let inline_keyboard = reply_markup!(inline_keyboard,
///     ["button" callback "0,0", "button" callback "0,1"],
///     ["button" callback "1,0", "button" callback "1,1", "button" callback "1,2"]
/// );
///
/// # drop(inline_keyboard);
/// # drop(selective_force_reply);
/// # drop(force_reply);
/// # drop(selective_remove_keyboard);
/// # drop(remove_keyboard);
/// # drop(reply_keyboard);
/// # }
/// ```
#[macro_export]
macro_rules! reply_markup {
    (remove_keyboard) => ({
          $crate::ReplyKeyboardRemove::default()
    });

    (remove_keyboard, selective) => ({
          let mut keyboard = reply_markup!(remove_keyboard);
          keyboard.selective();
          keyboard
    });

    (force_reply) => ({
          $crate::ForceReply::default()
    });

    (force_reply, selective) => ({
          let mut keyboard = reply_markup!(force_reply);
          keyboard.selective();
          keyboard
    });

    (reply_keyboard, $($content:tt)*) => ({
        reply_markup!(_reply_keyboard, $($content)*)
    });

    (_reply_keyboard, resize, $($content:tt)*) => ({
        let mut keyboard = reply_markup!(_reply_keyboard, $($content)*);
        keyboard.resize_keyboard();
        keyboard
    });

    (_reply_keyboard, one_time, $($content:tt)*) => ({
        let mut keyboard = reply_markup!(_reply_keyboard, $($content)*);
        keyboard.one_time_keyboard();
        keyboard
    });

    (_reply_keyboard, selective, $($content:tt)*) => ({
        let mut keyboard = reply_markup!(_reply_keyboard, $($content)*);
        keyboard.selective();
        keyboard
    });

    (_reply_keyboard, $([$($content:tt)*]), *) => (
        $crate::ReplyKeyboardMarkup::from(vec![$(reply_markup![_reply_keyboard_row, $($content)*]), *])
    );

    (_reply_keyboard_row, ($($acc:tt)*); $value:expr) => (vec![$($acc)* reply_markup!(_reply_keyboard_button, $value)]);
    (_reply_keyboard_row, ($($acc:tt)*); $value:expr, $($remaining:tt)*) => (
    reply_markup!(_reply_keyboard_row, ($($acc)* reply_markup!(_reply_keyboard_button,  $value),); $($remaining)*)

    );

    (_reply_keyboard_row, ($($acc:tt)*); $value:tt $request:tt) => (vec![$($acc)* reply_markup!(_reply_keyboard_button,  $value, $request)]);
    (_reply_keyboard_row, ($($acc:tt)*); $value:tt $request:tt, $($remaining: tt)*) => (
        reply_markup!(_reply_keyboard_row, ($($acc)* reply_markup!(_reply_keyboard_button, $value, $request),); $($remaining)*)
    );

    (_reply_keyboard_row, $($content:expr), *) => (vec![$(reply_markup!(_reply_keyboard_button,  $content)), *]);
    (_reply_keyboard_row, $($content:tt)*) => (reply_markup!(_reply_keyboard_row, (); $($content)*));

    (_reply_keyboard_button, $value:expr, contact) => ({
        let mut button: $crate::KeyboardButton = reply_markup!(_reply_keyboard_button, $value);
        button.request_contact();
        button
    });

    (_reply_keyboard_button, $value:expr, location) => ({
        let mut button: $crate::KeyboardButton = reply_markup!(_reply_keyboard_button, $value);
        button.request_location();
        button
    });
    (_reply_keyboard_button, $value:expr) => ($value.into());

    (inline_keyboard, $([$($content:tt)*]), *) => (
        $crate::InlineKeyboardMarkup::from(vec![$(reply_markup![_inline_keyboard_row, $($content)*]), *])
    );

    (_inline_keyboard_row, ($($acc:tt)*); $text:tt $request:tt $callback:tt) => (
        vec![$($acc)* reply_markup!(_inline_keyboard_button, $request,  $text, $callback)]
    );
    (_inline_keyboard_row, $($text:tt $request:tt $callback:tt), *) => (
        vec![$(reply_markup!(_inline_keyboard_button, $request, $text, $callback)), *]
    );
    (_inline_keyboard_row, $($content:tt)*) => (reply_markup!(_inline_keyboard_row, (); $($content)*));

    (_inline_keyboard_button, callback, $text:expr, $callback:expr) => (
        $crate::InlineKeyboardButton::callback($text, $callback)
    );
    (_inline_keyboard_button, url, $text:expr, $url:expr) => (
        $crate::InlineKeyboardButton::url($text, $url)
    );
}

#[cfg(test)]
mod tests {
    use telegram_bot_raw::*;

    #[test]
    fn test_simple() {
        let mut remove_keyboard = ReplyKeyboardRemove::default();
        assert_eq!(remove_keyboard, reply_markup!(remove_keyboard));

        remove_keyboard.selective();
        assert_eq!(remove_keyboard, reply_markup!(remove_keyboard, selective));

        let mut force_reply = ForceReply::default();
        assert_eq!(force_reply, reply_markup!(force_reply));

        force_reply.selective();
        assert_eq!(force_reply, reply_markup!(force_reply, selective));
    }

    #[test]
    fn test_reply_keyboard() {
        let mut keyboard = ReplyKeyboardMarkup::default();
        assert_eq!(keyboard, reply_markup!(reply_keyboard,));

        keyboard.add_empty_row();
        assert_eq!(keyboard, reply_markup!(reply_keyboard, []));

        {
            let row = keyboard.add_empty_row();
            row.push(KeyboardButton::new("foo"));
            row.push(KeyboardButton::new("bar"));
        }
        assert_eq!(keyboard, reply_markup!(reply_keyboard, [], ["foo", "bar"]));

        {
            let row = keyboard.add_empty_row();
            row.push(KeyboardButton::new("baz"));
        }
        assert_eq!(
            keyboard,
            reply_markup!(reply_keyboard, [], ["foo", "bar"], ["baz"])
        );

        {
            let row = keyboard.add_empty_row();

            let mut contact_button = KeyboardButton::new("contact");
            contact_button.request_contact();
            row.push(contact_button);

            let mut location_button = KeyboardButton::new("location");
            location_button.request_location();
            row.push(location_button)
        }
        assert_eq!(
            keyboard,
            reply_markup!(
                reply_keyboard, [], ["foo", "bar"], ["baz"],
                ["contact" contact, "location" location]
            )
        );

        {
            let row = keyboard.add_empty_row();
            row.push(KeyboardButton::new("spam"));
        }
        assert_eq!(
            keyboard,
            reply_markup!(
                reply_keyboard, [], ["foo", "bar"], ["baz"],
                ["contact" contact, "location" location],
                ["spam"]
            )
        );

        keyboard.selective();
        assert_eq!(
            keyboard,
            reply_markup!(
                reply_keyboard, selective, [], ["foo", "bar"], ["baz"],
                ["contact" contact, "location" location],
                ["spam"]
            )
        );

        keyboard.resize_keyboard();
        assert_eq!(
            keyboard,
            reply_markup!(
                reply_keyboard, resize, selective, [], ["foo", "bar"], ["baz"],
                ["contact" contact, "location" location],
                ["spam"]
            )
        );

        keyboard.one_time_keyboard();
        assert_eq!(
            keyboard,
            reply_markup!(
                reply_keyboard, resize, selective, one_time, [], ["foo", "bar"], ["baz"],
                ["contact" contact, "location" location],
                ["spam"]
            )
        );
    }

    #[test]
    fn test_inline_keyboard() {
        let mut markup = InlineKeyboardMarkup::default();
        assert_eq!(markup, reply_markup!(inline_keyboard,));

        markup.add_empty_row();
        assert_eq!(markup, reply_markup!(inline_keyboard, []));

        {
            let row = markup.add_empty_row();
            row.push(InlineKeyboardButton::callback("foo", "bar"));
            row.push(InlineKeyboardButton::callback("baz", "quux"));
        }
        assert_eq!(
            markup,
            reply_markup!(inline_keyboard, [], ["foo" callback "bar", "baz" callback "quux"])
        );
    }
}
