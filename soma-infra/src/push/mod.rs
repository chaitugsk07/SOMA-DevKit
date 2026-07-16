//! Push notification clients: APNs, FCM, Web Push (VAPID), Expo, Telegram, and WhatsApp.
//!
//! Pure mechanism: raw send + typed errors that surface the provider's reason/status.
//! No device-token lifecycle, no fanout, no retry policy — those are the consuming
//! service's policy.
//!
//! # Feature
//!
//! Enabled via `features = ["push"]` in `Cargo.toml`.
//!
//! # Modules
//!
//! | module | client | protocol |
//! |--------|--------|----------|
//! | [`apns`] | [`apns::ApnsClient`] | APNs HTTP/2 + ES256 provider JWT |
//! | [`fcm`] | [`fcm::FcmClient`] | FCM HTTP v1 + RS256 → OAuth2 access token |
//! | [`vapid`] | [`vapid::WebPushClient`] | RFC 8291/8292: aes128gcm + VAPID JWT |
//! | [`expo`] | [`expo::ExpoClient`] | Expo Push API (exp.host, no server credentials) |
//! | [`telegram`] | [`telegram::TelegramClient`] | Telegram Bot API sendMessage |
//! | [`whatsapp`] | [`whatsapp::WhatsAppClient`] | WhatsApp Business Cloud API |

pub mod apns;
pub mod expo;
pub mod fcm;
pub mod telegram;
pub mod vapid;
pub mod whatsapp;
