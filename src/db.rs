use rocksdb::DB;

pub fn open_db() -> DB {
    let path = "./rocksdb";
    let db = DB::open_default(path).expect("Failed to open RocksDB");
    return db;
}

#[cfg(test)]
mod test {
    // Import the testing module
    use super::*;

    #[test]
    pub fn test_db() {
        // Open a RocksDB database
        let path = "./rocksdb";
        let db = DB::open_default(path).expect("Failed to open RocksDB");

        // Write data to RocksDB
        let key = "example_key";
        let value = "example_value";
        db.put(key.as_bytes(), value.as_bytes())
            .expect("Failed to write to RocksDB");

        // Read data from RocksDB
        let read_result = db.get(key.as_bytes());
        match read_result {
            Ok(Some(value)) => {
                let value_str = String::from_utf8_lossy(&value);
                println!("Read from RocksDB: {}", value_str);
            }
            Ok(None) => println!("Key not found in RocksDB"),
            Err(err) => eprintln!("Error reading from RocksDB: {:?}", err),
        }
    }
}
