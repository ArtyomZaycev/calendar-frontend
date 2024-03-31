pub use calendar_lib::api::utils::TableId;

pub trait DbTableItem {
    fn get_id(&self) -> TableId;
}

pub trait DbTableUpdateItem {
    fn get_id(&self) -> TableId;
}

pub trait DbTableNewItem {}
