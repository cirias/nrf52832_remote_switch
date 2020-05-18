use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ResponseBody<T> {
    pub ok: bool,
    pub description: Option<String>,
    pub result: Option<T>,
}

#[derive(Serialize, Deserialize)]
pub struct GetUpdatesParams {
    offset:	i64,	// Optional	Identifier of the first update to be returned. Must be greater by one than the highest among the identifiers of previously received updates. By default, updates starting with the earliest unconfirmed update are returned. An update is considered confirmed as soon as getUpdates is called with an offset higher than its update_id. The negative offset can be specified to retrieve updates starting from -offset update from the end of the updates queue. All previous updates will forgotten.
    limit:	i64,	// Optional	Limits the number of updates to be retrieved. Values between 1-100 are accepted. Defaults to 100.
    timeout:	i64,	// Optional	Timeout in seconds for long polling. Defaults to 0, i.e. usual short polling. Should be positive, short polling should be used for testing purposes only.
    allowed_updates:	Vec<String>, // Optional	A JSON-serialized list of the update types you want your bot to receive. For example, specify [“message”, “edited_channel_post”, “callback_query”] to only receive updates of these types. See Update for a complete list of available update types. Specify an empty list to receive all updates regardless of type (default). If not specified, the previous setting will be used.
    // Please note that this parameter doesn't affect updates created before the call to the getUpdates, so unwanted updates may be received for a short period of time.
}

#[derive(Serialize, Deserialize)]
pub struct SendMessageParams {
    chat_id: i64,	// Required. Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    text:	String,	// Required. Text of the message to be sent, 1-4096 characters after entities parsing
    parse_mode:	String,	// Optional	Mode for parsing entities in the message text. See formatting options for more details.
    disable_web_page_preview:	bool,	// Optional	Disables link previews for links in this message
    disable_notification:	bool,	// Optional	Sends the message silently. Users will receive a notification with no sound.
    reply_to_message_id:	i64,	// Optional	If the message is a reply, ID of the original message
    reply_markup:	ReplyKeyboardMarkup, // InlineKeyboardMarkup or ReplyKeyboardMarkup or ReplyKeyboardRemove or ForceReply,	// Optional	Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
}

#[derive(Serialize, Deserialize)]
pub struct ReplyKeyboardMarkup {
    keyboard: Vec<Vec<KeyboardButton>>,	// Array of button rows, each represented by an Array of KeyboardButton objects
    resize_keyboard:	bool,	// Optional. Requests clients to resize the keyboard vertically for optimal fit (e.g., make the keyboard smaller if there are just two rows of buttons). Defaults to false, in which case the custom keyboard is always of the same height as the app's standard keyboard.
    one_time_keyboard:	bool,	// Optional. Requests clients to hide the keyboard as soon as it's been used. The keyboard will still be available, but clients will automatically display the usual letter-keyboard in the chat – the user can press a special button in the input field to see the custom keyboard again. Defaults to false.
    selective:	bool,	//Optional. Use this parameter if you want to show the keyboard to specific users only. Targets: 1) users that are @mentioned in the text of the Message object; 2) if the bot's message is a reply (has reply_to_message_id), sender of the original message.
}

#[derive(Serialize, Deserialize)]
pub struct KeyboardButton {
    text:	String,	// Text of the button. If none of the optional fields are used, it will be sent as a message when the button is pressed
    request_contact: bool,	// Optional. If True, the user's phone number will be sent as a contact when the button is pressed. Available in private chats only
    request_location:	bool,	// Optional. If True, the user's current location will be sent when the button is pressed. Available in private chats only
// request_poll	KeyboardButtonPollType	Optional. If specified, the user will be asked to create a poll and send it to the bot when the button is pressed. Available in private chats only
}

