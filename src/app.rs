use crate::component::{self, packages::PkgInfo};
use std::{sync::{Arc, Mutex}, collections::HashMap};

pub enum CurrentPanel {
    Unfocus,
    PackageStatusPanel,
}

pub struct App {
    stats: CurrentPanel,

    pub pkg_info_table: component::packages::PkgInfoTable,
    fetch_status: Arc<Mutex<FetchStatus>>,
}

struct FetchStatus {
    felix: bool,
    melon: bool,
}

impl std::default::Default for App {
    fn default() -> Self {
        Self {
            stats: CurrentPanel::Unfocus,
            pkg_info_table: component::packages::PkgInfoTable::default(),
            fetch_status: Arc::new(Mutex::new(FetchStatus {
                felix: false,
                melon: false,
            })),
        }
    }
}

macro_rules! async_eval {
    ($code:block) => {
        tokio::task::block_in_place(move || tokio::runtime::Handle::current().block_on($code))
    };
}

impl App {
    pub fn update(&mut self) -> anyhow::Result<()> {
        use crate::req;

        let felix_status =
            async_eval!({ async move { tokio::join!(req::felix::PackageStatus::download()) } });

        if let (Ok(status),) = felix_status {
            let data = status
                .into_iter()
                .map(|s| (s.pkgname.clone(), PkgInfo::builder().name(s.pkgname).build()))
                .collect::<HashMap<_, _>>();
            self.pkg_info_table.data = data;
        }

        Ok(())
    }

    pub fn stats(&self) -> &CurrentPanel {
        &self.stats
    }

    pub fn key_down(&mut self) {
        let table = &mut self.pkg_info_table;
        match self.stats {
            CurrentPanel::PackageStatusPanel => table.next(),
            CurrentPanel::Unfocus => table.next(),
        }
    }

    pub fn key_up(&mut self) {
        let table = &mut self.pkg_info_table;
        match self.stats {
            CurrentPanel::PackageStatusPanel => table.previous(),
            CurrentPanel::Unfocus => table.previous(),
        }
    }

    pub fn key_right(&mut self) {
        match self.stats {
            CurrentPanel::PackageStatusPanel => (),
            CurrentPanel::Unfocus => (),
        }
    }

    pub fn key_left(&mut self) {
        match self.stats {
            CurrentPanel::PackageStatusPanel => (),
            CurrentPanel::Unfocus => (),
        }
    }
}
