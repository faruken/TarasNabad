use rocksdb::{DB, Options, Error};
use std::path::Path;
use uuid::Uuid;
use std::sync::Arc;

pub trait TarasNabadImpl {
    fn list(&self);
    fn put(&mut self, value: String) -> Uuid;
    fn get(&self, key: &Uuid) -> Result<Option<Vec<u8>>, Error>;
    // fn delete(&self, key: u64);
}


#[derive(Clone)]
pub struct TarasNabad {
    db: Arc<DB>,
}


impl Default for TarasNabad {
    #[inline]
    fn default() -> TarasNabad {
        let mut options = Options::default();
        options.create_if_missing(true);
        let db = Arc::new(DB::open_default("taras.db").unwrap());
        TarasNabad { db }
    }
}


impl TarasNabad {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let mut options = Options::default();
        options.create_if_missing(true);
        let db = Arc::new(DB::open_default(path).unwrap());
        TarasNabad { db }
    }
}


impl TarasNabadImpl for TarasNabad {
    fn list(&self) {
        unimplemented!()
    }

    fn put(&mut self, value: String) -> Uuid {
        let uuid = Uuid::new_v4();
        self.db.put(uuid.as_bytes(), value.as_bytes()).expect("put");
        return uuid;
    }

    fn get(&self, key: &Uuid) -> Result<Option<Vec<u8>>, Error> {
        return self.db.get(key.as_bytes());
    }

    // fn delete(&self, key: u64) {
    //     self.db.delete(key);
    // }
}