#[derive(Serialize, Deserialize)]
pub struct Update {
    update_id:	i64, // The update‘s unique identifier. Update identifiers start from a certain positive number and increase sequentially. This ID becomes especially handy if you’re using Webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    message:	Message,	// Optional. New incoming message of any kind — text, photo, sticker, etc.
/*
 * edited_message	Message	Optional. New version of a message that is known to the bot and was edited
 * channel_post	Message	Optional. New incoming channel post of any kind — text, photo, sticker, etc.
 * edited_channel_post	Message	Optional. New version of a channel post that is known to the bot and was edited
 * inline_query	InlineQuery	Optional. New incoming inline query
 * chosen_inline_result	ChosenInlineResult	Optional. The result of an inline query that was chosen by a user and sent to their chat partner. Please see our documentation on the feedback collecting for details on how to enable these updates for your bot.
 * callback_query	CallbackQuery	Optional. New incoming callback query
 * shipping_query	ShippingQuery	Optional. New incoming shipping query. Only for invoices with flexible price
 * pre_checkout_query	PreCheckoutQuery	Optional. New incoming pre-checkout query. Contains full information about checkout
 * poll	Poll	Optional. New poll state. Bots receive only updates about stopped polls and polls, which are sent by the bot
 * poll_answer	PollAnswer	Optional. A user changed their answer in a non-anonymous poll. Bots receive new votes only in polls that were sent by the bot itself.
 */
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    message_id: i64,	// Unique message identifier inside this chat
    from: User,	// Optional. Sender, empty for messages sent to channels
    date: i64,	// Date the message was sent in Unix time
    chat: Chat,	// Conversation the message belongs to
/*
 * forward_from	User	Optional. For forwarded messages, sender of the original message
 * forward_from_chat	Chat	Optional. For messages forwarded from channels, information about the original channel
 * forward_from_message_id	Integer	Optional. For messages forwarded from channels, identifier of the original message in the channel
 * forward_signature	String	Optional. For messages forwarded from channels, signature of the post author if present
 * forward_sender_name	String	Optional. Sender's name for messages forwarded from users who disallow adding a link to their account in forwarded messages
 * forward_date	Integer	Optional. For forwarded messages, date the original message was sent in Unix time
 * reply_to_message	Message	Optional. For replies, the original message. Note that the Message object in this field will not contain further reply_to_message fields even if it itself is a reply.
 * edit_date	Integer	Optional. Date the message was last edited in Unix time
 * media_group_id	String	Optional. The unique identifier of a media message group this message belongs to
 * author_signature	String	Optional. Signature of the post author for messages in channels
 * text	String	Optional. For text messages, the actual UTF-8 text of the message, 0-4096 characters
 * entities	Array of MessageEntity	Optional. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
 * animation	Animation	Optional. Message is an animation, information about the animation. For backward compatibility, when this field is set, the document field will also be set
 * audio	Audio	Optional. Message is an audio file, information about the file
 * document	Document	Optional. Message is a general file, information about the file
 * photo	Array of PhotoSize	Optional. Message is a photo, available sizes of the photo
 * sticker	Sticker	Optional. Message is a sticker, information about the sticker
 * video	Video	Optional. Message is a video, information about the video
 * video_note	VideoNote	Optional. Message is a video note, information about the video message
 * voice	Voice	Optional. Message is a voice message, information about the file
 * caption	String	Optional. Caption for the animation, audio, document, photo, video or voice, 0-1024 characters
 * caption_entities	Array of MessageEntity	Optional. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
 * contact	Contact	Optional. Message is a shared contact, information about the contact
 * dice	Dice	Optional. Message is a dice with random value from 1 to 6
 * game	Game	Optional. Message is a game, information about the game. More about games »
 * poll	Poll	Optional. Message is a native poll, information about the poll
 * venue	Venue	Optional. Message is a venue, information about the venue. For backward compatibility, when this field is set, the location field will also be set
 * location	Location	Optional. Message is a shared location, information about the location
 * new_chat_members	Array of User	Optional. New members that were added to the group or supergroup and information about them (the bot itself may be one of these members)
 * left_chat_member	User	Optional. A member was removed from the group, information about them (this member may be the bot itself)
 * new_chat_title	String	Optional. A chat title was changed to this value
 * new_chat_photo	Array of PhotoSize	Optional. A chat photo was change to this value
 * delete_chat_photo	True	Optional. Service message: the chat photo was deleted
 * group_chat_created	True	Optional. Service message: the group has been created
 * supergroup_chat_created	True	Optional. Service message: the supergroup has been created. This field can‘t be received in a message coming through updates, because bot can’t be a member of a supergroup when it is created. It can only be found in reply_to_message if someone replies to a very first message in a directly created supergroup.
 * channel_chat_created	True	Optional. Service message: the channel has been created. This field can‘t be received in a message coming through updates, because bot can’t be a member of a channel when it is created. It can only be found in reply_to_message if someone replies to a very first message in a channel.
 * migrate_to_chat_id	Integer	Optional. The group has been migrated to a supergroup with the specified identifier. This number may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier.
 * migrate_from_chat_id	Integer	Optional. The supergroup has been migrated from a group with the specified identifier. This number may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier.
 * pinned_message	Message	Optional. Specified message was pinned. Note that the Message object in this field will not contain further reply_to_message fields even if it is itself a reply.
 * invoice	Invoice	Optional. Message is an invoice for a payment, information about the invoice. More about payments »
 * successful_payment	SuccessfulPayment	Optional. Message is a service message about a successful payment, information about the payment. More about payments »
 * connected_website	String	Optional. The domain name of the website on which the user has logged in. More about Telegram Login »
 * passport_data	PassportData	Optional. Telegram Passport data
 * reply_markup	InlineKeyboardMarkup	Optional. Inline keyboard attached to the message. login_url buttons are represented as ordinary url buttons.
 */
}

