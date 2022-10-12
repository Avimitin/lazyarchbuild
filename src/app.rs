use crossterm::event::KeyCode;

use crate::component::{
    self, menu,
    packages::{PkgInfo, PkgInfoBuilder},
};
use std::{
    collections::HashMap,
    sync::{atomic::AtomicBool, Arc},
};

pub enum DisplayMode {
    ViewingPackageStatusTable,
    /// Show menu for package status table
    PopUpPstMenu(menu::PopUpMenu),
}

pub enum InputMode {
    Normal,
    HasPrefix(Vec<KeyCode>),
}

/// A data collection of current running status, data resources...etc. Each modification will
/// triger a re-render.
pub struct App {
    current_display: DisplayMode,
    pub is_running: Arc<AtomicBool>,

    pub input_mode: InputMode,
    pub pkg_info_table: component::packages::PkgInfoTable,
}

impl std::default::Default for App {
    fn default() -> Self {
        Self {
            current_display: DisplayMode::ViewingPackageStatusTable,
            input_mode: InputMode::Normal,
            pkg_info_table: component::packages::PkgInfoTable::default(),
            is_running: Arc::new(AtomicBool::new(true)),
        }
    }
}

macro_rules! async_eval {
    ($code:block) => {
        tokio::task::block_in_place(move || {
            tokio::runtime::Handle::current().block_on(async move { $code })
        })
    };

    ($func:expr) => {
        tokio::task::block_in_place(move || tokio::runtime::Handle::current().block_on($func))
    };
}

impl App {
    pub fn reset_input_mode(&mut self) {
        self.input_mode = InputMode::Normal;
    }

    async fn fetch_data() -> Vec<PkgInfo> {
        use crate::req;

        enum Message {
            Felix(anyhow::Result<Vec<req::felix::PackageStatus>>),
            Melon(anyhow::Result<req::melon::Response>),
        }

        struct DownloadStatus {
            felix: bool,
            melon: bool,
        }

        impl DownloadStatus {
            fn is_done(&self) -> bool {
                self.felix && self.melon
            }
        }

        let mut status = DownloadStatus {
            felix: false,
            melon: false,
        };

        let (tx, rx) = std::sync::mpsc::channel();

        let tx1 = tx.clone();
        tokio::spawn(async move {
            let pkgs = req::melon::fetch().await;
            tx1.send(Message::Melon(pkgs))
        });

        tokio::spawn(async move {
            let pkgs = req::felix::PackageStatus::download().await;
            tx.send(Message::Felix(pkgs))
                .expect("Unexpected closed channel during sending download message");
        });

        let mut buffer: HashMap<Box<str>, PkgInfoBuilder> = HashMap::new();

        loop {
            if status.is_done() {
                break;
            }

            let msg = rx
                .recv()
                .expect("Unexpected lose connecion to the download status message sender");
            match msg {
                Message::Felix(pkgs) => {
                    status.felix = true;
                    let pkgs = pkgs.unwrap_or_else(|err| {
                        panic!("fail to download package information from felixc page: {err}")
                    });

                    for pkg in pkgs {
                        if buffer.get_mut(&pkg.pkgname).is_none() {
                            let builder = PkgInfoBuilder::default().name(pkg.pkgname.clone());
                            buffer.insert(pkg.pkgname, builder);
                        }
                    }
                }
                Message::Melon(pkgs) => {
                    status.melon = true;
                    let pkgs = pkgs.unwrap_or_else(|err| {
                        panic!("fail to download package information from melon bot: {err}")
                    });

                    if pkgs.marklist.is_empty() && pkgs.worklist.is_empty() {
                        continue;
                    }

                    for mark in pkgs.marklist {
                        let builder = buffer
                            .remove(&mark.name)
                            .unwrap_or_else(|| PkgInfoBuilder::default().name(mark.name.clone()));
                        let builder = builder.marks(mark.marks);
                        buffer.insert(mark.name, builder);
                    }

                    for work in pkgs.worklist {
                        for pack in work.packages {
                            let builder = buffer
                                .remove(pack.as_ref())
                                .unwrap_or_else(|| PkgInfoBuilder::default().name(pack.clone()));
                            let builder = builder.assignee(work.alias.clone());
                            buffer.insert(pack, builder);
                        }
                    }
                }
            }
        }

        buffer
            .into_values()
            .map(|builder| builder.build().unwrap())
            .filter(|pkg| pkg.has_assignee() || pkg.has_process() || pkg.has_marks())
            .collect::<Vec<_>>()
    }

