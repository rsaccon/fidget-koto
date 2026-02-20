use super::{
    KotoCircle, KotoDifference, KotoIntersection, KotoInverse, KotoMove, KotoScale, KotoSphere,
    KotoTree, KotoUnion,
};
use fidget::context::Tree;
use koto::runtime::KObject;

pub(crate) fn maybe_tree(obj: &KObject) -> Option<Tree> {
    if obj.is_a::<KotoTree>() {
        let k_tree = obj.cast::<KotoTree>();
        Some(k_tree.unwrap().inner())
    } else if obj.is_a::<KotoCircle>() {
        let k_tree = obj.cast::<KotoCircle>();
        Some(Tree::from(k_tree.unwrap().inner()))
    } else if obj.is_a::<KotoSphere>() {
        let k_tree = obj.cast::<KotoSphere>();
        Some(Tree::from(k_tree.unwrap().inner()))
    } else if obj.is_a::<KotoUnion>() {
        let k_tree = obj.cast::<KotoUnion>();
        Some(Tree::from(k_tree.unwrap().inner()))
    } else if obj.is_a::<KotoIntersection>() {
        let k_tree = obj.cast::<KotoIntersection>();
        Some(Tree::from(k_tree.unwrap().inner()))
    } else if obj.is_a::<KotoDifference>() {
        let k_tree = obj.cast::<KotoDifference>();
        Some(Tree::from(k_tree.unwrap().inner()))
    } else if obj.is_a::<KotoInverse>() {
        let k_tree = obj.cast::<KotoInverse>();
        Some(Tree::from(k_tree.unwrap().inner()))
    } else if obj.is_a::<KotoMove>() {
        let k_tree = obj.cast::<KotoMove>();
        Some(Tree::from(k_tree.unwrap().inner()))
    } else if obj.is_a::<KotoScale>() {
        let k_tree = obj.cast::<KotoScale>();
        Some(Tree::from(k_tree.unwrap().inner()))
    } else {
        None
    }
}
