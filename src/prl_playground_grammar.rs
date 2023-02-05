use crate::prl_playground_grammar_trait::{PrlPlayground, PrlPlaygroundGrammarTrait};
#[allow(unused_imports)]
use parol_runtime::Result;
use std::fmt::{Debug, Display, Error, Formatter};

///
/// Data structure that implements the semantic actions for our PrlPlayground grammar
/// !Change this type as needed!
///
#[derive(Debug, Default)]
pub struct PrlPlaygroundGrammar<'t> {
    pub prl_playground: Option<PrlPlayground<'t>>,
}

impl PrlPlaygroundGrammar<'_> {
    pub fn new() -> Self {
        PrlPlaygroundGrammar::default()
    }
}

impl Display for PrlPlayground<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        write!(f, "{:?}", self)
    }
}

impl Display for PrlPlaygroundGrammar<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        match &self.prl_playground {
            Some(prl_playground) => writeln!(f, "{}", prl_playground),
            None => write!(f, "No parse result"),
        }
    }
}

impl<'t> PrlPlaygroundGrammarTrait<'t> for PrlPlaygroundGrammar<'t> {
    // !Adjust your implementation as needed!

    /// Semantic action for non-terminal 'PrlPlayground'
    fn prl_playground(&mut self, arg: &PrlPlayground<'t>) -> Result<()> {
        self.prl_playground = Some(arg.clone());
        Ok(())
    }
}
