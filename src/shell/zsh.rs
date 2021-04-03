use crate::shell::Shell;
use std::path::Path;

#[derive(Debug)]
pub struct Zsh;

impl Shell for Zsh {
    fn path(&self, path: &Path) -> String {
        format!("export PATH={:?}:$PATH", path.to_str().unwrap())
    }

    fn set_env_var(&self, name: &str, value: &str) -> String {
        format!("export {}={:?}", name, value)
    }

    fn use_on_cd(&self, _config: &crate::config::FarmConfig) -> String {
        indoc::indoc!(
            r#"
                autoload -U add-zsh-hook
                _farm_autoload_hook () {
                    if [[ -f .ruby-version ]]; then
                        farm local
                    fi
                }

                add-zsh-hook chpwd _farm_autoload_hook \
                    && _farm_autoload_hook
            "#
        )
        .into()
    }
}
