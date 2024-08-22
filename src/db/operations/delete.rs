use crate::db::storage::principal_store::PrincipalStore;

pub fn delete_secret(store: &mut PrincipalStore, id: &str) -> bool {
    store.remove_secret(&id.to_string())
}