#[derive(Serialize, Deserialize)]
pub struct Chat {
id:	i64, // Unique identifier for this chat. This number may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier.
// type:	String	// Type of chat, can be either “private”, “group”, “supergroup” or “channel”
title:	String,	// Optional. Title, for supergroups, channels and group chats
username:	String,	// Optional. Username, for private chats, supergroups and channels if available
/*
 * first_name	String	Optional. First name of the other party in a private chat
 * last_name	String	Optional. Last name of the other party in a private chat
 * photo	ChatPhoto	Optional. Chat photo. Returned only in getChat.
 * description	String	Optional. Description, for groups, supergroups and channel chats. Returned only in getChat.
 * invite_link	String	Optional. Chat invite link, for groups, supergroups and channel chats. Each administrator in a chat generates their own invite links, so the bot must first generate the link using exportChatInviteLink. Returned only in getChat.
 * pinned_message	Message	Optional. Pinned message, for groups, supergroups and channels. Returned only in getChat.
 * permissions	ChatPermissions	Optional. Default chat member permissions, for groups and supergroups. Returned only in getChat.
 * slow_mode_delay	Integer	Optional. For supergroups, the minimum allowed delay between consecutive messages sent by each unpriviledged user. Returned only in getChat.
 * sticker_set_name	String	Optional. For supergroups, name of group sticker set. Returned only in getChat.
 * can_set_sticker_set	Boolean	Optional. True, if the bot can change the group sticker set. Returned only in getChat.
 */
}

#[derive(Serialize, Deserialize)]
pub struct User {
    id: i64,
    is_bot: bool,
    first_name: String,                // User‘s or bot’s first name
    last_name: String,                 // Optional. User‘s or bot’s last name
    username: String,                  // Optional. User‘s or bot’s username
    /*
     * language_code: String,             // Optional. IETF language tag of the user's language
     * can_join_groups: bool,             // Optional. True, if the bot can be invited to groups. Returned only in getMe.
     * can_read_all_group_messages: bool, // Optional. True, if privacy mode is disabled for the bot. Returned only in getMe.
     * supports_inline_queries: bool,     // Optional. True, if the bot supports inline queries. Returned only in getMe.
     */
}
