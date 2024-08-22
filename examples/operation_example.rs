use secret_db::db::storage::principal_store::PrincipalStore;
use secret_db::db::operations::{create, read, delete};
use secret_db::utils::time::current_timestamp;

fn main() {
    let mut store = PrincipalStore::new();

    // Create operation
    let secret = create::create_secret(&mut store, "my_secret_1", current_timestamp() + 2000);
    println!("Created secret: {:?}", secret);

    // Read operations
    let non_existent_result = read::read_secret(&mut store, "NOT_EXISTING_ID");
    match non_existent_result {
        Some(secret) => println!("Read non-existent secret: {:?}", secret),
        None => println!("Non-existent secret not found, as expected"),
    }

    let read_result = read::read_secret(&mut store, &secret.id);
    match read_result {
        Some(secret) => println!("Read secret: {:?}", secret),
        None => println!("Secret not found, which is unexpected"),
    }

    // Delete operation
    let delete_result = delete::delete_secret(&mut store, &secret.id);
    if delete_result {
        println!("Secret deleted successfully");
    } else {
        println!("Failed to delete secret");
    }

    // Try to read the deleted secret
    let deleted_read_result = read::read_secret(&mut store, &secret.id);
    match deleted_read_result {
        Some(secret) => println!("Unexpectedly read deleted secret: {:?}", secret),
        None => println!("Deleted secret not found, as expected"),
    }
}