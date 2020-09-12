#![allow(unused_imports)]

use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
struct Pair<K, V> {
    key: K,
    value: V,
}

mod key_from_field {
    use super::*;

    #[test]
    fn by_cloning() {
        let mut map = HashMap::new();

        {
            let foo = Pair {
                key: String::from("foo"),
                value: 42,
            };

            map.insert(foo.key.clone(), foo);
        }

        assert_eq!(map["foo"].value, 42);
    }

    #[test]
    fn by_not_owning_either() {
        let mut map = HashMap::new();

        let foo = Pair {
            key: String::from("foo"),
            value: 42,
        };

        map.insert(&foo.key[..], &foo);

        assert_eq!(map["foo"].value, 42);
    }

    #[test]
    fn by_using_rc() {
        let mut map = HashMap::new();

        {
            let foo = Pair {
                key: Rc::new(String::from("foo")),
                value: 42,
            };

            map.insert(Rc::clone(&foo.key), foo);
        }

        assert_eq!(map[&String::from("foo")].value, 42);

        // check the RC counts before and after removing from the map
        let key = Rc::clone(map.keys().nth(0).unwrap());
        assert_eq!(Rc::strong_count(&key), 3); // local key + map key and value.key
        map.remove(&key);
        assert_eq!(Rc::strong_count(&key), 1); // only local key
    }
}
