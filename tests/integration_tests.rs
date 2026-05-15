const USERS: &str = "http://127.0.0.1:7878/users";
const LOGIN: &str = "http://127.0.0.1:7878/login";
const TRANSACTIONS: &str = "http://127.0.0.1:7878/transactions";
const BALANCE: &str = "http://127.0.0.1:7878/balance";

#[tokio::test]
async fn full_flow_test() {
    let client = reqwest::Client::new();

    let user_test = client
        .post(USERS)
        .json(&serde_json::json!({
            "email": "test@test.com",
            "password": "test1234"
        }))
        .send()
        .await
        .expect("Error creating user");
    let status = user_test.status();
    let body = user_test.text().await.unwrap_or_default();
    println!("User Creation results:\n{}\n{}", status, body);
    assert!(status.is_success());


    let login_test = client
        .post(LOGIN)
        .json(&serde_json::json!({
            "email": "test@test.com",
            "password": "test1234"
        }))
        .send()
        .await
        .expect("Error logging in");

    let status = login_test.status();
    let body: serde_json::Value = login_test.json().await.unwrap();
    println!("Login results:\n{}\n{}", status, body);
    assert!(status.is_success());

    let token = body["access_token"].as_str().unwrap();
    let auth_header = format!("Bearer {}", token);
    let transactions_test = client
        .post(TRANSACTIONS)
        .header("Authorization", auth_header.clone())
        .json(&serde_json::json!({
            "amount": 101.01,
            "category": "PERSONAL"
        }))
        .send()
        .await
        .expect("Error posting transaction");

    let status = transactions_test.status();
    println!("Transactions results:\n{}", status);
    assert!(status.is_success());

    let balance_test = client
        .get(BALANCE)
        .header("Authorization", auth_header.clone())
        .send()
        .await
        .expect("Error getting balance");

    let status = balance_test.status();
    let body = balance_test.text().await.unwrap();
    println!("Balance results:\n{}\n{}", status, body);
    assert!(status.is_success());

    client
        .post(TRANSACTIONS)
        .header("Authorization", auth_header.clone())
        .json(&serde_json::json!({
            "amount": 350.99,
            "category": "PAYCHECK"
        }))
        .send()
        .await
        .expect("Error posting transaction");

    client
        .post(TRANSACTIONS)
        .header("Authorization", auth_header.clone())
        .json(&serde_json::json!({
            "amount": -91.30,
            "category": "BUSINESS"
        }))
        .send()
        .await
        .expect("Error posting transaction");

    let balance_test = client
        .get(BALANCE)
        .header("Authorization", auth_header.clone())
        .send()
        .await
        .expect("Error getting balance");

    let status = balance_test.status();
    let body = balance_test.text().await.unwrap();
    println!("Balance should be: {}", 101.01 + 350.99 - 91.30);
    println!("Balance results:\n{}\n{}", status, body);
    assert!(status.is_success());
}