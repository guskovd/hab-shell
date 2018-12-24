extern crate dirs;

use common;
use std::fs;

static HAB_SHELL_PUB_NAME: &str = "hab-shell-20180516110337.pub";
static HAB_SHELL_PUB_CONTENT: &str = r#"SIG-PUB-1
hab-shell-20180516110337

a3j//px8SQp1Mv8FbOMShfa5toaUwOM3H0RQb8dIEFg="#;

static HAB_SHELL_SIG_KEY_NAME: &str = "hab-shell-20180516112716.sig.key";
static HAB_SHELL_SIG_KEY_CONTENT: &str = r#"SIG-SEC-1
hab-shell-20180516112716

IJnGRn7q1xWegkTkfroOKeBmBYDPFM29ooEQRVD+/nLuu1mbzVLGaAxCoya1esMRcTXmn2V62iqTrrCMvi6wsw=="#;

fn init_hab_shell_dir() {
    let home = common::get_home();
    let cache_path = format!("{}/.hab-shell-test/cache/keys", home.display());
    fs::create_dir_all(&cache_path).unwrap();
    init_keys(&cache_path);
}

fn init_keys(path: &str) {
    let hab_shell_pub_file = format!("{}/{}", path, HAB_SHELL_PUB_NAME);
    let hab_shell_sig_key_file = format!("{}/{}", path, HAB_SHELL_SIG_KEY_NAME);
    fs::write(hab_shell_pub_file, HAB_SHELL_PUB_CONTENT).unwrap();
    fs::write(hab_shell_sig_key_file, HAB_SHELL_SIG_KEY_CONTENT).unwrap();
}

pub fn init() {
    init_hab_shell_dir();
}
