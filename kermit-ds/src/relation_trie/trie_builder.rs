use {
    crate::{relation_builder::RelationBuilder, relation_trie::trie::RelationTrie},
    std::{fmt::Debug, str::FromStr},
};

pub struct TrieBuilder<KT: PartialOrd + PartialEq + Clone + FromStr + Debug> {
    cardinality: usize,
    tuples: Vec<Vec<KT>>,
}

impl<KT: PartialOrd + PartialEq + Clone + FromStr + Debug> RelationBuilder<KT, RelationTrie<KT>>
    for TrieBuilder<KT>
{
    fn new(cardinality: usize) -> Self {
        TrieBuilder {
            cardinality,
            tuples: vec![],
        }
    }

    fn build(self) -> RelationTrie<KT> {
        RelationTrie::from_mut_tuples(self.cardinality, self.tuples)
    }

    fn add_tuple(mut self, tuple: Vec<KT>) -> TrieBuilder<KT> {
        self.tuples.push(tuple);
        self
    }

    fn add_tuples(mut self, tuples: Vec<Vec<KT>>) -> TrieBuilder<KT> {
        self.tuples.extend(tuples);
        self
    }
}

#[cfg(test)]
mod tests {

    // use crate::ds::relation_trie::{node::TrieFields,
    // trie_builder::TrieBuilder};
    //
    // Read from file
    // #[test]
    // fn trie_builder_read_from_file() {
    // let trie = TrieBuilder::<String>::new(3)
    // .from_file::<&str>("test.csv")
    // .unwrap()
    // .build();
    // assert_eq!(trie.children()[0].key(), "1");
    // assert_eq!(trie.children()[1].key(), "3");
    // assert_eq!(trie.children()[0].children()[0].key(), "3");
    // assert_eq!(trie.children()[0].children()[1].key(), "4");
    // assert_eq!(trie.children()[0].children()[2].key(), "5");
    // assert_eq!(trie.children()[1].children()[0].key(), "5");
    // assert_eq!(trie.children()[0].children()[0].children()[0].key(), "4");
    // assert_eq!(trie.children()[0].children()[0].children()[1].key(), "5");
    // assert_eq!(trie.children()[0].children()[1].children()[0].key(), "6");
    // assert_eq!(trie.children()[0].children()[1].children()[1].key(), "8");
    // assert_eq!(trie.children()[0].children()[1].children()[2].key(), "9");
    // assert_eq!(trie.children()[0].children()[2].children()[0].key(), "2");
    // assert_eq!(trie.children()[1].children()[0].children()[0].key(), "2");
    // }
}
