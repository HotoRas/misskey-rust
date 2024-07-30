#![feature(never_type)]
// Update to Rust 2024 and remove this macro before release
#![allow(non_snake_case, non_camel_case_types)]

mod consts;
pub use consts::{
    followersVisibilities, followingVisibilities, moderationLogTypes, mutedNoteReasons,
    noteVisibilities, notificationTypes, permissions,
};