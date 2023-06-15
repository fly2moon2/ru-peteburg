use std::hash::Hash;
use std::fmt::{Debug, Formatter, Result};
use petgraph::graph::Graph;
use petgraph::graphmap::UnGraphMap;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Tag {
    pub tag: String,
}

/* impl Debug for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Ta")
         .finish()
    }
} */

/// tagged objects
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Tagged<T> {
    pub tag: Tag,
    pub obj: T,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct TagG<T> {
    pub val: T,
}

/* impl<T> Debug for TagG<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("TaG")
         .finish()
    }
} */

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TaggedG<T> {
    pub tag: Tag,
    pub vals: Vec<Box<T>>,
    
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Pet {
    pub tag: String,
}

#[cfg(test)]
mod tests {
    /// Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn can_gen_graph_ok() { 
        let mut graph = Graph::<(), ()>::new(); // directed and unlabeled

        graph.extend_with_edges(&[ (0, 1) ]);

        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);
    }

    #[test]
    fn can_gen_tag_graph_ok() { 
        let mut graph = UnGraphMap::<_, ()>::new();
        let fav_tag1 = Tag { tag: "crimea".into()};
        let fav_tag2 = Tag { tag: "mariupol".into()};
        let fav_iamtag1 = Tagged{ tag: Tag { tag: "mariupol".into()}, obj:Tag { tag: "crimea".into()}};
        let fav_iamtag2 = Tagged{ tag: Tag { tag: "mariupol2".into()}, obj:Tag { tag: "bakrumet".into()}};
    
  /*       graph.add_node(&fav_tag1);
        graph.add_node(&fav_tag2);
        graph.add_edge(&fav_tag1, &fav_tag2, ()); */

        graph.add_node(&fav_iamtag1);
        graph.add_node(&fav_iamtag2);
        graph.add_edge(&fav_iamtag1, &fav_iamtag2, ());

        //graph.add_node(&fav_iamtag1);

        /// vector to hold generics
        /// https://users.rust-lang.org/t/vector-with-generic-types-heterogeneous-container/46644/3
        let mut dyn_list: Vec<Box<dyn Debug>> = vec![];

        let var_u16 = TagG::<u16>{ val : u16::default() };
        let var_tag = TagG::<Tag>{ val : Tag { tag: "crimea".into()} };

        dyn_list.push(Box::new( var_u16 ));
        dyn_list.push(Box::new( var_tag ));
    
        dyn_list.push(Box::new( "hello" ));

        println!("{:?}", dyn_list);

/*         let mut graphg = UnGraphMap::<_, ()>::new();
        graphg.add_node(&dyn_list); */
        //graphg.add_node(&var_tag);
        

/*         assert!(graph.contains_node(&fav_tag1));
        assert!(graph.contains_node(&fav_tag2));
        assert!(graph.contains_edge(&fav_tag1, &fav_tag2));
        assert!(graph.contains_edge(&fav_tag2, &fav_tag1)); */

        assert!(graph.contains_node(&fav_iamtag1));
        assert!(graph.contains_node(&fav_iamtag2));
        assert!(graph.contains_edge(&fav_iamtag1, &fav_iamtag2));
        assert!(graph.contains_edge(&fav_iamtag2, &fav_iamtag1));
    }

}