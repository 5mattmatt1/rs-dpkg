extern crate ar; // .deb
extern crate flate2; // .gz
extern crate lzma; // .xz
extern crate tar; // .tar
extern crate md5; // md5 digests for validation of files.
extern crate termion; // Terminal color

use flate2::read::GzDecoder;

mod error;
mod rfc822;
mod status;
mod types;
mod util;
mod warning;

use crate::error::Result;

// Debian RFC822 Control data format


// fn read_deb()
// {

// }

// RFC822
fn parse_ctrl_file<R>(r: &mut R) -> Result<()>
where R: std::io::Read
{
    let mut buffer = String::from("");
    r.read_to_string(&mut buffer)?;
    let ctrl_dict = crate::rfc822::parse_rfc822_buffer(&mut buffer);
    println!("{:?}", ctrl_dict);
    Ok(())
}

fn handle_ctrl_tar<R>(a: &mut tar::Archive<R>) -> Result<()>
where R: std::io::Read
{
    for entry_res in a.entries()?
    {
        let mut entry = entry_res?;
        let fpath = entry.path()?.into_owned();
        
        if fpath.to_str().unwrap() == "./control"
        {
            parse_ctrl_file(&mut entry)?;
        }

        println!("---> {}", fpath.to_str().unwrap());
    }

    Ok(())
}

fn handle_data_tar<R>(a: &mut tar::Archive<R>) -> Result<()>
where R: std::io::Read
{
    crate::util::lsdir(a)
}

fn handle_deb<R>(a: &mut ar::Archive<R>) -> Result<()>
where R: std::io::Read
{
    while let Some(entry_result) = a.next_entry() 
    {
        let entry = entry_result?;
        let ident = std::str::from_utf8(entry.header().identifier())?;
        println!("{}", ident);
        
        if ident.starts_with("control.tar")
        {
            let file_ext_opt = crate::util::get_file_ext(ident);
            match file_ext_opt
            {
                Some(".xz") => {
                    let d = lzma::reader::LzmaReader::new_decompressor(entry)?;
                    let mut a = tar::Archive::new(d);
                    handle_ctrl_tar(&mut a)?;
                },
                Some(".gz") => {
                    let d = GzDecoder::new(entry);
                    let mut a = tar::Archive::new(d);
                    handle_ctrl_tar(&mut a)?;
                },
                Some(_) => {
                    continue;
                },
                None => {
                    continue;
                }
            }
        }
        else if ident.starts_with("data.tar")
        {
            let file_ext_opt = crate::util::get_file_ext(ident);
            match file_ext_opt
            {
                Some(".xz") => {
                    let d = lzma::reader::LzmaReader::new_decompressor(entry)?;
                    let mut a = tar::Archive::new(d);
                    handle_data_tar(&mut a)?;
                },
                Some(".gz") => {
                    let d = GzDecoder::new(entry);
                    let mut a = tar::Archive::new(d);
                    handle_data_tar(&mut a)?;
                },
                Some(_) => {
                    continue;
                },
                None => {
                    continue;
                }
            }
            println!("Found data archive");
            // let d = lzma::read(entry).unwrap();
        }

        // Create a new file with the same name as the archive entry:
        // let mut file = File::create(
        //    str::from_utf8(entry.header().identifier()).unwrap(),
        // ).unwrap();
        // The Entry object also acts as an io::Read, so we can easily copy the
        // contents of the archive entry into the file:
        // io::copy(&mut entry, &mut file).unwrap();
    }
    Ok(())
}

fn parse_path_list<R>(r: &mut R) -> std::io::Result<Vec<std::path::PathBuf>>
where R: std::io::Read
{
    let mut res: Vec<std::path::PathBuf> = Vec::new();

    let mut buffer = String::from(""); 
    r.read_to_string(&mut buffer)?;

    for (lineno, line) in buffer.lines().enumerate()
    {
        let pth = std::path::Path::new(line);
        
        // We don't want to clear out directories
        // Especially
        if pth.is_dir()
        {
            match pth.to_str()
            {
                Some(pth_str) => {
                    println!("Found directory: {}. Lineno: {}", pth_str, lineno);
                },
                None => {
                    println!("Non-path found. Lineno: {}", lineno);
                }
            }
            continue;
        }

        res.push(pth.to_path_buf());
    }

    Ok(res)
}

