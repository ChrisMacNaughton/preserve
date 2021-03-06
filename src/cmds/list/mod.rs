use backend;
use clap::ArgMatches;
use json;
use keystore::KeyStore;


pub fn execute(args: &ArgMatches) {
    #[cfg(not(feature="vault"))]
    let args_keyfile = args.value_of("keyfile").expect("internal error");
    let args_backend = args.value_of("backend").expect("internal error");

	#[cfg(feature="vault")]
    let keystore = match KeyStore::load_from_vault() {
        Ok(keystore) => keystore,
        Err(err) => {
            error!("Unable to load keyfile: {}", err);
            return;
        }
    };
    #[cfg(not(feature="vault"))]
    let keystore = match KeyStore::load_from_path(args_keyfile) {
        Ok(keystore) => keystore,
        Err(err) => {
            error!("Unable to load keyfile: {}", err);
            return;
        }
    };

    let mut backend = match backend::backend_from_backend_path(args_backend) {
        Ok(backend) => backend,
        Err(err) => {
            error!("Unable to load backend: {}", err);
            return;
        }
    };

    let encrypted_archive_names = match backend.list_archives() {
        Ok(names) => names,
        Err(err) => {
            error!("There was a problem listing the archives: {}", err);
            return;
        }
    };
    let mut archive_names = Vec::new();
    for encrypted_archive_name in &encrypted_archive_names {
        let archive_name = match keystore.decrypt_archive_name(encrypted_archive_name) {
            Ok(name) => name,
            Err(err) => {
                warn!("Could not decrypt one of the archive names, '{}', because: {}",
                      encrypted_archive_name.to_string(),
                      err);
                continue;
            }
        };
        archive_names.push(archive_name);
    }
    // Sort alphabetically
    archive_names.sort();

    if args.is_present("json") {
        println!("{}", json::stringify(archive_names));
    } else {
        println!("{}", archive_names.join("\n"))
    }

    if encrypted_archive_names.is_empty() {
        println!("No archives found");
    }
}
