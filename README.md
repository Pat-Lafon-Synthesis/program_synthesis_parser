# program_synthesis_parser

Parser for io benchmark tests like used in Brust/Smyth/Trio

Based on/stolen from <https://github.com/pslhy/trio_artifacts/tree/master/burst/src>

Note that `tests/benchmarks/{tree_balanced, expr_div, expr_sub}.mls` has been modified to put parenthesis around the inner match statements for parsing. I'm blaming this on lax ambiguity resolution in Menhir.

You will probably want to `cargo install cargo-insta` if you want to run the test suite.

I use my fork of the hashconsing crate at the moment.
