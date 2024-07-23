// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

fn main() {
    let new_repo = CustomMenuItem::new("New_repo".to_string(), "新建仓库");
    let add_repo = CustomMenuItem::new("add_repo".to_string(), "添加仓库");
    let clone_repo: CustomMenuItem = CustomMenuItem::new("clone_repo".to_string(), "克隆仓库");
    let opt: CustomMenuItem = CustomMenuItem::new("opt".to_string(), "设置");
    let quit = CustomMenuItem::new("quit".to_string(), "退出");

    let file_menu = Submenu::new(
        "文件",
        Menu::new()
            .add_item(new_repo)
            .add_native_item(MenuItem::Separator)
            .add_item(add_repo)
            .add_item(clone_repo)
            .add_native_item(MenuItem::Separator)
            .add_item(opt)
            .add_native_item(MenuItem::Separator)
            .add_item(quit)
    );

    let undo = CustomMenuItem::new("undo".to_string(), "撤销");
    let redo = CustomMenuItem::new("redo".to_string(), "重做");
    let cut = CustomMenuItem::new("cut".to_string(), "剪切");
    let copy = CustomMenuItem::new("copy".to_string(), "复制");
    let paste = CustomMenuItem::new("paste".to_string(), "粘贴");
    let all: CustomMenuItem = CustomMenuItem::new("all".to_string(), "全选");
    let find = CustomMenuItem::new("find".to_string(), "查找");

    let edit_menu = Submenu::new(
        "编辑",
        Menu::new()
            .add_item(undo)
            .add_item(redo)
            .add_native_item(MenuItem::Separator)
            .add_item(cut)
            .add_item(copy)
            .add_item(paste)
            .add_item(all)
            .add_native_item(MenuItem::Separator)
            .add_item(find)
    );

    let change = CustomMenuItem::new("change".to_string(), "更改");
    let history = CustomMenuItem::new("history".to_string(), "历史记录");
    let repo_list = CustomMenuItem::new("repo_list".to_string(), "仓库列表");
    let branch_list = CustomMenuItem::new("branch_list".to_string(), "分支列表");
    let full_screen = CustomMenuItem::new("fullscreen".to_string(), "全屏");

    let view_menu = Submenu::new(
        "视图",
        Menu::new()
            .add_item(change)
            .add_item(history)
            .add_item(repo_list)
            .add_item(branch_list)
            .add_native_item(MenuItem::Separator)
            .add_item(full_screen)
    );

    let pull = CustomMenuItem::new("pull".to_string(), "拉取");
    let _sync = CustomMenuItem::new("sync".to_string(), "同步");
    let del = CustomMenuItem::new("del".to_string(), "删除");
    let upload = CustomMenuItem::new("upload".to_string(), "上传");
    let view_on_github = CustomMenuItem::new("view_on_github".to_string(), "在 Github 上查看");
    let open_cmd = CustomMenuItem::new("open_cmd".to_string(), "在终端打开");
    let open_explorer = CustomMenuItem::new("open_explorer".to_string(), "在资源管理器中打开");

    let repo_menu = Submenu::new(
        "仓库",
        Menu::new()
        .add_item(upload)
        .add_item(pull)
        .add_item(_sync)
        .add_item(del)
        .add_native_item(MenuItem::Separator)
        .add_item(view_on_github)
        .add_item(open_cmd)
        .add_item(open_explorer)
    ); // 仓库菜单

    let menu = Menu::new()
        .add_submenu(file_menu)
        .add_submenu(edit_menu)
        .add_submenu(view_menu)
        .add_submenu(repo_menu);

    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| {
            match event.menu_item_id() {
                "new_repo" => {
                    println!("Open menu item clicked");
                }
                "save" => {
                    println!("Save menu item clicked");
                }
                "quit" => {
                    std::process::exit(0);
                }
                "undo" => {
                    println!("Undo menu item clicked");
                }
                "redo" => {
                    println!("Redo menu item clicked");
                }
                "cut" => {
                    println!("Cut menu item clicked");
                }
                "copy" => {
                    println!("Copy menu item clicked");
                }
                "paste" => {
                    println!("Paste menu item clicked");
                }
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


