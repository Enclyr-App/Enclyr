// SPDX-License-Identifier: AGPL-3.0-only

use imap::ClientBuilder;
use native_tls::TlsConnector;

use crate::account::account::EmailAccount;
use crate::error::EnclyrError;

pub struct ImapClient;

impl ImapClient {
    pub fn list_folders(account: &EmailAccount) -> Result<Vec<String>, EnclyrError> {
        let tls = TlsConnector::builder()
            .build()
            .map_err(|_| EnclyrError::Imap)?;

        let client = ClientBuilder::new(&account.imap_host, account.imap_port)
            .connect(|domain, tcp| tls.connect(domain, tcp))
            .map_err(|_| EnclyrError::Imap)?;

        let mut session = client
            .login(&account.username, &account.password)
            .map_err(|_| EnclyrError::Imap)?
            .0;

        let folders = session
            .list(None, Some("*"))
            .map_err(|_| EnclyrError::Imap)?;

        let names = folders
            .iter()
            .map(|f| f.name().to_string())
            .collect();

        session.logout().ok();

        Ok(names)
    }
}
