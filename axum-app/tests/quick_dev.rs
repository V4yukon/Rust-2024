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
            "username": "demo",
            "pwd": "lanmei"
        })
    );

    req_login.await?.print().await?;

    let req_create = hc.do_post(
        "/api/ticket",
        json!({
            "title": "TICKET LANMEI"
        })
    );
    req_create.await?.print().await?;
    

    let req_delete = hc.do_delete(
        "/api/ticket/1"
    );
    req_delete.await?.print().await?;
    hc.do_get("/api/ticket").await?.print().await?;


    Ok(())
}