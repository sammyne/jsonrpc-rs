use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct X {
    #[serde(with = "super")]
    data: Vec<u8>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Y {
    #[serde(with = "super::optionals", skip_serializing_if = "Option::is_none")]
    data: Option<Vec<u8>>,
}

impl X {
    pub fn new(data: &[u8]) -> Self {
        Self {
            data: data.to_vec(),
        }
    }
}

impl Y {
    pub fn none() -> Self {
        Self { data: None }
    }

    pub fn some(data: &[u8]) -> Self {
        Self {
            data: Some(data.to_vec()),
        }
    }
}

#[test]
fn marshal() {
    // in form of vec![(input, expect)]
    let test_vector = vec![
        (X::new(&[1, 2, 3, 4]), r#"{"data":"AQIDBA=="}"#),
        (X::new(&[]), r#"{"data":""}"#),
    ];

    for (i, v) in test_vector.iter().enumerate() {
        let got = serde_json::to_string(&v.0)
            .map_err(|err| format!("#{} to_string: {}", i, err))
            .unwrap();

        assert_eq!(v.1, &got, "#{}", i);
    }
}

#[test]
fn marshal_optionals() {
    // in form of vec![(input, expect)]
    let test_vector = vec![
        (Y::some(&[1, 2, 3, 4]), r#"{"data":"AQIDBA=="}"#),
        (Y::some(&[]), r#"{"data":""}"#),
        (Y::none(), r#"{}"#),
    ];

    for (i, v) in test_vector.iter().enumerate() {
        let got = serde_json::to_string(&v.0)
            .map_err(|err| format!("#{} to_string: {}", i, err))
            .unwrap();

        assert_eq!(v.1, &got, "#{}", i);
    }
}

#[test]
fn unmarshal() {
    let test_vector = vec![
        (
            serde_json::json!({ "data": "AQIDBA==" }),
            X::new(&[1, 2, 3, 4]),
        ),
        (serde_json::json!({ "data": "" }), X::new(&[])),
    ];

    for (i, v) in test_vector.iter().enumerate() {
        let got: X = serde_json::from_value(v.0.clone())
            .map_err(|err| format!("#{} unmarshal: {}", i, err))
            .unwrap();

        assert_eq!(v.1, got, "#{}", i);
    }
}

#[test]
fn unmarshal_optionals() {
    let test_vector = vec![
        (
            serde_json::json!({ "data": "AQIDBA==" }),
            Y::some(&[1, 2, 3, 4]),
        ),
        (serde_json::json!({ "data": "" }), Y::some(&[])),
    ];

    for (i, v) in test_vector.iter().enumerate() {
        let got: Y = serde_json::from_value(v.0.clone())
            .map_err(|err| format!("#{} unmarshal: {}", i, err))
            .unwrap();

        assert_eq!(v.1, got, "#{}", i);
    }
}
