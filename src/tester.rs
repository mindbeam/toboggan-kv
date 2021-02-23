use crate::{Toboggan, Tree};

pub struct Tester<S: Toboggan>(pub S);
impl<S: Toboggan> Tester<S> {
    pub fn test(&self) {
        let foo = self.0.open_tree("foo").unwrap();
        foo.insert("meow", "cat").unwrap();
        foo.insert("woof", "dog").unwrap();

        // Argh comparisons are hard with trait bounds
        // let items: Vec<_> = foo.iter().map(|r| r.unwrap()).collect();
        // assert_eq!(items, [(b"meow", b"cat"), (b"woof", b"dog")]);
        // assert_eq!(items, &[(&b"meow"[..], &b"cat"[..]), (&b"woof"[..], &b"dog"[..])][..]);

        let mut iter = foo.iter();

        // TODO 4 - fix trait bounds so we don't need to do such rediculous wrangling
        let item = iter.next().unwrap().unwrap();
        assert_eq!(
            (&item.0 as &[u8], &item.1 as &[u8]),
            (&b"meow"[..], &b"cat"[..])
        );

        let item = iter.next().unwrap().unwrap();
        assert_eq!(
            (&item.0 as &[u8], &item.1 as &[u8]),
            (&b"woof"[..], &b"dog"[..])
        );

        assert!(iter.next().is_none());

        // Merge
        foo.set_merge_operator(concat_merge);
        foo.merge("woof", "pup").unwrap();
        assert_eq!(
            foo.get("woof").expect("result").expect("option").as_ref(),
            b"dogpup"
        );

        //Overwrite
        foo.insert("woof", "dawg").unwrap();
        assert_eq!(
            foo.get("woof").expect("result").expect("option").as_ref(),
            b"dawg"
        );

        // Clear
        foo.clear().expect("clear succeeded");
        assert!(foo.iter().next().is_none());
    }
}

fn concat_merge(_key: &[u8], last_value: Option<&[u8]>, merge_value: &[u8]) -> Option<Vec<u8>> {
    match last_value {
        Some(v) => {
            let mut new_value = v.to_vec();
            new_value.extend(merge_value.iter());
            Some(new_value)
        }
        None => Some(merge_value.to_vec()),
    }
}
