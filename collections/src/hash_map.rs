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

mod fallible_insert {
    use super::*;

    #[allow(dead_code)]
    fn fallible(key: &str) -> Result<i32, &'static str> {
        if key == "foo" {
            Ok(42)
        } else {
            Err("forbidden key")
        }
    }

    #[test]
    fn foo() {
        fn get_or_insert<'a>(map: &'a mut HashMap<String, i32>, key: String) -> Result<&'a i32, &'static str> {
            use std::collections::hash_map::Entry::*;

            Ok(match map.entry(key) {
                Occupied(e) => e.into_mut(),
                Vacant(e) => {
                    let value = fallible(e.key())?;
                    e.insert(value)
                }
            })
        }

        let mut map = HashMap::new();

        assert_eq!(get_or_insert(&mut map, String::from("foo")), Ok(&42));
        assert_eq!(get_or_insert(&mut map, String::from("bar")), Err("forbidden key"));
    }
}
