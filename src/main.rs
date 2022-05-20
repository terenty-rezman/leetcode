// mod excercism;
mod binary_tree;
mod binary_tree_serialize;
mod bst_insert;
mod inorder_iterator;
mod list;
mod list_design;
mod my_list;
mod reverse_list;

struct X {
    x: i32,
}

fn main() {
    // excercism::test()

    // let root = binary_tree::from_bfs_array(&[Some(1), None, Some(3)]);
    // dbg!(binary_tree::print_level_order(&root));

    // binary_tree_serialize::tests::test();

    // inorder_iterator::tests::test();

    // bst_insert::tests::test();

    // list_design::tests::test();

    reverse_list::tests::test();
}
