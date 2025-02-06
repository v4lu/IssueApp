use std::env;
use std::fs;

pub fn load_dotenv() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(".env")?;

    for line in contents.lines() {
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.splitn(2, '=').collect();
        if parts.len() == 2 {
            let key = parts[0].trim();
            let value = parts[1].trim();

            env::set_var(key, value);
        }
    }

    Ok(())
}
