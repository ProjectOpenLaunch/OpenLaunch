pub mod microsoft {
    use reqwest::{Client, /*Response*/};
    use std::string::*;
    //use serde::{Deserialize, Serialize};
    use serde_json::{json , Value};

    pub struct LiveAuthcontext {
        pub auth_code : String ,
        pub token_type : String ,
        pub expires_in : String ,
        pub access_token : String ,
        pub refresh_token : String ,
        pub scope : String ,
        pub user_id : String ,
        pub foci : String ,
    }

    pub struct LiveAuthXBLcontext {
        pub issue_instant : String ,
        pub not_after : String ,
        pub token : String ,
        pub uhs : String ,
    }

    pub struct LiveAuthXSTScontext {
        pub issue_instant : String ,
        pub not_after : String ,
        pub token : String ,
        pub uhs : String , 
    }

    pub struct LiveAuthMCcontext {
        pub username : String ,
        pub access_token : String ,
        pub token_type : String ,
        pub expires_in : String ,
    }

    pub async fn get_live_access_token(context : LiveAuthcontext) -> Result<LiveAuthcontext,reqwest::Error> {
        let req = json!({
            "client_id" : "00000000402b5328",
            "code" : context.auth_code,
            "grant_type" : "authorization_code",
            "redirect_uri" : "https://login.live.com/oauth20_desktop.srf",
            "scope" : "service::user.auth.xboxlive.com::MBI_SSL"
        });
        // Get live access token
        let client = Client::new();
        let resp = client.post("https://login.live.com/oauth20_token.srf")
        .body(req.to_string())
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send()
        .await?;
        // Deserialize response json
        let body = resp.text().await?;
        let json : Value = serde_json::from_slice(body.as_bytes()).unwrap();
        //let json : serde_json::Value = serde_json::from_str(body.as_str()).unwrap()?;
        let result = LiveAuthcontext {
            auth_code : context.auth_code,
            token_type : json["token_type"].to_string(),
            expires_in : json["expires_in"].to_string(),
            access_token : json["access_token"].to_string(),
            refresh_token : json["refresh_token"].to_string(),
            scope : json["scope"].to_string(),
            user_id : json["user_id"].to_string(),
            foci : json["foci"].to_string(),
        };
        return Ok(result);
    }

    pub async fn get_xbl_auth(context : LiveAuthcontext) -> Result<LiveAuthXBLcontext,reqwest::Error> {
        let req = json!({
            "Properties" : {
                "AuthMethod" : "RPS",
                "SiteName" : "user.auth.xboxlive.com",
                "RpsTicket" : "d".to_string()+context.access_token.as_str()
            },
            "RelyingParty" : "http://auth.xboxlive.com",
            "TokenType" : "JWT",
        });
        // Get live access token
        let client = Client::new();
        let resp = client.post("https://login.live.com/oauth20_token.srf")
        .body(req.to_string())
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Accept", "application/json")
        .send()
        .await?;
        // Deserialize response json
        let body = resp.text().await?;
        let json : Value = serde_json::from_slice(body.as_bytes()).unwrap();
        let result = LiveAuthXBLcontext {
            issue_instant : json["IssueInstant"].to_string(),
            not_after : json["NotAfter"].to_string(),
            token : json["Token"].to_string(),
            uhs : json["DisplayClaims"]["xui"][0]["uhs"].to_string(),
        };
        return Ok(result);
    }

    pub async fn get_xsts_token(context : LiveAuthXBLcontext) -> Result<LiveAuthXSTScontext , reqwest::Error> {
        let req = json!({
            "Properties" : {
                "SandboxId" : "RETAIL",
                "UserTokens" : [
                    context.token
                ],
                "RelyingParty" : "rp://api.minecraftservices.com/",
                "TokenType" : "JWT"
            }
        });
        // Get live access token
        let client = Client::new();
        let resp = client.post("https://login.live.com/oauth20_token.srf")
        .body(req.to_string())
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Accept", "application/json")
        .send()
        .await?;
        // Deserialize response json
        let body = resp.bytes().await?;
        let json : Value = serde_json::from_slice(&body).unwrap();
        let result = LiveAuthXSTScontext {
            issue_instant : json["IssueInstant"].to_string(),
            not_after : json["NotAfter"].to_string(),
            token : json["Token"].to_string(),
            uhs : json["DisplayClaims"]["xui"][0]["uhs"].to_string(),
        };
        return Ok(result);
    }

    pub async fn get_minecraft_token(context : LiveAuthXSTScontext) -> Result<LiveAuthMCcontext , reqwest::Error> {
        let req = json!({
            "identityToken" : "XBL3.0 x=".to_string()+context.uhs.as_str()+";"+context.token.as_str()
        });
        // Get live access token
        let client = Client::new();
        let resp = client.post("https://api.minecraftservices.com/authentication/login_with_xbox")
        .body(req.to_string())
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .send()
        .await?;
        // Deserialize response json
        let body = resp.bytes().await?;
        let json : Value = serde_json::from_slice(&body).unwrap();
        let result = LiveAuthMCcontext {
            username : json["username"].to_string(),
            access_token : json["access_token"].to_string(),
            token_type : json["token_type"].to_string(),
            expires_in : json["expires_in"].to_string(),
        };
        return Ok(result);
    }
}