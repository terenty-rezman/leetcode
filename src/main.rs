// mod excercism;
// mod list;
// mod my_list;
mod binary_tree;

fn main() {
    // excercism::test()
    let root = binary_tree::from_bfs_array(&[Some(1), None, Some(3)]);
    dbg!(binary_tree::print_level_order(&root));
}
