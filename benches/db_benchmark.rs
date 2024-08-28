use criterion::{criterion_group, criterion_main, Criterion};
use secret_db::db::core::DatabaseCore;
use rand::Rng;

fn generate_random_string(len: usize) -> String {
    let charset = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();

    (0..len)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect()
}

fn benchmark_secret_creation(c: &mut Criterion) {
    let db_core = DatabaseCore::new();
    let num_operations = 500;
    let payload_size = 256;
    let expiration = 3600;

    c.bench_function("secret_creation", |b| {
        b.iter(|| {
            for _ in 0..num_operations {
                let credential = DatabaseCore::generate_random_credential();
                let payload = generate_random_string(payload_size);

                // CREATE operation
                let _secret = db_core.create_secret(&payload, expiration, &credential).unwrap();
            }
        })
    });
}

fn benchmark_secret_creation_and_reading(c: &mut Criterion) {
    let db_core = DatabaseCore::new();
    let num_operations = 500;
    let payload_size = 256;
    let expiration = 3600;

    c.bench_function("secret_creation_and_reading", |b| {
        b.iter(|| {
            let mut created_secrets = Vec::new();

            // Crear secretos
            for _ in 0..num_operations {
                let credential = DatabaseCore::generate_random_credential();
                let payload = generate_random_string(payload_size);

                let secret = db_core.create_secret(&payload, expiration, &credential).unwrap();
                created_secrets.push((secret, credential));
            }

            // Leer secretos
            for (secret, credential) in created_secrets {
                let _ = db_core.read_secret(&secret.id).unwrap().unwrap().decrypt(&credential);
            }
        })
    });
}

fn benchmark_secret_creation_and_deletion(c: &mut Criterion) {
    let db_core = DatabaseCore::new();
    let num_operations = 1000;
    let payload_size = 256;
    let expiration = 3600;

    c.bench_function("secret_creation_and_deletion", |b| {
        b.iter(|| {
            let mut created_secrets = Vec::new();

            // Crear secretos
            for _ in 0..num_operations {
                let credential = DatabaseCore::generate_random_credential();
                let payload = generate_random_string(payload_size);

                let secret = db_core.create_secret(&payload, expiration, &credential).unwrap();
                created_secrets.push(secret);
            }

            // Eliminar secretos
            for secret in created_secrets {
                let _ = db_core.delete_secret(&secret.id);
            }
        })
    });
}

fn benchmark_mixed_operations(c: &mut Criterion) {
    let db_core = DatabaseCore::new();
    let num_operations = 1000;
    let payload_size = 256;
    let expiration = 3600;

    c.bench_function("mixed_operations", |b| {
        b.iter(|| {
            let mut created_secrets = Vec::new();

            // Crear secretos
            for _ in 0..num_operations {
                let credential = DatabaseCore::generate_random_credential();
                let payload = generate_random_string(payload_size);

                let secret = db_core.create_secret(&payload, expiration, &credential).unwrap();
                created_secrets.push((secret, credential));
            }

            // Operaciones mixtas: Leer la mitad y eliminar la mitad
            let midpoint = num_operations / 2;

            // Leer la primera mitad
            for i in 0..midpoint {
                let (secret, credential) = &created_secrets[i];
                let _ = db_core.read_secret(&secret.id).unwrap().unwrap().decrypt(credential);
            }

            // Eliminar la segunda mitad
            for i in midpoint..num_operations {
                let (secret, _) = &created_secrets[i];
                let _ = db_core.delete_secret(&secret.id);
            }
        })
    });
}

// TODO: Make the benchmarks commented work (something wrong with deletes)
criterion_group!(
    benches,
    benchmark_secret_creation,
    benchmark_secret_creation_and_reading,
    benchmark_secret_creation_and_deletion,
    benchmark_mixed_operations
);
criterion_main!(benches);
