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

    let new_branch = CustomMenuItem::new("new_branch".to_string(), "新建");
    let rename_branch = CustomMenuItem::new("rename_branch".to_string(), "重命名");
    let delete_branch = CustomMenuItem::new("delete_branch".to_string(), "删除");
    let discard_changes = CustomMenuItem::new("discard_changes".to_string(), "放弃更改");
    let stash = CustomMenuItem::new("stash".to_string(), "暂存更改");
    let fetch = CustomMenuItem::new("fetch".to_string(), "获取更新");
    let compare = CustomMenuItem::new("compare".to_string(), "与其他分支比较");
    let merge = CustomMenuItem::new("merge".to_string(), "合并到当前分支");
    let zip_merge = CustomMenuItem::new("zip_merge".to_string(), "压缩合并到当前分支");
    let rebase = CustomMenuItem::new("rebase".to_string(), "变基当前分支");
    let compare_web = CustomMenuItem::new("compare_web".to_string(), "在 Github 上比较");
    let view_web = CustomMenuItem::new("view_web".to_string(), "在 Github 上查看");
    let preview_pr = CustomMenuItem::new("preview_pr".to_string(), "预览拉取请求");
    let pr_web = CustomMenuItem::new("pr_web".to_string(), "在 Github 上查看拉取请求");

    let branch_menu = Submenu::new(
        "分支",
        Menu::new()
        .add_item(new_branch)
        .add_item(rename_branch)
        .add_item(delete_branch)
        .add_native_item(MenuItem::Separator)
        .add_item(discard_changes)
        .add_item(stash)
        .add_item(fetch)
        .add_native_item(MenuItem::Separator)
        .add_item(compare)
        .add_item(merge)
        .add_item(zip_merge)
        .add_item(rebase)
        .add_native_item(MenuItem::Separator)
        .add_item(compare_web)
        .add_item(view_web)
        .add_item(preview_pr)
        .add_item(pr_web)
    ); // 分支菜单

    let issue_url = CustomMenuItem::new("issue_url".to_string(), "问题反馈");
    let guide = CustomMenuItem::new("guide".to_string(), "用户指南");
    let short_key = CustomMenuItem::new("short_key".to_string(), "快捷键");
    let log = CustomMenuItem::new("log".to_string(), "日志");
    let about = CustomMenuItem::new("about".to_string(), "关于");

    let help_menu = Submenu::new(
        "帮助",
        Menu::new()
        .add_item(issue_url)
        .add_item(guide)
        .add_item(short_key)
        .add_item(log)
        .add_native_item(MenuItem::Separator)
        .add_item(about)
    );

    let menu = Menu::new()
        .add_submenu(file_menu)
        .add_submenu(edit_menu)
        .add_submenu(view_menu)
        .add_submenu(repo_menu)
        .add_submenu(branch_menu)
        .add_submenu(help_menu);

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


