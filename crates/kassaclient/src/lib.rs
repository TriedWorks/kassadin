#![allow(unused)]

mod lcu;
mod socket;

use anyhow::Result;
use kassatypes::consts::Region;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, ClientBuilder, Response};
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::str::FromStr;
use sysinfo::{ProcessExt, SystemExt};

// TODO: make client an option
// TODO: add logging
#[derive(Debug, Clone)]
pub struct LCU {
    pub info: Option<ClientInfo>,
    pub requester: Client,
}

impl LCU {
    pub fn new() -> Self {
        Self {
            info: None,
            requester: ClientBuilder::new()
                .danger_accept_invalid_certs(true)
                .build()
                .unwrap(),
        }
    }

    pub async fn simple_request<T: DeserializeOwned + Debug>(&self, url: &str) -> Result<T> {
        let url = format!("https://127.0.0.1:{}{}", self.info().port.as_str(), url);
        let response = self.requester.get(url).send().await?;
        let parsed = response.json::<T>().await?;
        Ok(parsed)
    }

    pub async fn simple_request_raw(&self, url: &str) -> Response {
        let url = format!("https://127.0.0.1:{}{}", self.info().port.as_str(), url);
        let request = self.requester.get(url).send().await.unwrap();
        request
    }

    pub fn base64_auth(&self) -> String {
        let auth = format!("riot:{}", &self.info().token);
        base64::encode(auth)
    }

    pub fn find(&mut self) -> Result<&ClientInfo> {
        let info = ClientInfo::new()?;
        self.info = Some(info);
        let base64_auth = self.base64_auth();

        let mut headers = HeaderMap::new();
        headers.append("Authorization", format!("Basic {}", base64_auth).parse()?);

        self.requester = ClientBuilder::new()
            .danger_accept_invalid_certs(true)
            .default_headers(headers)
            .build()
            .unwrap();

        Ok(self.info())
    }

    pub fn info(&self) -> &ClientInfo {
        &self.info.as_ref().unwrap()
    }

    pub fn socket_url_auth(&self) -> String {
        let info = self.info();
        let base64_auth = self.base64_auth();
        format!("wss://riot:{}@127.0.0.1:{}", info.token, info.port)
    }

    pub fn socket_url(&self) -> String {
        let info = self.info();
        format!("wss://127.0.0.1:{}", info.port)
    }

    pub fn socket(&self) -> WebSocket {
        WebSocket { lcu: &self }
    }

    pub fn champ_select(&self) -> ChampSelect {
        ChampSelect { lcu: &self }
    }

    pub fn summoner(&self) -> Summoner {
        Summoner { lcu: &self }
    }

    pub fn chat(&self) -> Chat {
        Chat { lcu: &self }
    }

    pub fn lobby(&self) -> Lobby {
        Lobby { lcu: &self }
    }
}

// TODO: add the other attributes
#[derive(Debug, Clone)]
pub struct ClientInfo {
    pub pid: usize,
    pub path: String,
    pub token: String,
    pub port: String,
    pub region: Region,
    pub locale: String,
}

impl ClientInfo {
    pub fn new() -> Result<Self> {
        let mut system = sysinfo::System::new_all();
        system.refresh_all();

        let mut client_pid = None;
        let mut client_process = None;
        for (pid, process) in system.get_processes() {
            if ["LeagueClientUx.exe", "LeagueClientUx"].contains(&process.name()) {
                client_pid = Some(pid);
                client_process = Some(process)
            }
        }

        let pid = client_pid.unwrap().clone();
        let process = client_process.unwrap();
        let env = process.cmd();

        let path = env[0].to_string();
        let mut token = None;
        let mut port = None;
        let mut region = None;
        let mut locale = None;

        for var in env {
            if var.starts_with("--remoting-auth-token=") {
                token = Some(var.split("--remoting-auth-token=").collect::<Vec<&str>>()[1])
            } else if var.starts_with("--app-port=") {
                port = Some(var.split("--app-port=").collect::<Vec<&str>>()[1])
            } else if var.starts_with("--region=") {
                let string_region = var.split("--region=").collect::<Vec<&str>>()[1];
                region = Some(Region::from_str(string_region)?)
            } else if var.starts_with("--locale=") {
                locale = Some(var.split("--locale=").collect::<Vec<&str>>()[1])
            }
        }

        Ok(Self {
            pid,
            path,
            token: token.unwrap().to_string(),
            port: port.unwrap().to_string(),
            region: region.unwrap(),
            locale: locale.unwrap().to_string(),
        })
    }
}

pub struct WebSocket<'a> {
    lcu: &'a LCU,
}

pub struct ChampSelect<'a> {
    lcu: &'a LCU,
}

pub struct Summoner<'a> {
    lcu: &'a LCU,
}

pub struct Chat<'a> {
    lcu: &'a LCU,
}

pub struct EndOfGame<'a> {
    lcu: &'a LCU,
}

pub struct Lobby<'a> {
    lcu: &'a LCU,
}

#[cfg(test)]
mod tests {
    use crate::LCU;

    #[test]
    fn create_lcu() {
        let mut client = LCU::new();
        let info = client.find().unwrap();
        println!("{:?}", info)
    }

    #[test]
    fn current_summoner() {
        let mut client = LCU::new();
        let info = client.find().unwrap();
        let current_summoner = tokio_test::block_on(client.summoner().current());

        println!("{:?}", current_summoner);
    }

    #[test]
    fn websocket() {
        let mut client = LCU::new();
        client.find().unwrap();
        tokio_test::block_on(client.socket().run());
    }
}
