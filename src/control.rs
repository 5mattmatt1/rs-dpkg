struct Dependency
{
    name: String,
    min_version: String,
}

struct ControlInfo
{
    package: String,
    version: String,
    architecture: String,
    // Optional, use an empty Vector to represent this?
    conflicts: Vec<Dependency>,
    dependencies: Vec<Dependency>,
    // Could maybe be split into name and email?
    maintainer: String,
    original_maintainer: Option<String>,
    install_size: usize,
    provides: String,
    priority: String,
    home_page: String,
    description: String
}

struct ControlArchiveInfo
{
    control: ControlInfo,
    md5sums: HashMap<md5::Digest, String>,
    conffiles: Vec<Path>,
    preinst: Option<Path>,
    postinst: Option<Path>,
    prerm: Option<Path>,
    postrm: Option<Path>,
    config: Option<Path>,
    shlibs: Vec<String>
}