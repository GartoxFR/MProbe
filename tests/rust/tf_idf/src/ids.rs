use std::borrow::Borrow;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct IdMap {
    next_id: u32,
    inner: HashMap<String, u32>,
}

impl IdMap {
    pub fn get(&self, word: &str) -> Option<u32> {
        self.inner.get(word).copied()
    }

    pub fn register_word(&mut self, word: impl Into<String>) -> u32 {
        self.inner.insert(word.into(), self.next_id);
        let return_id = self.next_id;
        self.next_id += 1;
        return_id
    }

    pub fn get_or_register(&mut self, word: impl Borrow<str> + Into<String>) -> u32 {
        match self.get(word.borrow()) {
            Some(id) => id,
            None => self.register_word(word),
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::IdMap;

    #[test] 
    fn test_id_map() {
        let mut map = IdMap::default();
        let id = map.get_or_register("hello");
        assert!(id == 0);

        let id = map.get_or_register("hello".to_string());
        assert!(id == 0);

        let id = map.get("toto");
        assert!(matches!(id, None));

        let id = map.get_or_register("toto");
        assert!(id == 1);
    }
}
