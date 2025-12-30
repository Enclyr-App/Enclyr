// SPDX-License-Identifier: AGPL-3.0-only

#[derive(Debug, Clone)]
pub struct EmailAccount {
    pub email: String,
    pub imap_host: String,
    pub imap_port: u16,
    pub username: String,
    pub password: String, // in-memory only
}
