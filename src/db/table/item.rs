pub use calendar_lib::api::utils::TableId;
use serde::{de::DeserializeOwned, Serialize};

pub trait DbTableItem
where
    Self: 'static + DeserializeOwned + Serialize + Send,
{
    fn get_id(&self) -> TableId;
}

pub trait DbTableUpdateItem
where
    Self: 'static + DeserializeOwned + Serialize + Send,
{
    fn get_id(&self) -> TableId;
}

pub trait DbTableNewItem
where
    Self: 'static + DeserializeOwned + Serialize + Send,
{
}
