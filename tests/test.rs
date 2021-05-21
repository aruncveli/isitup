#[test]
fn test_isitup_up() {
    let response = isitup::isitup("google.com".to_string());
    assert!(response.is_ok());
    assert_eq!(response.unwrap().status_code, 1);
}

#[test]
fn test_isitup_down() {
    let response = isitup::isitup("blah.com".to_string());
    assert!(response.is_ok());
    assert_eq!(response.unwrap().status_code, 2);
}

#[test]
fn test_isitup_invalid_domain() {
    let response = isitup::isitup("blahblah".to_string());
    assert!(response.is_err());
}