    pub fn update(&mut self) -> anyhow::Result<()> {
        let new_data = async_eval!(App::fetch_data());

        self.pkg_info_table.data = new_data;

        Ok(())
    }

    /// Mutate self data based on the input
    pub fn handle_input(&mut self, keycode: KeyCode) {
        match &self.input_mode {
            InputMode::Normal => self.handle_normal_input(keycode),
            InputMode::HasPrefix(prefix) => self.handle_input_with_prefix(keycode, prefix.to_vec()),
        }
    }

    pub fn shutdown(&mut self) {
        self.is_running
            .store(false, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn is_running(&self) -> bool {
        self.is_running.load(std::sync::atomic::Ordering::SeqCst)
    }

    fn handle_normal_input(&mut self, keycode: KeyCode) {
        match keycode {
            KeyCode::Char('q') => self.shutdown(),
            KeyCode::Char('G') => self.key_end(),
            KeyCode::Char('g') => {
                self.input_mode = InputMode::HasPrefix(vec![KeyCode::Char('g')]);
            }
            KeyCode::Up | KeyCode::Char('j') => self.key_up(),
            KeyCode::Down | KeyCode::Char('k') => self.key_down(),
            #[allow(clippy::single_match)]
            KeyCode::Enter => match self.current_display() {
                DisplayMode::ViewingPackageStatusTable => {
                    self.show_pst_menu();
                }
                _ => (),
            },
            _ => (),
        }
    }

    fn handle_input_with_prefix(&mut self, keycode: KeyCode, prefix: Vec<KeyCode>) {
        if prefix.is_empty() {
            panic!("some logical error occurs for keys");
        }
        match prefix[0] {
            KeyCode::Char('g') => match keycode {
                // handle key 'gg'
                KeyCode::Char('g') => {
                    self.key_begining();
                    self.reset_input_mode();
                }
                _ => self.reset_input_mode(),
            },
            _ => self.reset_input_mode(),
        }
    }

    pub fn current_display(&self) -> &DisplayMode {
        &self.current_display
    }

    /// Set current display to a menu that showing available option for current selection
    pub fn show_pst_menu(&mut self) {
        let current_selection = self.pkg_info_table.cursor.selected().unwrap_or(0);
        let data = self.pkg_info_table.data.get(current_selection);
        if data.is_none() {
            return;
        }
        let data = data.unwrap();

        let mut menu_items = Vec::new();

        if !data.has_assignee() {
            menu_items.push("Assign");
        }

        if data.has_marks() {
            menu_items.push("Check Marks")
        }

        // TODO: Add drop menu when assigness is user

        menu_items.push("View package details");
        menu_items.push("View package build log");

        self.current_display = DisplayMode::PopUpPstMenu(menu::PopUpMenu::from(&menu_items));
    }

    pub fn key_down(&mut self) {
        use DisplayMode::*;
        let table = &mut self.pkg_info_table;
        match &mut self.current_display {
            ViewingPackageStatusTable => table.next(),
            PopUpPstMenu(ref mut menu) => {
                menu.next();
            }
        }
    }

    pub fn key_up(&mut self) {
        use DisplayMode::*;
        let table = &mut self.pkg_info_table;
        match &mut self.current_display {
            ViewingPackageStatusTable => table.previous(),
            PopUpPstMenu(ref mut menu) => {
                menu.previous();
            }
        }
    }

    pub fn key_begining(&mut self) {
        let table = &mut self.pkg_info_table;
        if let DisplayMode::ViewingPackageStatusTable = self.current_display {
            table.beginning()
        }
    }

    pub fn key_end(&mut self) {
        let table = &mut self.pkg_info_table;
        if let DisplayMode::ViewingPackageStatusTable = self.current_display {
            table.end()
        }
    }
}
