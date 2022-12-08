pub mod authlib {
    use reqwest::{Client, /*Response*/};
    //use serde::{Deserialize, Serialize};
    use serde_json::{json,Value};

    pub struct AuthlibAuthContext {
        pub authlib_url: String,

        pub agent_name: String,
        pub agent_version: String,
        pub username: String,
        pub password: String,
        pub client_token: String,
        pub request_user: bool,

        pub access_token: String,
        pub selected_profile: AuthlibAuthProfile,
        //pub available_profiles: Vec<AuthlibAuthProfile>,
        pub user: AuthlibAuthUser,
    }

    pub struct AuthlibAuthProfile {
        pub agent : String ,
        pub id : String ,
        pub name : String ,
        pub user_id : String ,
        pub created_at : String ,
        pub legacy_profile :bool ,
        pub suspended : bool ,
        pub paid : bool ,
        pub migrated : bool ,
        pub legacy : bool ,
    }

    pub struct AuthlibAuthUser {
        pub id : String ,
        pub email : String ,
        pub username : String ,
        pub register_ip : String ,
        pub migrated_from : String ,
        pub migrated_at : String ,
        pub registered_at : String ,
        pub password_changed_at : String ,
        pub date_of_birth : String ,
        pub suspended : bool ,
        pub blocked : bool ,
        pub secured : bool ,
        pub migrated : bool ,
        pub email_verified : bool ,
        pub legacy_user : bool ,
        pub verified_by_parent : bool ,
        pub perferred_language : String ,
    }

    pub async fn get_authlib_access_token(context: AuthlibAuthContext) -> Result<AuthlibAuthContext, reqwest::Error> {
        let req = json!({
            "agent" : {
                "name" : context.agent_name,
                "version" : context.agent_version
            },
            "username" : context.username,
            "password" : context.password,
            "clientToken" : context.client_token,
            "requestUser" : context.request_user
        });
        // Get authlib access token
        let client = Client::new();
        let resp = client.post(context.authlib_url.to_owned()+"/authenticate")
            .body(req.to_string())
            .header("Content-Type", "application/json")
            .send()
            .await?;
        // Deserialize response json
        let body = resp.text().await?;
        let json: Value = serde_json::from_slice(body.as_bytes()).unwrap();
        //let json : serde_json::Value = serde_json::from_str(body.as_str()).unwrap()?;
        let result = AuthlibAuthContext {
            authlib_url: context.authlib_url,
            agent_name: context.agent_name,
            agent_version: context.agent_version,
            username: context.username,
            password: context.password,
            client_token: context.client_token,
            request_user: context.request_user,
            access_token: json["accessToken"].to_string(),
            selected_profile: AuthlibAuthProfile {
                agent: json["selectedProfile"]["agent"].to_string(),
                id: json["selectedProfile"]["id"].to_string(),
                name: json["selectedProfile"]["name"].to_string(),
                user_id: json["selectedProfile"]["userId"].to_string(),
                created_at: json["selectedProfile"]["createdAt"].to_string(),
                legacy_profile: json["selectedProfile"]["legacyProfile"].to_string().parse::<bool>().unwrap(),
                suspended: json["selectedProfile"]["suspended"].to_string().parse::<bool>().unwrap(),
                paid: json["selectedProfile"]["paid"].to_string().parse::<bool>().unwrap(),
                migrated: json["selectedProfile"]["migrated"].to_string().parse::<bool>().unwrap(),
                legacy: json["selectedProfile"]["legacy"].to_string().parse::<bool>().unwrap(),
            },
            user: AuthlibAuthUser {
                id: json["user"]["id"].to_string(),
                email: json["user"]["email"].to_string(),
                username: json["user"]["username"].to_string(),
                register_ip: json["user"]["registerIp"].to_string(),
                migrated_from: json["user"]["migratedFrom"].to_string(),
                migrated_at: json["user"]["migratedAt"].to_string(),
                registered_at: json["user"]["registeredAt"].to_string(),
                password_changed_at: json["user"]["passwordChangedAt"].to_string(),
                date_of_birth: json["user"]["dateOfBirth"].to_string(),
                suspended: json["user"]["suspended"].to_string().parse::<bool>().unwrap(),
                blocked: json["user"]["blocked"].to_string().parse::<bool>().unwrap(),
                secured: json["user"]["secured"].to_string().parse::<bool>().unwrap(),
                migrated: json["user"]["migrated"].to_string().parse::<bool>().unwrap(),
                email_verified: json["user"]["emailVerified"].to_string().parse::<bool>().unwrap(),
                legacy_user: json["user"]["legacyUser"].to_string().parse::<bool>().unwrap(),
                verified_by_parent: json["user"]["verifiedByParent"].to_string().parse::<bool>().unwrap(),
                perferred_language: json["user"]["properties"][0]["value"].to_string(),
            },
        };
        Ok(result)
    }

    pub async fn refresh_authlib_access_token(context: AuthlibAuthContext) -> Result<AuthlibAuthContext, reqwest::Error> {
        let req = json!({
            "accessToken" : context.access_token,
            "clientToken" : context.client_token,
            "requestUser" : context.request_user
        });
        // Refresh authlib access token
        let client = Client::new();
        let resp = client.post(context.authlib_url.to_owned()+"/refresh")
            .body(req.to_string())
            .header("Content-Type", "application/json")
            .send()
            .await?;
        // Deserialize response json
        let body = resp.text().await?;
        let json: Value = serde_json::from_slice(body.as_bytes()).unwrap();
        //let json : serde_json::Value = serde_json::from_str(body.as_str()).unwrap()?;
        let result = AuthlibAuthContext {
            authlib_url: context.authlib_url,
            access_token : json["accessToken"].to_string(),
            client_token : context.client_token,
            request_user : context.request_user,
            agent_name : context.agent_name,
            agent_version : context.agent_version,
            username : context.username,
            password : context.password,
            selected_profile : AuthlibAuthProfile {
                agent : json["selectedProfile"]["agent"].to_string(),
                id : json["selectedProfile"]["id"].to_string(),
                name : json["selectedProfile"]["name"].to_string(),
                user_id : json["selectedProfile"]["userId"].to_string(),
                created_at : json["selectedProfile"]["createdAt"].to_string(),
                legacy_profile : json["selectedProfile"]["legacyProfile"].to_string().parse::<bool>().unwrap(),
                suspended : json["selectedProfile"]["suspended"].to_string().parse::<bool>().unwrap(),
                paid : json["selectedProfile"]["paid"].to_string().parse::<bool>().unwrap(),
                migrated : json["selectedProfile"]["migrated"].to_string().parse::<bool>().unwrap(),
                legacy : json["selectedProfile"]["legacy"].to_string().parse::<bool>().unwrap(),
            },
            user : AuthlibAuthUser {
                id : json["user"]["id"].to_string(),
                email : context.user.email,
                username : context.user.username,
                register_ip : context.user.register_ip,
                migrated_from : context.user.migrated_from,
                migrated_at : context.user.migrated_at,
                registered_at : context.user.registered_at,
                password_changed_at : context.user.password_changed_at,
                date_of_birth : context.user.date_of_birth,
                suspended : context.user.suspended,
                blocked : context.user.blocked,
                secured : context.user.secured,
                migrated : context.user.migrated,
                email_verified : context.user.email_verified,
                legacy_user : context.user.legacy_user,
                verified_by_parent : context.user.verified_by_parent,
                perferred_language : context.user.perferred_language,
            },
        };
        Ok(result)
    }

    pub async fn validate_authlib_access_token(context: AuthlibAuthContext) -> Result<bool , reqwest::Error> {
        let req = json!({
            "accessToken" : context.access_token,
            "clientToken" : context.client_token
        });
        // Validate authlib access token
        let client = Client::new();
        let resp = client.post(context.authlib_url.to_owned()+"/validate")
            .body(req.to_string())
            .header("Content-Type", "application/json")
            .send()
            .await?;
        if resp.status() == 403  {
            return Ok(false);
        }
        Ok(true)
    }

    pub async fn signout_authlib_access_token(context: AuthlibAuthContext) -> Result<bool , reqwest::Error> {
        let req = json!({
            "username" : context.username,
            "password" : context.password
        });
        // Signout authlib access token
        let client = Client::new();
        let resp = client.post(context.authlib_url.to_owned()+"/signout")
            .body(req.to_string())
            .header("Content-Type", "application/json")
            .send()
            .await?;
        if resp.bytes().await?.is_empty() {
            return Ok(true);
        }
        Ok(false)
    }

    pub async fn invalidate_authlib_access_token(context: AuthlibAuthContext) -> Result<bool , reqwest::Error> {
        let req = json!({
            "access_token" : context.access_token,
            "client_token" : context.client_token
        });
        // Invalidate authlib access token
        let client = Client::new();
        let resp = client.post(context.authlib_url.to_owned()+"/invalidate")
            .body(req.to_string())
            .header("Content-Type", "application/json")
            .send()
            .await?;
        if resp.bytes().await?.is_empty() {
            return Ok(true);
        }
        Ok(false)
    }
}