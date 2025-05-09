use super::{KCircle, KDifference, KIntersection, KInverse, KMove, KScale, KSphere, KTree, KUnion};
use fidget::context::Tree;
use koto::runtime::KObject;

pub(crate) fn maybe_tree(obj: &KObject) -> Option<Tree> {
    if obj.is_a::<KTree>() {
        let k_tree = obj.cast::<KTree>();
        Some(k_tree.unwrap().inner())
    } else if obj.is_a::<KCircle>() {
        let k_tree = obj.cast::<KCircle>();
        Some(Tree::from(k_tree.unwrap().inner()))
    } else if obj.is_a::<KSphere>() {
        let k_tree = obj.cast::<KSphere>();
        Some(Tree::from(k_tree.unwrap().inner()))
    } else if obj.is_a::<KUnion>() {
        let k_tree = obj.cast::<KUnion>();
        Some(Tree::from(k_tree.unwrap().inner()))
    } else if obj.is_a::<KIntersection>() {
        let k_tree = obj.cast::<KIntersection>();
        Some(Tree::from(k_tree.unwrap().inner()))
    } else if obj.is_a::<KDifference>() {
        let k_tree = obj.cast::<KDifference>();
        Some(Tree::from(k_tree.unwrap().inner()))
    } else if obj.is_a::<KInverse>() {
        let k_tree = obj.cast::<KInverse>();
        Some(Tree::from(k_tree.unwrap().inner()))
    } else if obj.is_a::<KMove>() {
        let k_tree = obj.cast::<KMove>();
        Some(Tree::from(k_tree.unwrap().inner()))
    } else if obj.is_a::<KScale>() {
        let k_tree = obj.cast::<KScale>();
        Some(Tree::from(k_tree.unwrap().inner()))
    } else {
        None
    }
}
