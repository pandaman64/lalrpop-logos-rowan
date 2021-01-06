// auto-generated: "lalrpop 0.19.3"
// sha3: eb6c83d415136938874211c84fdb665f19f9709a94b3de756b79401f3545df
use super::{
    lex,
    syntax::{Language, SyntaxKind},
};
use rowan::{GreenNode, GreenToken, Language as _, NodeOrToken};
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Expr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use super::super::{lex, syntax::{Language, SyntaxKind}};
    use rowan::{Language as _, GreenNode, GreenToken, NodeOrToken};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<'source>
     {
        Variant0(&'source str),
        Variant1(NodeOrToken<GreenNode, GreenToken>),
    }
    const __ACTION: &[i8] = &[
        // State 0
        2, 0, 0, 0, 0, 0, 11,
        // State 1
        2, 0, 0, 0, 0, 0, 11,
        // State 2
        2, 0, 0, 0, 0, 0, 11,
        // State 3
        2, 0, 0, 0, 0, 0, 11,
        // State 4
        2, 0, 0, 0, 0, 0, 11,
        // State 5
        2, 0, 0, 0, 0, 0, 11,
        // State 6
        0, 0, 0, 3, 4, 0, 0,
        // State 7
        0, -3, 5, -3, -3, 6, 0,
        // State 8
        0, -8, -8, -8, -8, -8, 0,
        // State 9
        0, -6, -6, -6, -6, -6, 0,
        // State 10
        0, -7, -7, -7, -7, -7, 0,
        // State 11
        0, 17, 0, 3, 4, 0, 0,
        // State 12
        0, -1, 5, -1, -1, 6, 0,
        // State 13
        0, -2, 5, -2, -2, 6, 0,
        // State 14
        0, -4, -4, -4, -4, -4, 0,
        // State 15
        0, -5, -5, -5, -5, -5, 0,
        // State 16
        0, -9, -9, -9, -9, -9, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 7 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        0,
        // State 2
        0,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        -10,
        // State 7
        -3,
        // State 8
        -8,
        // State 9
        -6,
        // State 10
        -7,
        // State 11
        0,
        // State 12
        -1,
        // State 13
        -2,
        // State 14
        -4,
        // State 15
        -5,
        // State 16
        -9,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            0 => match state {
                1 => 11,
                _ => 6,
            },
            1 => match state {
                2 => 12,
                3 => 13,
                _ => 7,
            },
            2 => 8,
            3 => match state {
                4 => 14,
                5 => 15,
                _ => 9,
            },
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> Vec<::std::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###""-""###,
            r###""/""###,
            r###""Number""###,
        ];
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct __StateMachine<'source>
    where 
    {
        __phantom: ::std::marker::PhantomData<(&'source ())>,
    }
    impl<'source> __state_machine::ParserDefinition for __StateMachine<'source>
    where 
    {
        type Location = usize;
        type Error = lex::NoError;
        type Token = lex::Token<'source>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'source>;
        type Success = NodeOrToken<GreenNode, GreenToken>;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, ::std::marker::PhantomData::<(&())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 7 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, ::std::marker::PhantomData::<(&())>)
        }

        fn expected_tokens(&self, state: i8) -> Vec<String> {
            __expected_tokens(state)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut Vec<i8>,
            symbols: &mut Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                action,
                start_location,
                states,
                symbols,
                ::std::marker::PhantomData::<(&())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            panic!("error recovery not enabled for this grammar")
        }
    }
    fn __token_to_integer<
        'source,
    >(
        __token: &lex::Token<'source>,
        _: ::std::marker::PhantomData<(&'source ())>,
    ) -> Option<usize>
    {
        match *__token {
            lex::Token { kind: SyntaxKind::ParenOpen, text: _ } if true => Some(0),
            lex::Token { kind: SyntaxKind::ParenClose, text: _ } if true => Some(1),
            lex::Token { kind: SyntaxKind::Mult, text: _ } if true => Some(2),
            lex::Token { kind: SyntaxKind::Plus, text: _ } if true => Some(3),
            lex::Token { kind: SyntaxKind::Minus, text: _ } if true => Some(4),
            lex::Token { kind: SyntaxKind::Div, text: _ } if true => Some(5),
            lex::Token { kind: SyntaxKind::Number, text: _ } if true => Some(6),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'source,
    >(
        __token_index: usize,
        __token: lex::Token<'source>,
        _: ::std::marker::PhantomData<(&'source ())>,
    ) -> __Symbol<'source>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 => match __token {
                lex::Token { kind: SyntaxKind::ParenOpen, text: __tok0 } | lex::Token { kind: SyntaxKind::ParenClose, text: __tok0 } | lex::Token { kind: SyntaxKind::Mult, text: __tok0 } | lex::Token { kind: SyntaxKind::Plus, text: __tok0 } | lex::Token { kind: SyntaxKind::Minus, text: __tok0 } | lex::Token { kind: SyntaxKind::Div, text: __tok0 } | lex::Token { kind: SyntaxKind::Number, text: __tok0 } if true => __Symbol::Variant0(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct ExprParser {
        _priv: (),
    }

    impl ExprParser {
        pub fn new() -> ExprParser {
            ExprParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'source,
            __TOKEN: __ToTriple<'source, >,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            __tokens0: __TOKENS,
        ) -> Result<NodeOrToken<GreenNode, GreenToken>, __lalrpop_util::ParseError<usize, lex::Token<'source>, lex::NoError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    __phantom: ::std::marker::PhantomData::<(&())>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<
        'source,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'source>,usize)>,
        _: ::std::marker::PhantomData<(&'source ())>,
    ) -> Option<Result<NodeOrToken<GreenNode, GreenToken>,__lalrpop_util::ParseError<usize, lex::Token<'source>, lex::NoError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(__lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            1 => {
                __reduce1(__lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            2 => {
                __reduce2(__lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            3 => {
                __reduce3(__lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            4 => {
                __reduce4(__lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            5 => {
                __reduce5(__lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            6 => {
                __reduce6(__lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            7 => {
                __reduce7(__lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            8 => {
                __reduce8(__lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            9 => {
                // __Expr = Expr => ActionFn(0);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(__sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant1<
      'source,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'source>,usize)>
    ) -> (usize, NodeOrToken<GreenNode, GreenToken>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'source,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'source>,usize)>
    ) -> (usize, &'source str, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
        'source,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'source>,usize)>,
        _: ::std::marker::PhantomData<(&'source ())>,
    ) -> (usize, usize)
    {
        // Expr = Expr, "+", Factor => ActionFn(1);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action1::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 0)
    }
    pub(crate) fn __reduce1<
        'source,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'source>,usize)>,
        _: ::std::marker::PhantomData<(&'source ())>,
    ) -> (usize, usize)
    {
        // Expr = Expr, "-", Factor => ActionFn(2);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action2::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 0)
    }
    pub(crate) fn __reduce2<
        'source,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'source>,usize)>,
        _: ::std::marker::PhantomData<(&'source ())>,
    ) -> (usize, usize)
    {
        // Expr = Factor => ActionFn(3);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 0)
    }
    pub(crate) fn __reduce3<
        'source,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'source>,usize)>,
        _: ::std::marker::PhantomData<(&'source ())>,
    ) -> (usize, usize)
    {
        // Factor = Factor, "*", Term => ActionFn(4);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action4::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 1)
    }
    pub(crate) fn __reduce4<
        'source,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'source>,usize)>,
        _: ::std::marker::PhantomData<(&'source ())>,
    ) -> (usize, usize)
    {
        // Factor = Factor, "/", Term => ActionFn(5);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action5::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 1)
    }
    pub(crate) fn __reduce5<
        'source,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'source>,usize)>,
        _: ::std::marker::PhantomData<(&'source ())>,
    ) -> (usize, usize)
    {
        // Factor = Term => ActionFn(6);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce6<
        'source,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'source>,usize)>,
        _: ::std::marker::PhantomData<(&'source ())>,
    ) -> (usize, usize)
    {
        // Num = "Number" => ActionFn(9);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 2)
    }
    pub(crate) fn __reduce7<
        'source,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'source>,usize)>,
        _: ::std::marker::PhantomData<(&'source ())>,
    ) -> (usize, usize)
    {
        // Term = Num => ActionFn(7);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 3)
    }
    pub(crate) fn __reduce8<
        'source,
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'source>,usize)>,
        _: ::std::marker::PhantomData<(&'source ())>,
    ) -> (usize, usize)
    {
        // Term = "(", Expr, ")" => ActionFn(8);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action8::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 3)
    }
}
pub use self::__parse__Expr::ExprParser;

fn __action0<'source>(
    (_, __0, _): (usize, NodeOrToken<GreenNode, GreenToken>, usize),
) -> NodeOrToken<GreenNode, GreenToken> {
    __0
}

fn __action1<'source>(
    (_, l, _): (usize, NodeOrToken<GreenNode, GreenToken>, usize),
    (_, _, _): (usize, &'source str, usize),
    (_, r, _): (usize, NodeOrToken<GreenNode, GreenToken>, usize),
) -> NodeOrToken<GreenNode, GreenToken> {
    GreenNode::new(Language::kind_to_raw(SyntaxKind::BinOp), vec![l, r]).into()
}

fn __action2<'source>(
    (_, l, _): (usize, NodeOrToken<GreenNode, GreenToken>, usize),
    (_, _, _): (usize, &'source str, usize),
    (_, r, _): (usize, NodeOrToken<GreenNode, GreenToken>, usize),
) -> NodeOrToken<GreenNode, GreenToken> {
    GreenNode::new(Language::kind_to_raw(SyntaxKind::BinOp), vec![l, r]).into()
}

fn __action3<'source>(
    (_, __0, _): (usize, NodeOrToken<GreenNode, GreenToken>, usize),
) -> NodeOrToken<GreenNode, GreenToken> {
    __0
}

fn __action4<'source>(
    (_, l, _): (usize, NodeOrToken<GreenNode, GreenToken>, usize),
    (_, _, _): (usize, &'source str, usize),
    (_, r, _): (usize, NodeOrToken<GreenNode, GreenToken>, usize),
) -> NodeOrToken<GreenNode, GreenToken> {
    GreenNode::new(Language::kind_to_raw(SyntaxKind::BinOp), vec![l, r]).into()
}

fn __action5<'source>(
    (_, l, _): (usize, NodeOrToken<GreenNode, GreenToken>, usize),
    (_, _, _): (usize, &'source str, usize),
    (_, r, _): (usize, NodeOrToken<GreenNode, GreenToken>, usize),
) -> NodeOrToken<GreenNode, GreenToken> {
    GreenNode::new(Language::kind_to_raw(SyntaxKind::BinOp), vec![l, r]).into()
}

fn __action6<'source>(
    (_, __0, _): (usize, NodeOrToken<GreenNode, GreenToken>, usize),
) -> NodeOrToken<GreenNode, GreenToken> {
    __0
}

fn __action7<'source>(
    (_, __0, _): (usize, NodeOrToken<GreenNode, GreenToken>, usize),
) -> NodeOrToken<GreenNode, GreenToken> {
    __0
}

fn __action8<'source>(
    (_, _, _): (usize, &'source str, usize),
    (_, __0, _): (usize, NodeOrToken<GreenNode, GreenToken>, usize),
    (_, _, _): (usize, &'source str, usize),
) -> NodeOrToken<GreenNode, GreenToken> {
    __0
}

fn __action9<'source>(
    (_, text, _): (usize, &'source str, usize),
) -> NodeOrToken<GreenNode, GreenToken> {
    lex::Token {
        kind: SyntaxKind::Number,
        text,
    }
    .into()
}

pub trait __ToTriple<'source> {
    fn to_triple(
        value: Self,
    ) -> Result<
        (usize, lex::Token<'source>, usize),
        __lalrpop_util::ParseError<usize, lex::Token<'source>, lex::NoError>,
    >;
}

impl<'source> __ToTriple<'source> for (usize, lex::Token<'source>, usize) {
    fn to_triple(
        value: Self,
    ) -> Result<
        (usize, lex::Token<'source>, usize),
        __lalrpop_util::ParseError<usize, lex::Token<'source>, lex::NoError>,
    > {
        Ok(value)
    }
}
impl<'source> __ToTriple<'source> for Result<(usize, lex::Token<'source>, usize), lex::NoError> {
    fn to_triple(
        value: Self,
    ) -> Result<
        (usize, lex::Token<'source>, usize),
        __lalrpop_util::ParseError<usize, lex::Token<'source>, lex::NoError>,
    > {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
