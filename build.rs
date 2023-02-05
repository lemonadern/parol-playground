use parol::build::Builder;

fn main() {
    // CLI equivalent is:
    // parol -f ./prl_playground.par -e ./prl_playground-exp.par -p ./src/prl_playground_parser.rs -a ./src/prl_playground_grammar_trait.rs -t PrlPlaygroundGrammar -m prl_playground_grammar -g
    Builder::with_explicit_output_dir("src")
        .grammar_file("prl_playground.par")
        .expanded_grammar_output_file("../prl_playground-exp.par")
        .parser_output_file("prl_playground_parser.rs")
        .actions_output_file("prl_playground_grammar_trait.rs")
        .enable_auto_generation()
        .user_type_name("PrlPlaygroundGrammar")
        .user_trait_module_name("prl_playground_grammar")
        .generate_parser()
        .unwrap();
}
