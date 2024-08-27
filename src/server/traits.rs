use crate::db::core::DatabaseCore;

pub trait Server {
    fn start(&self, db_core: &DatabaseCore);
}
