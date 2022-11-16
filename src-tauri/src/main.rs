#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use git2::build::{CheckoutBuilder, RepoBuilder};
use git2::{FetchOptions, Progress, RemoteCallbacks};
use std::cell::RefCell;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use std::process::Command;

fn download_godot() {}
use std::ascii::escape_default;
use std::str;

#[tauri::command]
fn launch_game_from_source() {
    // we require export templates to be installed
    println!("Starting Game from Source Code");

    let output2 = Command::new("godot\\godot.exe")
        .args(["--path", "repo", "--client"])
        .spawn()
        .expect("failed to execute process");
}

#[tauri::command]
fn build_game() {
    // we require export templates to be installed
    println!("Starting Engine");

    let output2 = Command::new("godot\\godot.exe")
        .args([
            "--path",
            "repo",
            "--export",
            "WindowsDesktop",
            "../game/DungeonsAndDungeons.pck",
        ])
        .output()
        .expect("failed to execute process");

    let hello = output2.stdout;
    let result = String::from_utf8(hello);
    let mut resultstr = String::from("No result");
    match result {
        Ok(v) => {
            resultstr = v;
        }
        Err(e) => println!("Could not parse return"),
    }
    println!("{:#?}", resultstr.lines());
}

#[tauri::command]
fn launch_game() {
    // we require export templates to be installed
    println!("Starting Game");

    let output2 = Command::new("game\\DungeonsAndDungeons.exe")
        .output()
        .expect("failed to execute process");

    let hello = output2.stdout;
    let result = String::from_utf8(hello);
    let mut resultstr = String::from("No result");
    match result {
        Ok(v) => {
            resultstr = v;
        }
        Err(e) => println!("Could not parse return"),
    }
    println!("{:#?}", resultstr.lines());
}

#[tauri::command]
fn update_repo(path: &str) {
    print!("Fetching Repo to {}", path);
    let mut fetch = false;
    let args = Args {
        arg_path: String::from("repo"),
        arg_url: String::from("https://github.com/Mineorbit/DungeonsAndDungeons"),
    };
    match run(&args) {
        Ok(()) => {}
        Err(e) => {
            println!("error: {}", e);
            fetch = true
        }
    }
    if fetch {
        println!("Repo fetched!");
    }

    println!("Ready to build!");
}

struct Args {
    arg_url: String,
    arg_path: String,
}

struct State {
    progress: Option<Progress<'static>>,
    total: usize,
    current: usize,
    path: Option<PathBuf>,
    newline: bool,
}

fn print(state: &mut State) {
    let stats = state.progress.as_ref().unwrap();
    let network_pct = (100 * stats.received_objects()) / stats.total_objects();
    let index_pct = (100 * stats.indexed_objects()) / stats.total_objects();
    let co_pct = if state.total > 0 {
        (100 * state.current) / state.total
    } else {
        0
    };
    let kbytes = stats.received_bytes() / 1024;
    if stats.received_objects() == stats.total_objects() {
        if !state.newline {
            println!();
            state.newline = true;
        }
        print!(
            "Resolving deltas {}/{}\r",
            stats.indexed_deltas(),
            stats.total_deltas()
        );
    } else {
        print!(
            "net {:3}% ({:4} kb, {:5}/{:5})  /  idx {:3}% ({:5}/{:5})  \
             /  chk {:3}% ({:4}/{:4}) {}\r",
            network_pct,
            kbytes,
            stats.received_objects(),
            stats.total_objects(),
            index_pct,
            stats.indexed_objects(),
            stats.total_objects(),
            co_pct,
            state.current,
            state.total,
            state
                .path
                .as_ref()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_default()
        )
    }
    io::stdout().flush().unwrap();
}

fn run(args: &Args) -> Result<(), git2::Error> {
    let state = RefCell::new(State {
        progress: None,
        total: 0,
        current: 0,
        path: None,
        newline: false,
    });
    let mut cb = RemoteCallbacks::new();
    cb.transfer_progress(|stats| {
        let mut state = state.borrow_mut();
        state.progress = Some(stats.to_owned());
        print(&mut *state);
        true
    });

    let mut co = CheckoutBuilder::new();
    co.progress(|path, cur, total| {
        let mut state = state.borrow_mut();
        state.path = path.map(|p| p.to_path_buf());
        state.current = cur;
        state.total = total;
        print(&mut *state);
    });

    let mut fo = FetchOptions::new();
    fo.remote_callbacks(cb);
    RepoBuilder::new()
        .fetch_options(fo)
        .with_checkout(co)
        .clone(&args.arg_url, Path::new(&args.arg_path))?;
    println!();

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            update_repo,
            build_game,
            launch_game,
            launch_game_from_source
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
