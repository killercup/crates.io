use time::Timespec;

use pg;
use Model;
use db::Connection;
use util::CargoResult;

pub struct VersionDownload {
    pub id: i32,
    pub version_id: i32,
    pub downloads: i32,
    pub counted: i32,
    pub date: Timespec,
}

#[deriving(Encodable, Decodable)]
pub struct EncodableVersionDownload {
    pub id: i32,
    pub version: i32,
    pub downloads: i32,
    pub date: String,
}

impl VersionDownload {
    pub fn find(conn: &Connection, id: i32) -> CargoResult<VersionDownload> {
        Model::find(conn, id)
    }

    pub fn encodable(self) -> EncodableVersionDownload {
        let VersionDownload { id, version_id, downloads, counted: _,
                              date } = self;
        EncodableVersionDownload {
            id: id,
            version: version_id,
            downloads: downloads,
            date: ::encode_time(date),
        }
    }
}

impl Model for VersionDownload {
    fn from_row(row: &pg::Row) -> VersionDownload {
        VersionDownload {
            id: row.get("id"),
            version_id: row.get("version_id"),
            downloads: row.get("downloads"),
            counted: row.get("counted"),
            date: row.get("date"),
        }
    }

    fn table_name(_: Option<VersionDownload>) -> &'static str { "version_downloads" }
}

pub struct CrateDownload {
    pub id: i32,
    pub crate_id: i32,
    pub downloads: i32,
    pub date: Timespec,
}

impl CrateDownload {
    pub fn find(conn: &Connection, id: i32) -> CargoResult<CrateDownload> {
        Model::find(conn, id)
    }
}

impl Model for CrateDownload {
    fn from_row(row: &pg::Row) -> CrateDownload {
        CrateDownload {
            id: row.get("id"),
            crate_id: row.get("crate_id"),
            downloads: row.get("downloads"),
            date: row.get("date"),
        }
    }

    fn table_name(_: Option<CrateDownload>) -> &'static str { "crate_downloads" }
}
