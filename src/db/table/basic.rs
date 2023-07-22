use super::item::*;

pub trait DbTable<T: DbTableItem, Container = Vec<T>> {
    fn get(&self) -> &Container;
    fn get_mut(&mut self) -> &mut Container;
}

pub trait DbTableGetById<T: DbTableItem, Container = Vec<T>> {
    fn get_by_id(&self, id: T::Id) -> Option<&T>;
    fn get_by_id_mut(&mut self, id: T::Id) -> Option<&mut T>;
}

impl<Item: DbTableItem, Table> DbTableGetById<Item, Vec<Item>> for Table
where
    Table: DbTable<Item, Vec<Item>>,
{
    fn get_by_id(&self, id: <Item as DbTableItem>::Id) -> Option<&Item> {
        self.get().iter().find(|i| i.get_id() == id)
    }

    fn get_by_id_mut(&mut self, id: <Item as DbTableItem>::Id) -> Option<&mut Item> {
        self.get_mut().iter_mut().find(|i| i.get_id() == id)
    }
}
