use lettre::{EmailAddress, Envelope};
use lettre_email::EmailBuilder;

#[test]
fn build_with_envelope_test() {
    let e = Envelope::new(
        Some(EmailAddress::new("from@example.org".to_string()).unwrap()),
        vec![EmailAddress::new("to@example.org".to_string()).unwrap()],
    )
    .unwrap();
    let _email = EmailBuilder::new()
        .envelope(e)
        .subject("subject")
        .text("message")
        .build()
        .unwrap();
}

#[test]
fn build_with_envelope_without_from_test() {
    let e = Envelope::new(
        None,
        vec![EmailAddress::new("to@example.org".to_string()).unwrap()],
    )
    .unwrap();
    let _email = EmailBuilder::new()
        .envelope(e)
        .subject("subject")
        .text("message")
        .build()
        .unwrap_err();
}
