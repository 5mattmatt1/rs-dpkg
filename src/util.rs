use crate::error::{Error, Result};
use crate::types::ConfFiles;

// I guess Q: ?Sized only works if the trait has it as well?
pub trait OkOrMap<K, V, Q>
where Q: ?Sized
{
    fn get_or<E>(&self, k: &Q, error: E) -> std::result::Result<&V, E>
        where K: std::borrow::Borrow<Q>;
    fn get_mut_or<E>(&mut self, k: &Q, error: E) -> std::result::Result<&mut V, E>
        where K: std::borrow::Borrow<Q>;
}

impl<K, V, Q> OkOrMap<K, V, Q> for std::collections::HashMap<K, V>
where K: std::hash::Hash + Eq,
      Q: std::hash::Hash + Eq + ?Sized
{
    fn get_or<E>(&self, k: &Q, e: E) -> std::result::Result<&V, E>
    where K: std::borrow::Borrow<Q>
    {
        self.get(k).ok_or(e)
    }

    fn get_mut_or<E>(&mut self, k: &Q, e: E) -> std::result::Result<&mut V, E>
        where K: std::borrow::Borrow<Q>
    {
        self.get_mut(k).ok_or(e)
    }
}

pub fn get_file_ext(fpath: &str) -> Option<&str>
{
    let mid_opt = fpath.rfind(".");
    match mid_opt
    {
        Some(mid) => {
            let (_base, ext) = fpath.split_at(mid);
            return Some(ext);
        },
        None => {
            return None;
        }
    }
}

pub fn lsdir<R>(a: &mut tar::Archive<R>) -> Result<()>
where R: std::io::Read
{
    for entry_res in a.entries()?
    {
        let entry = entry_res?;
        let fpath = entry.path()?.into_owned();
        
        println!("---> {}", fpath.to_str().unwrap());
    }

    Ok(())
}

pub fn hex_string_to_md5(hex: &str) -> Result<md5::Digest>
{
    if hex.len() != 32
    {
        return Err(Error::MD5Error);
    }

    let mut buffer: [u8; 16] = [0; 16];
    for i in (0..32).step_by(2)
    {
        buffer[i/2] = u8::from_str_radix(&hex[i..i+1], 16)?;
    }

    Ok(md5::Digest {
        0: buffer
    })
}

pub fn parse_config_files(string: &str) -> Result<ConfFiles>
{
    // Possible to be obsolete. Neat.
    // Fuck.
    let mut conffiles = ConfFiles::new();
    let split: Vec<&str> = string.trim().split(" ").collect();

    // Needs to be divisible by two to be able to properly do step_by(2)
    if split.len() % 2 != 0
    {
        println!("{}", string);
    }

    for i in (0..split.len()).step_by(2)
    {
        conffiles.insert(split[i].to_string(), hex_string_to_md5(split[i+1])?);
    }

    Ok(conffiles)
}