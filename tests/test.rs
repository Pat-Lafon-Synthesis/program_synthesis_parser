use std::{fs::File, io::Read};

use program_synthesis_parser;

macro_rules! make_test {
    ($test_name:tt) => {
        #[test]
        fn $test_name() {
            let parser = program_synthesis_parser::spec::ProblemParser::new();
            let mut file =
                File::open(format!("tests/benchmarks/{}.mls", stringify!($test_name))).unwrap();
            let mut buffer = String::new();
            file.read_to_string(&mut buffer).unwrap();
            insta::assert_debug_snapshot!(parser.parse(&buffer).unwrap());
        }
    };
}

make_test!(bool_always_false);
make_test!(bool_always_true);
make_test!(bool_band);
make_test!(bool_bor);
make_test!(bool_impl);
make_test!(bool_neg);


make_test!(expr_boolean);
make_test!(expr_div);
make_test!(expr_sub);
make_test!(expr);


make_test!(list_append);
make_test!(list_compress);
make_test!(list_concat);
make_test!(list_drop);
make_test!(list_dropeven);
make_test!(list_even_parity);
make_test!(list_filter);
make_test!(list_fold);
make_test!(list_hd);
make_test!(list_inc);
make_test!(list_last);
make_test!(list_last2);
make_test!(list_length);
make_test!(list_make);
make_test!(list_map);
make_test!(list_nth);
make_test!(list_pairwise_swap);
make_test!(list_range);
make_test!(list_rev_append);
make_test!(list_rev_fold);
make_test!(list_rev_snoc);
make_test!(list_rev_tailcall);
make_test!(list_snoc);
make_test!(list_sort_sorted_insert);
make_test!(list_sorted_insert);
make_test!(list_stutter);
make_test!(list_sum);
make_test!(list_take);
make_test!(list_tl);


make_test!(nat_add);
make_test!(nat_iseven);
make_test!(nat_max);
make_test!(nat_mul);
make_test!(nat_pred);
make_test!(nat_sub);


make_test!(tree_balanced);
make_test!(tree_binsert);
make_test!(tree_collect_leaves);
make_test!(tree_count_leaves);
make_test!(tree_count_nodes);
make_test!(tree_height);
make_test!(tree_inorder);
make_test!(tree_lastleft);
make_test!(tree_map);
make_test!(tree_nodes_at_level);
make_test!(tree_notexist);
make_test!(tree_postorder);
make_test!(tree_preorder);
make_test!(tree_sum);