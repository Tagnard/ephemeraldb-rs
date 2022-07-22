use crate::{Database, Entry};

#[derive(Debug, Clone, Entry, PartialEq)]
pub struct ItemOne {
    pub id: u32,
}

impl ItemOne {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Entry, PartialEq)]
pub struct ItemTwo {
    pub id: u32,
}

impl ItemTwo {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}

#[test]
fn count_inserted_items() {
    let mut db = Database::default();

    db.insert(ItemOne::new(1));
    db.insert(ItemOne::new(2));
    db.insert(ItemTwo::new(1));

    assert_eq!(db.count::<ItemOne, _>(|_| true), 2);
    assert_eq!(db.count::<ItemTwo, _>(|_| true), 1);
}

#[test]
fn get_inserted_items() {
    let mut db = Database::default();

    db.insert(ItemOne::new(1));
    db.insert(ItemTwo::new(1));
    db.insert(ItemTwo::new(2));

    assert_eq!(
        db.get::<ItemOne, _>(|i| i.id == 1).unwrap()[0],
        ItemOne::new(1)
    );
    assert_eq!(
        db.get::<ItemTwo, _>(|i| i.id == 1).unwrap()[0],
        ItemTwo::new(1)
    );
    assert_eq!(
        db.get::<ItemTwo, _>(|i| i.id == 2).unwrap()[0],
        ItemTwo::new(2)
    );
}

#[test]
fn verify_counters() {
    let mut db = Database::default();

    db.insert(ItemOne::new(1));
    db.insert(ItemTwo::new(1));
    db.insert(ItemOne::new(2));

    assert_eq!(db.counter::<ItemOne>(), 2);
    assert_eq!(db.counter::<ItemTwo>(), 1);
}
