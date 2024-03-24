/*
 * SPDX-FileCopyrightText: 2024 Philip Molares <philip.molares@udo.edu>
 *
 * SPDX-License-Identifier: MIT
 */

use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Scryfall error {:?}", error))]
    ScryfallError { error: scryfall::Error },

    #[snafu(display("Megalodon error {:?}", error))]
    MegalodonError { error: megalodon::error::Error },

    #[snafu(display("Teloxide error {:?}", error))]
    TeloxideError { error: teloxide_core::RequestError },

    #[snafu(display("Unable to read configuration variable '{}'", key))]
    ReadConfiguration { key: String },

    #[snafu(display("Unable to find image png"))]
    ImageNotFound,

    #[snafu(display("Unable to find text"))]
    TextNotFound,

    #[snafu(display("Requested card layout {:?} is not known", layout))]
    UnknownCardLayout { layout: scryfall::card::Layout },

    #[snafu(display("Unable to upload file {}", file_name))]
    ImageUploadFailed { file_name: String },

    #[snafu(display("Can't rotate image"))]
    ImageRotationFailed,
}

impl From<reqwest::Error> for Error {
    fn from(_: reqwest::Error) -> Self {
        Error::ImageNotFound
    }
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Error::ImageNotFound
    }
}

impl From<scryfall::Error> for Error {
    fn from(error: scryfall::Error) -> Self {
        Error::ScryfallError { error: error }
    }
}

impl From<megalodon::error::Error> for Error {
    fn from(error: megalodon::error::Error) -> Self {
        Error::MegalodonError { error: error }
    }
}

impl From<teloxide_core::RequestError> for Error {
    fn from(error: teloxide_core::RequestError) -> Self {
        Error::TeloxideError { error: error }
    }
}

impl From<image::ImageError> for Error {
    fn from(_: image::ImageError) -> Self {
        Error::ImageRotationFailed
    }
}

pub type Result<T> = std::result::Result<T, Error>;
