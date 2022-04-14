use std::mem;

use rowan::{GreenNodeBuilder, Language};

use crate::{
    lexer::Token,
    syntax::{Fql, SyntaxKind},
};

use super::{Event, Parse, ParseError};

pub(super) struct Sink<'t, 'input> {
    builder: GreenNodeBuilder<'static>,
    tokens: &'t [Token<'input>],
    cursor: usize,
    events: Vec<Event>,
    errors: Vec<ParseError>,
}

impl<'t, 'input> Sink<'t, 'input> {
    pub(super) fn new(tokens: &'t [Token<'input>], events: Vec<Event>) -> Self {
        Self {
            builder: GreenNodeBuilder::new(),
            tokens,
            events,
            cursor: 0,
            errors: vec![],
        }
    }

    pub(super) fn finish(mut self) -> Parse {
        for idx in 0..self.events.len() {
            match mem::replace(&mut self.events[idx], Event::Placeholder) {
                Event::StartNode {
                    kind,
                    forward_parent,
                } => {
                    if forward_parent.is_none() {
                        self.builder.start_node(Fql::kind_to_raw(kind));
                    } else {
                        // Recursively follow the forward_parent fields until they run out,
                        // building a stack of syntax kinds to start.

                        let mut kinds = vec![kind];
                        let mut idx = idx;
                        let mut forward_parent = forward_parent;

                        while let Some(fp) = forward_parent {
                            idx += fp;
                            forward_parent = if let Event::StartNode {
                                kind,
                                forward_parent,
                            } =
                                mem::replace(&mut self.events[idx], Event::Placeholder)
                            {
                                kinds.push(kind);
                                forward_parent
                            } else {
                                panic!(
                                    "forward_parent referenced an event that wasn't a StartNode"
                                );
                            };
                        }

                        for kind in kinds.into_iter().rev() {
                            self.builder.start_node(Fql::kind_to_raw(kind));
                        }
                    }
                }
                Event::AddToken => self.token(),
                Event::FinishNode => self.builder.finish_node(),
                Event::Error(error) => {
                    self.errors.push(error);
                }
                Event::Placeholder => {}
            }
        }

        Parse {
            green_node: self.builder.finish(),
            errors: self.errors,
        }
    }

    fn token(&mut self) {
        let Token { kind, text, .. } = self.tokens[self.cursor];
        self.cursor += 1;
        self.builder
            .token(Fql::kind_to_raw(SyntaxKind::from(kind)), text.into());
    }
}
