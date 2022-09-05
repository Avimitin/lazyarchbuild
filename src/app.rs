use crate::component;

pub enum CurrentPanel {
    Unfocus,
    PackageStatusPanel,
}

pub struct App {
    stats: CurrentPanel,

    pkg_info_table: component::packages::PkgInfoTable,
}

impl std::default::Default for App {
    fn default() -> Self {
        Self {
            stats: CurrentPanel::Unfocus,
            pkg_info_table: component::packages::PkgInfoTable::default(),
        }
    }
}

impl App {

    pub fn key_down(&mut self) {
        match self.stats {
            CurrentPanel::PackageStatusPanel => self.pkg_info_table.next(),
            CurrentPanel::Unfocus => self.pkg_info_table.next(),
        }
    }

    pub fn key_up(&mut self) {
        match self.stats {
            CurrentPanel::PackageStatusPanel => self.pkg_info_table.previous(),
            CurrentPanel::Unfocus => self.pkg_info_table.next(),
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
