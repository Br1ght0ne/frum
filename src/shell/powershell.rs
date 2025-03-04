use crate::shell::Shell;
use indoc::indoc;
use std::path::Path;

#[derive(Debug)]
pub struct PowerShell;

impl Shell for PowerShell {
    fn path(&self, path: &Path) -> String {
        let current_path = std::env::var_os("PATH").expect("Can't read PATH env var");
        let mut split_paths: Vec<_> = std::env::split_paths(&current_path).collect();
        split_paths.insert(0, path.to_path_buf());
        let new_path = std::env::join_paths(split_paths).expect("Can't join paths");
        self.set_env_var("PATH", new_path.to_str().expect("Can't read PATH"))
    }

    fn set_env_var(&self, name: &str, value: &str) -> String {
        format!(r#"$env:{} = "{}""#, name, value)
    }

    fn use_on_cd(&self, _config: &crate::config::FrumConfig) -> String {
        indoc!(
            r#"
            function Set-LocationWithFrum {
                param($path)
                Set-Location $path
                If (Test-Path .ruby-version) { & frum local --quiet }
            }
            Set-Alias cd_with_frum Set-LocationWithFrum -Force
            Remove-Item alias:\cd
            New-Alias cd Set-LocationWithFrum
        "#
        )
        .into()
    }

    fn into_clap_shell(&self) -> clap::Shell {
        clap::Shell::PowerShell
    }
}
