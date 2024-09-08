//! This module is where we store constants, like filepaths or the the version of the current
//! Minecraft version that the server is implementing.
// TODO: Maybe reimplement this with a real querying API, like a HashMap like object.

/// Module where we store information relevant to the Minecraft server.
pub mod minecraft {
    pub const VERSION: &'static str = "1.21.1"; //upgrade to 1.21.1 cuz wiki.vg is up to date
    pub const PROTOCOL_VERSION: usize = 767;
}

/// Server logging messages.
pub mod messages {

    use colored::*;
    use once_cell::sync::Lazy;

    use super::minecraft::VERSION;

    pub static SERVER_STARTING: Lazy<String> = Lazy::new(|| {
        format!("Starting minecraft server version {}", VERSION)
            .bold()
            .to_string()
    });

    pub static SERVER_STARTED: Lazy<String> =
        Lazy::new(|| "[ SERVER STARTED ]".bright_green().bold().to_string());

    pub static SERVER_SHUTDOWN: Lazy<String> =
        Lazy::new(|| "[ SERVER SHUT DOWN ]".bright_red().bold().to_string());

    pub static GREET: Lazy<String> =
        Lazy::new(|| "Hello, world from Copper!".green().bold().to_string());

    /// Used when exiting the server with an exit code.
    pub fn server_shutdown_code(code: i32) -> String {
        return format!("[ server shutdown with code: {code}]")
            .to_uppercase()
            .bright_red()
            .bold()
            .to_string();
    }
}

/// Module used to store file paths relative to the server binary.
pub mod filepaths {
    /// server.properties file, used to store server settings.
    pub const PROPERTIES: &'static str = "server.properties";
    pub const EULA: &'static str = "eula.txt";
    pub const OPERATORS: &'static str = "ops.json";
    pub const WHITELIST: &'static str = "whitelist.json";
    pub const BANNED_IP: &'static str = "banned-ips.json";
    pub const BANNED_PLAYERS: &'static str = "banned-players.json";
    pub const USERCACHE: &'static str = "usercache.json";
    pub const SESSION: &'static str = "session.lock";
}
pub mod folderpath {
    pub const WORLDS_DIRECTORY: &'static str = "world/";
    pub const THE_END: &'static str = "world/DIM1/";
    pub const NETHER: &'static str = "world/DIM-1/";
    pub const OVERWORLD: &'static str = "world/region/";
    pub const LOGS: &'static str = "logs/";
}

pub mod file_content {
    use crate::time;

    /// Returns the default content of the 'eula.txt' file.
    pub fn eula() -> String {
        let mut content = String::new();

        content += "# By changing the setting below to 'true' you are indicating your agreement to our EULA (https://aka.ms/MinecraftEULA).\n";
        content += &format!("# {}", time::get_formatted_time());
        content += "\neula=false";
        content
    }

    /// Returns the default content of the 'server.properties' file.
    pub fn server_properties() -> String {
        const SERVER_PROPERTIES_INNER: &str = r#"accepts-transfers=false
allow-flight=false
allow-nether=true
broadcast-console-to-ops=true
broadcast-rcon-to-ops=true
bug-report-link=
difficulty=easy
enable-command-block=false
enable-jmx-monitoring=false
enable-query=false
enable-rcon=false
enable-status=true
enforce-secure-profile=true
enforce-whitelist=false
entity-broadcast-range-percentage=100
force-gamemode=false
function-permission-level=2
gamemode=survival
generate-structures=true
generator-settings={}
hardcore=false
hide-online-players=false
initial-disabled-packs=
initial-enabled-packs=vanilla
level-name=world
level-seed=
level-type=minecraft\:normal
log-ips=true
max-chained-neighbor-updates=1000000
max-players=20
max-tick-time=60000
max-world-size=29999984
motd=A Minecraft Server
network-compression-threshold=256
online-mode=true
op-permission-level=4
player-idle-timeout=0
prevent-proxy-connections=false
pvp=true
query.port=25565
rate-limit=0
rcon.password=
rcon.port=25575
region-file-compression=deflate
require-resource-pack=false
resource-pack=
resource-pack-id=
resource-pack-prompt=
resource-pack-sha1=
server-ip=
server-port=25565
simulation-distance=10
spawn-animals=true
spawn-monsters=true
spawn-npcs=true
spawn-protection=16
sync-chunk-writes=true
text-filtering-config=
use-native-transport=true
view-distance=10
white-list=false"#;

        format!(
            "# Minecraft server properties\n{}\n{}",
            format!("# {}", time::get_formatted_time()),
            SERVER_PROPERTIES_INNER,
        )
    }
}
<<<<<<< HEAD
use serde::Serialize;

#[derive(Serialize)]
pub struct Versionmc {
    name: String,
    protocol: usize,
}

#[derive(Serialize)]
pub struct PlayerSample {
    name: String,
    id: String,
}

#[derive(Serialize)]
pub struct Players {
    max: u32,
    online: u32,
    sample: Vec<PlayerSample>,
}

#[derive(Serialize)]
pub struct Description {
    text: String,
}

#[derive(Serialize)]
pub struct PingResponse {
    version: Version,
    players: Players,
    description: Description,
    favicon: Option<String>,
    enforces_secure_chat: bool,
}

pub mod ping_slp {
    use crate::{config, player::get_uuid};

    use super::{
        minecraft::{PROTOCOL_VERSION, VERSION},
        Description, PlayerSample, Players, Versionmc,
    };

    /// Return the json which will be send as a response of the ping to the client.

    pub async fn json_response() -> () {
        // For the version like 1.21.1,767.
        let version = Versionmc {
            name: VERSION.to_string(),
            protocol: PROTOCOL_VERSION,
        };
        // For the players.
        let sample = vec![PlayerSample {
            name: "spectre".to_string(),
            id: match get_uuid("todo").await {
                Ok(body) => body,
                Err(_) => String::from("not found"),
            },
        }];
        let player = Players {
            max: config::Settings::new().max_players,
            online: 5, //todo
            sample,
        };
        let descritpion = Description {
            text: config::Settings::new().motd,
        };
        // Description:
        let description_text = config::Settings::new().motd;

    }
}
