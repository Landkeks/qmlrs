extern crate pkg_config;

use std::process::Command;
use std::process::Stdio;
use std::fs;
use std::path::Path;
use std::env;
use std::path::PathBuf;

fn main() {
    let wcd = env::current_dir().unwrap();
    let build = PathBuf::from(&wcd.join("ext/libqmlrswrapper/build"));
    let _ = fs::create_dir_all(&build);

    let mut cmake_command = Command::new("cmake");

    let mut make_command = Command::new("cmake");
    make_command.args(&vec!["--build","."]).current_dir(&build);

    if cfg!(windows) {
        let qt_dir = env::var("QTDIR").unwrap_or_else(|_| {
            panic!("Environment variable QTDIR not set");
        });

        // apparently, there's no way around hard-coding the generator
        cmake_command.args(&vec!["-GVisual Studio 14 2015 Win64", ".."]).current_dir(&build);

        // make sure only one environment variable needs to be set
        cmake_command.env("CMAKE_PREFIX_PATH", qt_dir);

        // silence output (MSVC output might contain non-UTF-8 characters depending on language)
        // cargo will fail if the output cannot be parsed as UTF-8
        make_command.stdout(Stdio::null());
    } else {
        cmake_command.args(&vec![".."]).current_dir(&build);
    }

    cmake_command.status().unwrap_or_else(|e| {
        panic!("Failed to run build: {}", e);
    });

    make_command.status().unwrap_or_else(|e| {
        panic!("Failed to run build: {}", e);
    });

    println!("cargo:rustc-link-lib=static=qmlrswrapper");

    if cfg!(windows) {
        println!("cargo:rustc-link-search=native={}\\system32", env::var("WINDIR").unwrap());
        println!("cargo:rustc-link-search=native={}\\Debug", build.display());
        println!("cargo:rustc-link-search=native={}\\lib", env::var("QTDIR").unwrap());

        println!("cargo:rustc-link-lib=dylib=Qt5Core");
        println!("cargo:rustc-link-lib=dylib=Qt5Gui");
        println!("cargo:rustc-link-lib=dylib=Qt5Qml");
        println!("cargo:rustc-link-lib=dylib=Qt5Quick");
    } else {
        println!("cargo:rustc-link-search=native={}", build.display());
        println!("cargo:rustc-link-lib=dylib=stdc++");
        pkg_config::find_library("Qt5Core Qt5Gui Qt5Qml Qt5Quick").unwrap();
    }

    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=framework=QtCore");
        println!("cargo:rustc-link-lib=framework=QtGui");
        println!("cargo:rustc-link-lib=framework=QtQml");
        println!("cargo:rustc-link-lib=framework=QtQuick");
    }
}
