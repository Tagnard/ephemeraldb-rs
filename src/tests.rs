use uuid::Uuid;

use crate::{Database, Entry};

#[derive(Debug, Clone, Entry, PartialEq)]
pub struct ItemOne {
    pub id: Uuid,
}

impl ItemOne {
    pub fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }
}

#[derive(Debug, Clone, Entry, PartialEq)]
pub struct ItemTwo {
    pub id: Uuid,
}

impl ItemTwo {
    pub fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }
}

#[test]
fn count_inserted_items() {
    let mut db = Database::default();

    db.insert(ItemOne::new());
    db.insert(ItemOne::new());
    db.insert(ItemTwo::new());

    assert_eq!(db.count::<ItemOne, _>(|_| true), 2);
    assert_eq!(db.count::<ItemTwo, _>(|_| true), 1);
}

#[test]
fn get_inserted_items() {
    let mut db = Database::default();

    let one = db.insert(ItemOne::new());
    let two = db.insert(ItemTwo::new());
    let three = db.insert(ItemTwo::new());

    assert_eq!(db.get::<ItemOne, _>(|i| i.id == one.id).unwrap()[0], one);
    assert_eq!(db.get::<ItemTwo, _>(|i| i.id == two.id).unwrap()[0], two);
    assert_eq!(
        db.get::<ItemTwo, _>(|i| i.id == three.id).unwrap()[0],
        three
    );
}

#[test]
fn verify_counters() {
    let mut db = Database::default();

    db.insert(ItemOne::new());
    db.insert(ItemTwo::new());
    db.insert(ItemOne::new());

    assert_eq!(db.counter::<ItemOne>(), 2);
    assert_eq!(db.counter::<ItemTwo>(), 1);
}
