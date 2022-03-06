use redis::Commands;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct RedisConfig {
    connection_string: String,
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            connection_string: "redis://127.0.0.1/".to_string(),
        }
    }
}

struct RedisManager<'a> {
    config: &'a RedisConfig,
    conn: Option<redis::Connection>,
}

impl<'a> RedisManager<'a> {
    fn new(config: &'a RedisConfig) -> Self {
        Self { config, conn: None }
    }

    fn set_string(&mut self, key: &str, val: &str) {
        if let None = self.conn {
            let client = redis::Client::open(self.config.connection_string.as_str()).unwrap();
            self.conn = Some(client.get_connection().unwrap());
        }
        self.conn
            .as_mut()
            .unwrap()
            .set::<&str, &str, ()>(key, val)
            .unwrap();
    }
}

struct SyncManager<'a> {
    redis: RedisManager<'a>,
    user_email: String,
}

impl<'a> SyncManager<'a> {
    fn new(redis: RedisManager<'a>, user_email: String) -> Self {
        Self { redis, user_email }
    }

    fn upload_file(&mut self, file_local_path: &str, upload_path: &str) {
        /*
         *  add validation on `upload_path` if that is correct path by convention.
         *  when uploading, the redis key will be: `{useremail}/upload_path`
         *
         * */

        let content = fs::read_to_string(file_local_path).unwrap();
        let redis_key = format!("{}/{}", self.user_email, upload_path);
        self.redis.set_string(&redis_key, &content);
    }
}

fn main() {
    let config = RedisConfig::default();
    let redis_manager = RedisManager::new(&config);
    let upload_path = "docker/docker-compose.yml";
    let email = "kr.mohit6794@gmail.com".to_string();

    let mut sync_manager = SyncManager::new(redis_manager, email);
    sync_manager.upload_file("docker-compose.yml", upload_path);

    // let content = fs::read_to_string(path).unwrap();
    // manager.set_string(path, &content);
}
