#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u32,
    pub storage_path: String,
    pub sqlite_path: String,
    pub workers: u32
}

impl ServerConfig {
    pub fn load_or_default() -> ServerConfig {
        ServerConfig {
            host: std::env::var("SERVER.HOST").unwrap_or("127.0.0.1".into()),
            port: std::env::var("SERVER.PORT").unwrap_or("8080".into()).parse().unwrap(),
            storage_path: std::env::var("STORAGE.PATH").unwrap_or("/data/tetris".into()),
            sqlite_path: std::env::var("DATABASE.PATH").unwrap_or("/data/tetris".into()),
            workers: std::env::var("SERVER.WORKERS").unwrap_or("4".into()).parse().unwrap(),
        }
    }
}