use colorize::AnsiColor;
use figlet_rs::FIGfont;

use crate::Container;

pub struct Printer {
    container: Container,
}

impl Printer {
    pub fn new(container: Container) -> Self {
        Self { container }
    }

    fn banner(&self) -> String {
        let standard_font = FIGfont::from_content(include_str!("doom.flf")).unwrap();

        let figure = standard_font.convert(&self.container.config.image);
        match figure {
            Some(s) => s.to_string().green(),
            None => self.container.config.image.to_string().green(),
        }
    }

    fn header(input: &str) -> String {
        format!("{:10}", input).blue()
    }

    fn status(&self) -> String {
        let status = match self.container.state.status.as_str() {
            "created" => "Created".blue(),
            "restarting" => "Restarting".red(),
            "paused" => "Paused".blue(),
            "running" => "Running".green(),
            "dead" => "Dead".red(),
            "exited" => "Exited".red(),
            value => value.to_string(),
        };

        format!("{}{}\n", Self::header("Status"), status)
    }

    pub fn uptime(&self) -> String {
        let mut value = self.container.uptime().num_seconds();
        let mut unit = "seconds";
        if value > 60 {
            value /= 60;
            unit = "minutes";
        }
        format!("{}{} {}\n", Self::header("Uptime"), value, unit)
    }

    pub fn name(&self) -> String {
        format!("{}{}\n", Self::header("Name"), self.container.name)
    }

    pub fn host(&self) -> String {
        format!(
            "{}{}\n",
            Self::header("hostname"),
            self.container.config.hostname
        )
    }

    pub fn ports(&self) -> String {
        format!(
            "{}{}\n",
            Self::header("Ports"),
            self.container
                .network_settings
                .ports
                .keys()
                .cloned()
                .collect::<Vec<String>>()
                .join(" ")
        )
    }

    pub fn print(&self) -> String {
        let mut result = self.banner();
        result.push_str(&self.status());
        result.push_str(&self.uptime());
        result.push_str(&self.name());
        result.push_str(&self.host());
        result.push_str(&self.ports());
        result
    }
}
