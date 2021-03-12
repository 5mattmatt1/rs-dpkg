use crate::types::ConfFiles;
use crate::rfc822;
use crate::error::{Error, Result};

use std::collections::HashMap;

#[derive(Debug)]
pub struct StatusEntry
{
    // Need a proper dependency struct
    pub depends: Option<Vec<String>>,
    pub description: String,
    pub version: String,
    pub config_version: Option<String>,
    pub homepage: Option<String>,
    pub section: String,
    pub source: Option<String>,
    pub status: String,
    // MB
    pub install_size: usize,
    pub package: String,
    pub provides: Option<String>,
    pub architecture: String,
    pub priority: String,
    pub maintainer: String,
    // Probably best to make it optional rather than zero-length status
    pub conffiles: Option<ConfFiles>
}

#[cfg(debug_assertions)]
fn get_entry<'a>(hmap: &'a HashMap<String, String>, entry: &str) -> Result<&'a String>
{
    // Could kill off the other trait but I already took the time to write it
    // *shrug*
    match hmap.get(entry)
    {
        Some(value) => {
            Ok(value)
        },
        None => {
            println!("{:?}", hmap);
            Err(Error::MissingStatusEntryField(entry.to_string()))
        }
    }
}

// #[!cfg(debug_assertions)]
// fn get_entry<'a>(hmap: &'a HashMap<String, String>, entry: &str) -> Result<&'a String>
// {
//     use crate::util::OkOrMap;
//     hmap.get_or(entry, Error::MissingStatusEntryField(entry.to_string()))
// }

fn get_owned_entry(hmap: &HashMap<String, String>, entry: &str) -> Option<String>
{
    match hmap.get(entry)
    {
        Some(value) => {
            Some(value.clone())
        },
        None => {
            None
        }
    }
}

impl std::convert::TryFrom<HashMap<String, String>> for StatusEntry
{
    type Error = Error;
    fn try_from(from: HashMap<String, String>) -> Result<Self>
    {
        let depends: Option<Vec<String>>;
        let description: String;
        let version: String;
        let config_version: Option<String>;
        let homepage: Option<String>;
        let section: String;
        let source: Option<String>;
        let status: String;
        let install_size: usize;
        let package: String;
        let provides: Option<String>;
        let architecture: String;
        let priority: String;
        let maintainer: String;
        let conffiles: Option<ConfFiles>;     
        
        // Minor cost of copying every field.
        // Have to do this so that we don't invalidate the HashMap
        // by doing moves.

        // Need to deref the None
        config_version = get_owned_entry(&from, "Config-Version");

        // Absolute nonsense required to map a split string to a vector of owned strings
        match from.get("Depends")
        {
            Some(depends_str) => {
                depends = Some(depends_str.split(", ").collect::<Vec<&str>>().iter().map(|str_buf| {
                    str_buf.to_string()
                }).collect());
            },
            None => {
                depends = None;
            }
        }

        description = get_entry(&from, "Description")?.clone();
        version = get_entry(&from, "Version")?.clone();
        homepage = get_owned_entry(&from, "Homepage");
        section = get_entry(&from, "Section")?.clone();
        source = get_owned_entry(&from, "Source");
        status = get_entry(&from, "Status")?.clone();
        install_size = get_entry(&from, "Installed-Size")?.parse::<usize>()?;
        package = get_entry(&from, "Package")?.clone();
        provides = get_owned_entry(&from, "Provides");
        architecture = get_entry(&from, "Architecture")?.clone();
        priority = get_entry(&from, "Priority")?.clone();
        maintainer = get_entry(&from, "Maintainer")?.clone();

        match from.get("Conffiles")
        {
            Some(conffiles_str) => {
                conffiles = Some(crate::util::parse_config_files(conffiles_str)?);
            },
            None => {
                conffiles = None;
            }
        }

        Ok(Self {
            depends : depends,
            description : description,
            version : version,
            config_version : config_version,
            homepage : homepage,
            section : section,
            source : source,
            status : status,
            install_size: install_size,
            package: package,
            provides: provides,
            architecture: architecture,
            priority: priority,
            maintainer: maintainer,
            conffiles: conffiles
        })
    }
}

fn parse_status_file<R>(r: &mut R) -> Result<Vec<StatusEntry>>
where R: std::io::Read
{
    use std::convert::TryFrom;
    let mut status_dicts: Vec<StatusEntry> = Vec::new();
    let mut buffer = String::from("");
    r.read_to_string(&mut buffer)?;

    // I only accept empty lines for seperators.
    // Might be worth looking into something that excepts any amount of whitespace
    // in the line.
    for status_buffer in buffer.split("\n\n")
    {
        let status_dict = rfc822::parse_rfc822_buffer(status_buffer).unwrap();
        // println!("{:?}", status_dict);
        // In case of trailing newline that creates an extra entry.
        if status_dict.len() > 0
        {
            status_dicts.push(StatusEntry::try_from(status_dict)?);
        }
    }

    Ok(status_dicts)
}

pub fn get_status_pkg(name: &str) -> Result<Option<StatusEntry>>
{
    const STATUS_PATH: &str = "/var/lib/dpkg/status";
    let mut file = std::fs::File::open(STATUS_PATH)?;
    let statuses = parse_status_file(&mut file)?;
    
    for status in statuses
    {
        if status.package == name
        {
            return Ok(Some(status));
        }
    }

    Ok(None)
}

pub fn display_status_pkg(name: &str) -> Result<()>
{
    let pkg_status_res = get_status_pkg(name)?;
    // println!("Status => {:?}", status);
    match pkg_status_res
    {
        Some(pkg_status) => {
            println!("{:?}", pkg_status);
        },
        None => {
            println!("No package with name: {}", name);
        }
    }

    Ok(())
}