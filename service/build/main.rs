use std::{env, path::PathBuf};

fn check_rustc_versions() {
    let cfg = autocfg::new();
    if cfg.probe_rustc_version(1, 51) {
        println!("cargo:rustc-cfg=const_fn");
    }
}

fn try_load_tassl_from_env(target: &str) -> Option<PathBuf> {
    let lib_path = env::var_os("TASSL_LIB_PATH");
    if lib_path.is_none() {
        return None;

    }
    let lib_path = PathBuf::from(lib_path.unwrap());
    if !lib_path.exists() {
        return None;
    }

    let include_path = env::var_os("TASSL_INCLUDE_PATH");
    if include_path.is_none() {
        return None;
    }
    let include_path = PathBuf::from(include_path.unwrap());
    if !include_path.exists() {
        return None;
    }

    let lib_kind = env::var("TASSL_LIB_KIND").unwrap_or(String::from("static"));
    if lib_kind.ne("static") && lib_kind.ne("dylib") {
        panic!("Only support static or dylib link lib");
    }
    println!("cargo:rustc-link-search=native={}", lib_path.display());
    if target.contains("windows") {
        println!("cargo:rustc-link-lib={}=ssleay32", lib_kind);
        println!("cargo:rustc-link-lib={}=libeay32", lib_kind);
        if lib_kind.eq("static") {
            println!("cargo:rustc-link-lib=dylib=gdi32");
            println!("cargo:rustc-link-lib=dylib=user32");
            println!("cargo:rustc-link-lib=dylib=crypt32");
            println!("cargo:rustc-link-lib=dylib=ws2_32");
            println!("cargo:rustc-link-lib=dylib=advapi32");
        }
    } else {
        println!("cargo:rustc-link-lib={}=ssl", lib_kind);
        println!("cargo:rustc-link-lib={}=crypto", lib_kind);
    }
    println!("cargo:include={}", include_path.display());
    println!("cargo:lib={}", lib_path.display());
    Some(include_path)
}

pub fn get_cfgs(openssl_version: u64) -> Vec<&'static str> {
    let mut cfgs = vec![];
    if openssl_version >= 0x3_00_00_00_0 {
        cfgs.push("ossl300");
    }
    if openssl_version >= 0x1_00_01_00_0 {
        cfgs.push("ossl101");
    }
    if openssl_version >= 0x1_00_02_00_0 {
        cfgs.push("ossl102");
    }
    if openssl_version >= 0x1_00_02_06_0 {
        cfgs.push("ossl102f");
    }
    if openssl_version >= 0x1_00_02_08_0 {
        cfgs.push("ossl102h");
    }
    if openssl_version >= 0x1_01_00_00_0 {
        cfgs.push("ossl110");
    }
    if openssl_version >= 0x1_01_00_06_0 {
        cfgs.push("ossl110f");
    }
    if openssl_version >= 0x1_01_00_07_0 {
        cfgs.push("ossl110g");
    }
    if openssl_version >= 0x1_01_00_08_0 {
        cfgs.push("ossl110h");
    }
    if openssl_version >= 0x1_01_01_00_0 {
        cfgs.push("ossl111");
    }
    if openssl_version >= 0x1_01_01_02_0 {
        cfgs.push("ossl111b");
    }
    if openssl_version >= 0x1_01_01_03_0 {
        cfgs.push("ossl111c");
    }
    cfgs
}

fn parse_version(version: &str) -> u64 {
    assert!(version.starts_with("0x"));
    let version = &version[2..];
    let version = version.trim_end_matches(|c: char| match c {
        '0'..='9' | 'a'..='f' | 'A'..='F' => false,
        _ => true,
    });
    u64::from_str_radix(version, 16).unwrap()
}

fn validate_headers(include_path: &PathBuf) {
    let mut gcc = cc::Build::new();
    gcc.include(include_path);
    let expanded = match gcc.file("build/expando.c").try_expand() {
        Ok(expanded) => expanded,
        Err(e) => {
            panic!(
                "
Header expansion error:
{:?}

Failed to find TASSL development headers.
",
                e
            );
        }
    };
    let expanded = String::from_utf8(expanded).unwrap();
    let mut enabled = vec![];
    let mut openssl_version = None;
    for line in expanded.lines() {
        let line = line.trim();
        let openssl_prefix = "RUST_VERSION_OPENSSL_";
        let conf_prefix = "RUST_CONF_";
        if line.starts_with(openssl_prefix) {
            let version = &line[openssl_prefix.len()..];
            openssl_version = Some(parse_version(version));
        } else if line.starts_with(conf_prefix) {
            enabled.push(&line[conf_prefix.len()..]);
        }
    }
    let openssl_version = openssl_version.unwrap();
    for enabled in &enabled {
        println!("cargo:rustc-cfg=osslconf=\"{}\"", enabled);
    }
    println!("cargo:conf={}", enabled.join(","));
    for cfg in get_cfgs(openssl_version) {
        println!("cargo:rustc-cfg={}", cfg);
    }
}

fn main() {
    check_rustc_versions();
    let target = env::var("TARGET").unwrap();
    let include_path = match try_load_tassl_from_env(&target) {
        Some(v) => v,
        None => {
            let artifacts = tassl_src::Builder::default().build();
            artifacts.print_cargo_metadata();
            artifacts.include_dir
        }
    };
    validate_headers(&include_path);
}