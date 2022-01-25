use crate::priv_prelude::*;

pub struct Program {
    pub kind: ProgramKind,
    pub dependencies: Vec<Dependency>,
}

impl Spanned for Program {
    fn span(&self) -> Span {
        let src = self.kind.span().context();
        let end = src.len();
        Span::new(src, 0, end)
    }
}

pub fn program() -> impl Parser<char, Program, Error = Cheap<char, Span>> + Clone {
    whitespace()
    .or_not()
    .then(program_kind())
    .then_optional_whitespace()
    .then(dependency().then_optional_whitespace().repeated())
    .map(|((_opt, kind), dependencies)| {
        Program { kind, dependencies }
    })
}

pub enum ProgramKind {
    Script {
        script_token: ScriptToken,
        semicolon_token: SemicolonToken,
    },
    Contract {
        contract_token: ContractToken,
        semicolon_token: SemicolonToken,
    },
    Predicate {
        predicate_token: PredicateToken,
        semicolon_token: SemicolonToken,
    },
    Library {
        library_token: LibraryToken,
        name: Ident,
        semicolon_token: SemicolonToken,
    },
}

impl Spanned for ProgramKind {
    fn span(&self) -> Span {
        match self {
            ProgramKind::Script { script_token, semicolon_token } => {
                Span::join(script_token.span(), semicolon_token.span())
            },
            ProgramKind::Contract { contract_token, semicolon_token } => {
                Span::join(contract_token.span(), semicolon_token.span())
            },
            ProgramKind::Predicate { predicate_token, semicolon_token } => {
                Span::join(predicate_token.span(), semicolon_token.span())
            },
            ProgramKind::Library { library_token, semicolon_token, .. } => {
                Span::join(library_token.span(), semicolon_token.span())
            },
        }
    }
}

pub fn program_kind() -> impl Parser<char, ProgramKind, Error = Cheap<char, Span>> + Clone {
    let script = {
        script_token()
        .then_optional_whitespace()
        .then(semicolon_token())
        .map(|(script_token, semicolon_token)| {
            ProgramKind::Script { script_token, semicolon_token }
        })
    };
    let contract = {
        contract_token()
        .then_optional_whitespace()
        .then(semicolon_token())
        .map(|(contract_token, semicolon_token)| {
            ProgramKind::Contract { contract_token, semicolon_token }
        })
    };
    let predicate = {
        predicate_token()
        .then_optional_whitespace()
        .then(semicolon_token())
        .map(|(predicate_token, semicolon_token)| {
            ProgramKind::Predicate { predicate_token, semicolon_token }
        })
    };
    let library = {
        library_token()
        .then_whitespace()
        .then(ident())
        .then_optional_whitespace()
        .then(semicolon_token())
        .map(|((library_token, name), semicolon_token)| {
            ProgramKind::Library { library_token, name, semicolon_token }
        })
    };

    script.or(contract).or(predicate).or(library)
}

