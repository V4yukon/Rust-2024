#![allow(unused)]
use serde_json::json;
use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    hc.do_get("/hello?name=hanghang").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "lanmei1"
        })
    );

    req_login.await?.print().await?;


    Ok(())
}