use super::{
    basic::DbTable,
    item::{DbTableItem, TableId},
};

pub struct Table<T: DbTableItem> {
    items: Vec<T>,
}

impl<T: DbTableItem> Default for Table<T> {
    fn default() -> Self {
        Self {
            items: Default::default(),
        }
    }
}

impl<T: DbTableItem> Table<T> {
    pub fn new() -> Self {
        Self {
            items: Vec::default(),
        }
    }
}

impl<T: DbTableItem> Table<T> {
    /// True if this is a new item, false if it was updated
    pub fn push_one(&mut self, new_item: T) -> bool {
        match self
            .items
            .iter_mut()
            .find(|i| i.get_id() == new_item.get_id())
        {
            Some(i) => {
                *i = new_item;
                false
            }
            None => {
                self.items.push(new_item);
                true
            }
        }
    }
    /// Returns removed item (if found)
    pub fn remove_one(&mut self, id: TableId) -> Option<T> {
        self.items
            .iter()
            .position(|i| i.get_id() == id)
            .map(|ind| self.items.remove(ind))
    }
    pub fn replace_all(&mut self, new_data: Vec<T>) {
        self.items = new_data;
    }
}

impl<T: DbTableItem> DbTable<T> for Table<T> {
    fn get(&self) -> &Vec<T> {
        &self.items
    }

    fn get_mut(&mut self) -> &mut Vec<T> {
        &mut self.items
    }
}
