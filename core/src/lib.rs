#[cfg(test)]
mod tests {
    use crate::account::account::EmailAccount;
    use crate::imap::client::ImapClient;

    #[test]
    fn test_list_folders() {
        let account = EmailAccount {
            email: "test@millionaire.email".into(),
            imap_host: "mail.millionaire.email".into(),
            imap_port: 993,
            username: "test@millionaire.email".into(),
            password: "Millionaire@123".into(),
        };

        let folders = ImapClient::list_folders(&account);
        assert!(folders.is_ok());
    }
}
