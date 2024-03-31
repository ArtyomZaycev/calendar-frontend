pub type TableId = i32;

pub trait DbTableItem {
    fn get_id(&self) -> TableId;
}

pub trait DbTableUpdateItem {
    fn get_id(&self) -> TableId;
}

pub trait DbTableNewItem {}
