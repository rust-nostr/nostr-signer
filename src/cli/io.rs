// Copyright (c) 2023 Yuki Kishimoto
// Distributed under the MIT software license

use dialoguer::{Confirm, Input, Password};
use nostr_sdk::Result;

pub fn get_input<S>(prompt: S) -> Result<String>
where
    S: Into<String>,
{
    Ok(Input::new().with_prompt(prompt).interact_text()?)
}

pub fn get_secret_key() -> Result<String> {
    Ok(Password::new().with_prompt("Secret key").interact()?)
}

/* pub fn get_password_with_confirmation() -> Result<String> {
    Ok(Password::new()
        .with_prompt("New password")
        .with_confirmation("Confirm password", "Passwords mismatching")
        .interact()?)
} */

pub fn ask<S>(prompt: S) -> Result<bool>
where
    S: Into<String> + std::marker::Copy,
{
    if Confirm::new()
        .with_prompt(prompt)
        .default(false)
        .interact()?
    {
        Ok(true)
    } else {
        Ok(false)
    }
}
