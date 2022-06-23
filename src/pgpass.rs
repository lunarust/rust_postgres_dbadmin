extern crate dirs;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;

//https://github.com/notryanb/psql_connect/blob/master/src/main.rs

#[derive(Debug, PartialEq)]
pub struct PgConfigList {
    pub configs: Vec<PgConfig>,
}

impl PgConfigList {
    pub fn new() -> PgConfigList {
        PgConfigList {
            configs: Vec::new(),
        }
    }

    pub fn add(&mut self, config: PgConfig) {
        self.configs.push(config);
    }

    pub fn list_aliases(&self) -> Vec<&String> {
        self.configs
            .iter()
            .map(|cfg| &cfg.alias)
            .collect()
    }

    pub fn select_config(&self, alias: &str) -> &PgConfig  {
        let config: Vec<_> = self.configs
            .iter()
            .filter(|cfg| cfg.alias == alias)
            .collect();
            config[0]
        /*
        let config: Vec<_> = self.configs
            .iter()
            .filter(|cfg| cfg.alias == alias)
            .collect();

        match config.len() {
            1 => Ok(config[0]),
            _ => panic!(),
        }
        */
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct PgConfig {
    pub alias: String,
    pub hostname: String,
    pub port: u16,
    pub dbname: String,
    pub username: String,
    pub password: String,
}
pub fn parse_pg_pass() -> Result<PgConfigList, Error> {

    let home = dirs::home_dir();
    let file_path = Path::new(".pgpass");
    let pg_pass_path = home.unwrap().join(file_path);
//    println!("testing that {}", pg_pass_path.display());
    //let tst = home.push("myfile.tar.gz");
//    Path::new("..").join("bin").join("openvpn.exe");
    let pg_pass_file = File::open(pg_pass_path)?;
    let reader = BufReader::new(pg_pass_file);
    let mut config_list = PgConfigList::new();

    for line in reader.lines() {
        let mut params = line.as_ref().unwrap().split(':');
        let hostname = params.next().unwrap().to_string();
        let port = params.next().unwrap().parse::<u16>().unwrap();
        let dbname = params.next().unwrap().into();
        let username = params.next().unwrap().into();
        let password = params.next().unwrap().into();
        let alias = match params.next() {
            Some(alias) => alias.into(),
            None => hostname.clone()
        };
//        println!("reading line: {}", alias);
        let config = PgConfig {
            alias: alias,
            hostname: hostname,
            port: port,
            username: username,
            password: password,
            dbname: dbname,
        };

        config_list.add(config);
    }

    Ok(config_list)

}
