use crate::identity::Identity;
use anyhow::Result;
use git2::Config;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum ConfigKey {
    Name,
    Email,
    SigningKey,
    SshKey,
}

impl std::str::FromStr for ConfigKey {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<ConfigKey, Self::Err> {
        match s {
            "name" => Ok(ConfigKey::Name),
            "email" => Ok(ConfigKey::Email),
            "signingkey" => Ok(ConfigKey::SigningKey),
            "sshkey" => Ok(ConfigKey::SshKey),
            _ => unreachable!(),
        }
    }
}

pub(crate) struct Manager {
    config: Config,
}

impl Manager {
    pub(crate) fn new() -> Result<Self> {
        let global_config_path = Config::find_global()?;
        let global_config = Config::open(&global_config_path)?;

        Ok(Self {
            config: global_config,
        })
    }

    pub(crate) fn add(&mut self, identity: &Identity) {
        self.config
            .set_str(&format!("user.{id}.name", id = identity.id), &identity.name)
            .unwrap();
        self.config
            .set_str(
                &format!("user.{id}.email", id = identity.id),
                &identity.email,
            )
            .unwrap();
        identity.signing_key.as_ref().map(|key| {
            self.config
                .set_str(&format!("user.{id}.signingkey", id = identity.id), key)
                .unwrap()
        });
        identity.ssh_key.as_ref().map(|key| {
            let s = key.as_path().to_str().unwrap();
            self.config
                .set_str(&format!("user.{id}.sshkey", id = identity.id), s)
                .unwrap()
        });
    }

    pub(crate) fn list_identities(&self) {
        let mut set = HashSet::new();

        for entry in &self.config.entries(Some("user.*.*.*")).unwrap() {
            let entry = entry.unwrap();
            let parts = entry.name().unwrap().split(".").collect::<Vec<_>>();
            match parts[parts.len() - 1] {
                "name" | "email" | "signingkey" | "sshkey" => {
                    let tag = parts[1..(parts.len() - 1)].join(".");
                    set.insert(tag);
                }
                _ => continue,
            }
        }

        for profile_name in set {
            println!("{}", profile_name);
        }
    }
}
