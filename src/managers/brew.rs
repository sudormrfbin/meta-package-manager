use crate::{Commands, Error, Operation, PackError, Package, PackageManager, Repo, SubCommand};

pub struct HomeBrew;

impl HomeBrew {
    pub fn new() -> Self {
        HomeBrew
    }

    fn parse_package<'a, 'b>(line: &'a str) -> Package<'b> {
        if let Some((name, version)) = line.split_once('@') {
            return Package::from(name.trim().to_owned()).with_version(version.trim().to_owned());
        }
        Package::from(line.trim().to_owned())
    }
}

impl PackageManager for HomeBrew {
    fn name(&self) -> &'static str {
        "HomeBrew"
    }

    fn search(&self, pack: &str) -> Vec<Package> {
        let out = self.execute_cmds(&[self.cmd(), pack]);
        // TODO evaluate whether this error should be handled
        let outstr = std::str::from_utf8(&out.stdout).unwrap();
        outstr.lines().map(|s| Self::parse_package(s)).collect()
    }

    fn execute_op(&self, pack: &Package, op: Operation) -> PackError<()> {
        let cmd = match op {
            Operation::Install => self.install_cmd(),
            Operation::Uninstall => self.uninstall_cmd(),
            Operation::Update => self.update_cmd(),
        };
        self.execute_cmds_status(&[cmd, &pack.fmt_with_delimiter('@')])
            .success()
            .then_some(())
            .ok_or(Error)
    }

    fn list_installed(&self) -> Vec<Package> {
        let out = self.execute_cmds(&[self.list_cmd()]);
        let outstr = std::str::from_utf8(&out.stdout).unwrap();
        outstr.lines().map(|s| Self::parse_package(s)).collect()
    }

    fn add_repo(&self, repo: Repo) -> PackError<()> {
        self.execute_cmds_status(&[self.repo_cmd(), repo.as_str()])
            .success()
            .then_some(())
            .ok_or(Error)
    }
}

impl Commands for HomeBrew {
    fn cmd(&self) -> &'static str {
        "brew"
    }
    fn sub_cmds(&self, sub_cmd: SubCommand) -> &'static str {
        match sub_cmd {
            SubCommand::Install => "install",
            SubCommand::Uninstall => "uninstall",
            SubCommand::Update | SubCommand::UpdateAll => "upgrade",
            SubCommand::List => "list",
            SubCommand::Sync => "update",
            SubCommand::AddRepo => "tap",
        }
    }
}
