use crossterm::event::KeyCode;

use crate::component::{
    self,
    packages::{PkgInfo, PkgInfoBuilder},
};
use std::collections::HashMap;

pub enum DisplayMode {
    ViewingPackageStatusTable,
    /// Show menu for package status table
    PopUpPstMenu,
}

pub enum InputMode {
    Normal,
    HasPrefix(Vec<KeyCode>),
}

pub struct App {
    current_display: DisplayMode,

    pub input_mode: InputMode,
    pub pkg_info_table: component::packages::PkgInfoTable,
}

impl std::default::Default for App {
    fn default() -> Self {
        Self {
            current_display: DisplayMode::ViewingPackageStatusTable,
            input_mode: InputMode::Normal,
            pkg_info_table: component::packages::PkgInfoTable::default(),
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

    pub fn current_display(&self) -> &DisplayMode {
        &self.current_display
    }

    /// Set current display to a menu that showing available option for current selection
    pub fn show_pst_menu(&mut self) {
        self.current_display = DisplayMode::PopUpPstMenu;
    }

    pub fn key_down(&mut self) {
        let table = &mut self.pkg_info_table;
        match self.current_display {
            DisplayMode::ViewingPackageStatusTable => table.next(),
            _ => (),
        }
    }

    pub fn key_up(&mut self) {
        let table = &mut self.pkg_info_table;
        match self.current_display {
            DisplayMode::ViewingPackageStatusTable => table.previous(),
            _ => (),
        }
    }

    pub fn key_begining(&mut self) {
        let table = &mut self.pkg_info_table;
        match self.current_display {
            DisplayMode::ViewingPackageStatusTable => table.beginning(),
            _ => (),
        }
    }

    pub fn key_end(&mut self) {
        let table = &mut self.pkg_info_table;
        match self.current_display {
            DisplayMode::ViewingPackageStatusTable => table.end(),
            _ => (),
        }
    }
}
