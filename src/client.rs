use std::error::Error;

use lazy_static::lazy_static;
use regex::Regex;
use reqwest::{self, Url};

use crate::regions::Regions;
use crate::uid::UID;
use crate::errors::*;
use crate::cookie::Cookie;

pub struct Client {
    cookie: String,
    lang: String,
    http: reqwest::Client,
    uid: UID
}

impl Client {
    pub fn new(cookie: Cookie, uid: &str) -> Result<Self, CookieError> {
        let destructured_cookie = {
            match cookie {
                Cookie::CookieString(cookie_string) => Self::destructure_cookie(&cookie_string),
                Cookie::CookieParsed(ltuid, ltoken, cookie_token, account_id, lang) => Ok((format!("ltuid={ltuid}; ltoken={ltoken}; cookie_token={cookie_token}; account_id={account_id}; mi18nLang={lang}"), 
                                                                                                                                   ltuid, ltoken, cookie_token, account_id, lang))
            }
        };

        if let Ok((cookie, _, _, _, _, lang)) = destructured_cookie {
            Ok(Client {
                cookie, lang,
                http: reqwest::Client::new(),
                uid: UID(String::from(uid))
            })
        } else {
            Err(destructured_cookie.unwrap_err())
        }        
    }

    pub fn destructure_cookie(cookie: &str) -> Result<(String, String, String, String, String, String), CookieError> {
        lazy_static! {
            static ref REGEX_LTUID: Regex = Regex::new(r"ltuid=([^;]+)").unwrap();
            static ref REGEX_LTOKEN: Regex = Regex::new(r"ltoken=([^;]+)").unwrap();
            static ref REGEX_COOKIE_TOKEN: Regex = Regex::new(r"cookie_token=([^;]+)").unwrap();
            static ref REGEX_ACCOUNT_ID: Regex = Regex::new(r"account_id=([^;]+)").unwrap();
            static ref REGEX_LANG: Regex = Regex::new(r"mi18nLang=([^;]+)").unwrap();
        }

        let ltuid = String::from(REGEX_LTUID.captures(cookie).unwrap().get(1).unwrap().as_str());
        let ltoken = String::from(REGEX_LTOKEN.captures(cookie).unwrap().get(1).unwrap().as_str());
        let cookie_token = String::from(REGEX_COOKIE_TOKEN.captures(cookie).unwrap().get(1).unwrap().as_str());
        let account_id = String::from(REGEX_ACCOUNT_ID.captures(cookie).unwrap().get(1).unwrap().as_str());
        let lang = String::from(REGEX_LANG.captures(cookie).unwrap().get(1).unwrap().as_str());

        Ok((
            format!("ltuid={ltuid}; ltoken={ltoken}; cookie_token={cookie_token}; account_id={account_id}; mi18nLang={lang}"),
            ltuid, ltoken, cookie_token, account_id, lang
        ))
    } 

    #[tokio::main]
    pub async fn claim_daily(&self) -> Result<(), Box<dyn Error>> {
        let mut params = Vec::<(&str, &str)>::new();
        let _region = &self.uid.get_server();
        
        if let Regions::OS = &self.uid.get_region() {
            params.push(("lang", &self.lang));
        } else {
            params.push(("uid", &self.uid.0));
            params.push(("region", _region));
        }

        let url = Url::parse_with_params(self.uid.get_region().get_claim_daily_url(), params).unwrap();

        let res = self.http.post(url)
            .header("Cookie", &self.cookie)
            .send()
            .await?;

        let body: serde_json::Value = serde_json::from_str(&res.text().await?)?;
        
        if body["retcode"] == -5003 {
            // Already claimed
            return Err(Box::from(HoyoError("Nay 't was already claimed today!".to_string())));
        }

        Ok(())
    }

    #[tokio::main]
    pub async fn claim_code(&self, code: &str) -> Result<(), Box<dyn Error>> {
        let url = Url::parse_with_params("https://sg-hk4e-api.hoyolab.com/common/apicdkey/api/webExchangeCdkey", [
            ("lang", "en"),
            ("uid", &self.uid.0),
            ("region", &self.uid.get_server()),
            ("cdkey", code),
            ("game_biz", "hk4e_global")
        ])?;

        let res = self.http.get(url)
            .header("Cookie", &self.cookie)
            .send()
            .await?;
        
        let body: serde_json::Value = serde_json::from_str(&res.text().await?)?;

        if body["retcode"] == -2003 {
            // Invalid
            return Err(Box::from(HoyoError("Invalid code".to_string())));
        } else if body["retcode"] == -2017 {
            // Already claimed
            return Err(Box::from(HoyoError("Already claimed".to_string())));
        } else if body["retcode"] == -2006 {
            // Max usage limit reached
            return Err(Box::from(HoyoError("Max usage limit reached".to_string())));
        } else if body["retcode"] == -2016 {
            // Code redemption in cooldown
            return Err(Box::from(HoyoError("Code redemption in cooldown".to_string())));
        } else if body["retcode"] == -1071 {
            // Not logged in
            return Err(Box::from(HoyoError("You are not logged in".to_string())));
        }

        Ok(())
    }
}