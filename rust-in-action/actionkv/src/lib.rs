use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use crc::{Crc, CRC_32_CKSUM};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{self, BufReader, BufWriter, Read, Seek, SeekFrom, Write},
    path::Path,
};

pub const CRC_32: Crc<u32> = Crc::<u32>::new(&CRC_32_CKSUM);

type ByteString = Vec<u8>;
type ByteStr = [u8];

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct KeyValuePair {
    pub key: ByteString,
    pub value: ByteString,
}

#[derive(Debug)]
pub struct ActionKV {
    f: File,
    // map of keys to file positions
    pub index: HashMap<ByteString, u64>,
}

impl ActionKV {
    // Creates an ActionKV instance with an opened file and empty index.
    pub fn open(path: &Path) -> io::Result<Self> {
        Ok(ActionKV {
            f: OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .append(true)
                .open(path)?,
            index: HashMap::new(),
        })
    }

    // Populates the index by mapping keys to file positions
    pub fn load(&mut self) -> io::Result<()> {
        let mut f = BufReader::new(&mut self.f);

        loop {
            let start_position = f.stream_position()?;

            let kv = match ActionKV::process_record(&mut f) {
                Ok(kv) => kv,
                Err(err) => match err.kind() {
                    io::ErrorKind::UnexpectedEof => {
                        break;
                    }
                    _ => return Err(err),
                },
            };

            self.index.insert(kv.key, start_position);
        }

        Ok(())
    }

    // Process a Bitcask file format record from anything that is Read (e.g. File or &[u8])
    //
    // Bitcast format:
    // +----------+---------+---------+---------------+----------------+
    // | checksum | key_ken | val_len | key           | value          |
    // +----------+---------+---------+---------------+----------------+
    //  u32        u32       u32       [u8; key_len]   [u8; val_len]
    //
    //  panics if the stored checksum in the metadata is not equal to the calculated checksum.
    fn process_record<R: Read>(f: &mut R) -> io::Result<KeyValuePair> {
        // Read metadata
        let checksum = f.read_u32::<LittleEndian>()?;
        let key_len = f.read_u32::<LittleEndian>()?;
        let val_len = f.read_u32::<LittleEndian>()?;

        let data_len = key_len + val_len;

        // Create buffer for data to be filled from file
        let mut data = ByteString::with_capacity(data_len as usize);

        f.by_ref().take(data_len as u64).read_to_end(&mut data)?;

        debug_assert_eq!(data.len(), data_len as usize);

        if CRC_32.checksum(&data) != checksum {
            panic!("data corruption encountered, checksum mismatched")
        }

        let value = data.split_off(key_len as usize);
        let key = data;

        Ok(KeyValuePair { key, value })
    }

    pub fn insert(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<()> {
        let mut f = BufWriter::new(&mut self.f);

        let key_len = key.len();
        let val_len = value.len();

        let mut data_buf = ByteString::with_capacity(key_len + val_len);
        data_buf.extend_from_slice(&key);
        data_buf.extend_from_slice(&value);

        let checksum = CRC_32.checksum(&data_buf);

        let start_position = f.stream_position()?;
        f.write_u32::<LittleEndian>(checksum)?;
        f.write_u32::<LittleEndian>(key_len as u32)?;
        f.write_u32::<LittleEndian>(val_len as u32)?;
        f.write_all(&mut data_buf)?;

        f.flush()?;
        self.index.insert(key.to_vec(), start_position);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{ActionKV, ByteString, KeyValuePair};
    use std::{collections::HashMap, fs::File, io::Seek};
    use tempfile::tempdir;

    fn open_store_at_tmp_file(file_name: &str) -> ActionKV {
        // Create tmp file
        let temp_dir = tempdir().expect("tempdir");
        let path = temp_dir.path().join(file_name);
        File::create(&path).expect("create tmp file");

        return ActionKV::open(path.as_path()).expect("open store");
    }

    fn create_store_with_test_data(file_name: &str, data: Vec<(&str, &str)>) -> ActionKV {
        let mut store = open_store_at_tmp_file(file_name);
        for (key, val) in data {
            store
                .insert(key.as_bytes(), val.as_bytes())
                .expect("insert");
        }

        return store;
    }

    #[test]
    fn test_process_record() {
        let checksum = &[216, 251, 175, 55];
        let u32_4_little_endian = &[4, 0, 0, 0];
        let key = "key1".as_bytes();
        let value = "val1".as_bytes();

        let mut data_buf = Vec::new();
        data_buf.extend_from_slice(checksum); // checksum
        data_buf.extend_from_slice(u32_4_little_endian); // key_len
        data_buf.extend_from_slice(u32_4_little_endian); // val_len
        data_buf.extend_from_slice(key.clone()); // key
        data_buf.extend_from_slice(value.clone()); // value

        let got_kv = ActionKV::process_record(&mut data_buf.as_slice()).expect("process_record");
        let expected_kv = KeyValuePair {
            key: key.to_vec(),
            value: value.to_vec(),
        };
        assert_eq!(got_kv, expected_kv);
    }

    #[test]
    fn test_load() {
        let mut store =
            create_store_with_test_data("test_load", vec![("key1", "val1"), ("key2", "val2")]);

        // reset index and file, then reload
        store.index.clear();
        store.f.rewind().expect("seek");
        store.load().expect("load");

        let expected_loaded_index: HashMap<ByteString, u64> =
            HashMap::from([(vec![107, 101, 121, 49], 0), (vec![107, 101, 121, 50], 20)]);
        assert_eq!(store.index, expected_loaded_index);
    }

    #[test]
    fn test_insert() {
        let store =
            create_store_with_test_data("test_insert", vec![("hehe", "haha"), ("huhu", "haha")]);

        let expected_index: HashMap<ByteString, u64> = HashMap::from([
            (vec![104, 101, 104, 101], 0),  // key "hehe" is written at index 0
            (vec![104, 117, 104, 117], 20), // key "huhu" is written at index 20
        ]);

        assert_eq!(store.index, expected_index);
    }
}
