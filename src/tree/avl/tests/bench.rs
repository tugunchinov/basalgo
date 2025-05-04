// use super::*;
// use test::Bencher;
//
// #[bench]
// fn bench_insert(b: &mut Bencher) {
//     let mut tree = AVLTree::new();
//     let mut i = 0;
//
//     b.iter(|| {
//         tree.insert(i, i);
//         i += 1;
//     });
// }
//
// #[bench]
// fn bench_get(b: &mut Bencher) {
//     let tree: AVLTree<_, _> = (0..1000).map(|i| (i, i)).collect();
//     let mut i = 0;
//
//     b.iter(|| {
//         tree.get(&(i % 1000));
//         i += 1;
//     });
// }