fn remove_pkg(name: &str) -> Result<()>
{
    use crate::warning::Warning;
    use crate::status::get_status_pkg;
    let status_pkg_opt = get_status_pkg(name)?;
    if status_pkg_opt.is_none()
    {
        // May turn this into 
        let warning_msg = format!("ignoring request to remove {} which isn't installed", name);
        let warning = Warning::new(&warning_msg);
        println!("{}", warning);
        return Ok(());
    }
    
    let status_pkg = status_pkg_opt.unwrap();
    if status_pkg.status == "deinstall ok config-files"
    {
        let warning_msg = format!("ignoring request to remove {}, only the config 
 files of which are on the system; use --purge to remove them too.", name);
        let warning = Warning::new(&warning_msg);
        println!("{}", warning);
        return Ok(());
    }
    
    // Probably a way to break some of this out.
    // postrm and prerm look almost exactly the same.
    const CMD_BASE: &'static str = "/var/lib/dpkg/info/";
    let cmd_pth = std::path::Path::new(CMD_BASE);
    
    let mut prerm_cmd_str = name.to_string();
    prerm_cmd_str.push_str(".prerm");
    let prerm_cmd_pth = cmd_pth.join(prerm_cmd_str);

    if prerm_cmd_pth.exists()
    {
        let mut prerm_cmd = std::process::Command::new(prerm_cmd_pth);
        prerm_cmd.arg("remove");
        prerm_cmd.output()?;
    } else
    {
        println!("No prerm script found.");
    }

    let mut list_str = name.to_string();
    list_str.push_str(".list");
    let list_pth = cmd_pth.join(list_str);
    if list_pth.exists()
    {
        let mut file = std::fs::File::open(list_pth)?;
        let list_ents = parse_path_list(&mut file)?;
        for list_ent in list_ents
        {
            match list_ent.to_str()
            {
                Some(list_ent_str) => {
                    println!("Deleting path: {}", list_ent_str);
                },
                None => {
                    println!("Got non-path.");
                }
            }
            std::fs::remove_file(list_ent)?;
        }
    } else
    {
        println!("No lists file found...");
    }
    
    // Would want to delete conffiles too in a purge...
    // It would also completely delete the entry from status in a purge.
    // Probably want to move purge and remove into their own remove.rs...
    // Which would require moving out parse_rfc822 into it's own module...

    let mut postrm_cmd_str = name.to_string();
    postrm_cmd_str.push_str(".postrm");
    let postrm_cmd_pth = cmd_pth.join(postrm_cmd_str);
    
    if postrm_cmd_pth.exists()
    {
        let mut postrm_cmd = std::process::Command::new(postrm_cmd_pth);
        postrm_cmd.arg("remove");
        postrm_cmd.output()?;
    } else
    {
        println!("No postrm script found.");
    }

    Ok(())
}

fn main() 
{
    // Fallen angel by 3 days grace
    let args: Vec<String> = std::env::args().collect();

    let pkg_name: String;
    if args.len() == 2
    {
        pkg_name = args[1].clone();
        println!("{}", pkg_name);
    } else
    {
        println!("Usage: rs-dpkg <package-name>");
        return;
    }

    use crate::status::display_status_pkg;
    match display_status_pkg(&pkg_name)
    {
        Ok(_) => {
        },
        Err(err) => {
            println!("{}", err);
        }
    }
    // match remove_pkg("tcsh")
    // {
    //     Ok(_) => {

    //     },
    //     Err(err) => {
    //         println!("{}", err);
    //     }
    // }
    // remove_pkg("tcsh").unwrap();
    // let mut archive = ar::Archive::new(File::open("tcsh_6.20.00-7+b1_amd64.deb").unwrap());
    // handle_deb(&mut archive).unwrap();
    // parse_status_file(&mut File::open("/var/lib/dpkg/status").unwrap()).unwrap();
    // Iterate over all entries in the archive:
    // println!("Entries:");
}
