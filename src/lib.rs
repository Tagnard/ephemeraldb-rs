use std::{
    any::{type_name, Any},
    collections::HashMap,
    sync::{Arc, Mutex},
};
use uuid::Uuid;

#[cfg(test)]
mod tests;

pub trait Entry {
    fn get_id(&self) -> Uuid;
}

pub use derive_macro::Entry;

type Table = HashMap<Uuid, Box<dyn 'static + Sync + Send + Any>>;

#[derive(Clone)]
pub struct Database {
    db: Arc<Mutex<HashMap<String, Table>>>,
    pub counters: Arc<Mutex<HashMap<String, u32>>>,
}

impl Database {
    fn new_table<I: Any + Entry>(&mut self) {
        self.db.lock().unwrap().insert(
            type_name::<I>().to_string(),
            HashMap::<Uuid, Box<dyn 'static + Sync + Send + Any>>::new(),
        );

        self.counters
            .lock()
            .unwrap()
            .insert(type_name::<I>().to_string(), 0);
    }

    fn table_exists<I: Any + Entry>(&self) -> bool {
        self.db.lock().unwrap().get(type_name::<I>()).is_some()
    }

    pub fn insert<T: 'static + Sync + Send + Any + Entry + Clone>(&mut self, mut value: T) -> T {
        if !self.table_exists::<T>() {
            self.new_table::<T>();
        }

        let mut table = self.db.lock().unwrap();
        let table = table.get_mut(type_name::<T>()).unwrap();

        table.insert(
            value.get_id(),
            Box::new(value.clone()) as Box<dyn 'static + Sync + Send + Any>,
        );

        value
    }

    pub fn get_by_id<T: 'static + Clone>(&self, id: Uuid) -> Option<T>
    where
        T: Clone,
    {
        let table = self.db.lock().unwrap();
        let table = table.get(&type_name::<T>().to_string()).unwrap();

        if let Some(val) = table.get(&id) {
            let t = val.downcast_ref::<T>().unwrap();
            Some(t.clone())
        } else {
            None
        }
    }

    pub fn get_all<T: 'static + Clone>(&self) -> Vec<T>
    where
        T: Clone,
    {
        let table = self.db.lock().unwrap();
        let table = table.get(&type_name::<T>().to_string()).unwrap();

        table
            .iter()
            .map(|(_, v)| v.downcast_ref::<T>().unwrap().clone())
            .collect::<Vec<T>>()
    }

    pub fn get<T: 'static + Entry, P: FnMut(&T) -> bool>(&self, predicate: P) -> Option<Vec<T>>
    where
        T: Clone,
    {
        let table = self.db.lock().unwrap();
        let table = table.get(&type_name::<T>().to_string()).unwrap();

        let t = table
            .iter()
            .map(|(_, v)| v.downcast_ref::<T>().unwrap().clone())
            .filter(predicate)
            .collect::<Vec<T>>();

        if !t.is_empty() {
            Some(t)
        } else {
            None
        }
    }

    pub fn count<T: 'static + Entry, P: FnMut(&&T) -> bool>(&self, predicate: P) -> u32 {
        let table = self.db.lock().unwrap();
        let table = table.get(&type_name::<T>().to_string()).unwrap();

        table
            .iter()
            .map(|(_, v)| v.downcast_ref::<T>().unwrap())
            .filter(predicate)
            .count() as u32
    }

    pub fn counter<T: 'static + Entry>(&self) -> u32 {
        let table = self.counters.lock().unwrap();
        *table.get(&type_name::<T>().to_string()).unwrap()
    }

    pub fn counter_inc<T: 'static + Entry>(&self) -> u32 {
        let mut table = self.counters.lock().unwrap();
        *table.get_mut(&type_name::<T>().to_string()).unwrap() += 1;
        *table.get(&type_name::<T>().to_string()).unwrap()
    }
}

impl Default for Database {
    fn default() -> Self {
        Self {
            db: Arc::new(Mutex::new(HashMap::new())),
            counters: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
