use drop_bomb::DropBomb;

use crate::{parser::Event, syntax::SyntaxKind};

use super::Parser;

pub(crate) struct Marker {
    position: usize,
    bomb: DropBomb,
}

impl Marker {
    pub fn new(position: usize) -> Self {
        Self {
            position,
            bomb: DropBomb::new("Markers need to be completed"),
        }
    }

    pub fn complete(mut self, p: &mut Parser, kind: SyntaxKind) -> CompletedMarker {
        let event = &mut p.events[self.position];
        assert_eq!(*event, Event::Placeholder);

        *event = Event::StartNode {
            kind,
            forward_parent: None,
        };
        p.events.push(Event::FinishNode);

        self.bomb.defuse();

        CompletedMarker {
            position: self.position,
        }
    }
}

pub(crate) struct CompletedMarker {
    position: usize,
}

impl CompletedMarker {
    pub fn precede(self, p: &mut Parser) -> Marker {
        let new_marker = p.start();

        if let Event::StartNode {
            ref mut forward_parent,
            ..
        } = p.events[self.position]
        {
            *forward_parent = Some(new_marker.position - self.position);
        } else {
            unreachable!();
        }

        new_marker
    }
}
