// SPDX-License-Identifier: AGPL-3.0-only

use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnclyrError {
    #[error("IMAP error")]
    Imap,

    #[error("SMTP error")]
    Smtp,

    #[error("Storage error")]
    Storage,

    #[error("MIME parse error")]
    Mime,
}
