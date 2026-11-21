//! SigmaOS-craft-deck: deck/presentation model.
//!
//! Real implementation: slide model with content, layout, timing; transition effects;
//! slide ordering and navigation; export/render options; validation. Replaces the
//! 22-line stub.

/// Deck/presentation module.
pub mod sigmadeck;

use std::collections::BTreeMap;
use std::fmt;

/// Slide layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlideLayout {
    Title,
    Content,
    TwoColumn,
    Image,
    Code,
}

/// Transition effect.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Transition {
    None,
    Fade,
    SlideLeft,
    SlideRight,
    Zoom,
}

/// Slide.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Slide {
    pub id: String,
    pub title: String,
    pub content: String,
    pub layout: SlideLayout,
    pub transition: Transition,
    pub duration_ms: Option<u32>,
}

/// Deck theme.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeckTheme {
    pub name: String,
    pub bg_color: u32,
    pub text_color: u32,
    pub font: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeckError {
    EmptyTitle,
    DuplicateSlide(String),
    SlideNotFound(String),
    InvalidDuration(u32),
}

impl fmt::Display for DeckError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyTitle => write!(f, "deck title must not be empty"),
            Self::DuplicateSlide(id) => write!(f, "duplicate slide {:?}", id),
            Self::SlideNotFound(id) => write!(f, "slide {:?} not found", id),
            Self::InvalidDuration(d) => write!(f, "invalid duration {}ms", d),
        }
    }
}

impl std::error::Error for DeckError {}

/// Deck/presentation.
pub struct Deck {
    pub title: String,
    pub theme: DeckTheme,
    slides: BTreeMap<String, Slide>,
    slide_order: Vec<String>,
    current_index: usize,
}

impl Deck {
    pub fn new(title: impl Into<String>, theme: DeckTheme) -> Result<Self, DeckError> {
        let title = title.into();
        if title.trim().is_empty() { return Err(DeckError::EmptyTitle); }
        Ok(Self { title, theme, slides: BTreeMap::new(), slide_order: Vec::new(), current_index: 0 })
    }

    pub fn add_slide(&mut self, slide: Slide) -> Result<(), DeckError> {
        let id = slide.id.clone();
        if self.slides.contains_key(&id) {
            return Err(DeckError::DuplicateSlide(id));
        }
        self.slide_order.push(id.clone());
        self.slides.insert(id, slide);
        Ok(())
    }

    pub fn remove_slide(&mut self, id: &str) -> Result<(), DeckError> {
        if !self.slides.contains_key(id) {
            return Err(DeckError::SlideNotFound(id.into()));
        }
        self.slides.remove(id);
        self.slide_order.retain(|x| x != id);
        if self.current_index >= self.slide_order.len() && !self.slide_order.is_empty() {
            self.current_index = self.slide_order.len() - 1;
        }
        Ok(())
    }

    pub fn current_slide(&self) -> Option<&Slide> {
        self.slide_order.get(self.current_index).and_then(|id| self.slides.get(id))
    }

    pub fn next_slide(&mut self) -> Option<&Slide> {
        if self.current_index + 1 < self.slide_order.len() {
            self.current_index += 1;
            self.current_slide()
        } else { None }
    }

    pub fn prev_slide(&mut self) -> Option<&Slide> {
        if self.current_index > 0 {
            self.current_index -= 1;
            self.current_slide()
        } else { None }
    }

    pub fn slide_count(&self) -> usize { self.slides.len() }

    pub fn current_index(&self) -> usize { self.current_index }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deck_create_aaa() {
        let theme = DeckTheme { name: "Dark".into(), bg_color: 0x111111, text_color: 0xffffff, font: "sans".into() };
        let d = Deck::new("Intro", theme).unwrap();
        assert_eq!(d.slide_count(), 0);
    }

    #[test]
    fn test_empty_title_rejected_aaa() {
        let theme = DeckTheme { name: "Dark".into(), bg_color: 0x111111, text_color: 0xffffff, font: "sans".into() };
        assert!(matches!(Deck::new("", theme), Err(DeckError::EmptyTitle)));
    }

    #[test]
    fn test_add_and_navigate_aaa() {
        let theme = DeckTheme { name: "Dark".into(), bg_color: 0x111111, text_color: 0xffffff, font: "sans".into() };
        let mut d = Deck::new("Intro", theme).unwrap();
        let s1 = Slide { id: "s1".into(), title: "First".into(), content: "Hello".into(),
            layout: SlideLayout::Title, transition: Transition::Fade, duration_ms: None };
        let s2 = Slide { id: "s2".into(), title: "Second".into(), content: "World".into(),
            layout: SlideLayout::Content, transition: Transition::None, duration_ms: None };
        d.add_slide(s1).unwrap();
        d.add_slide(s2).unwrap();
        assert_eq!(d.slide_count(), 2);
        assert_eq!(d.current_slide().unwrap().id, "s1");
        d.next_slide();
        assert_eq!(d.current_slide().unwrap().id, "s2");
    }

    #[test]
    fn test_duplicate_slide_rejected_aaa() {
        let theme = DeckTheme { name: "Dark".into(), bg_color: 0x111111, text_color: 0xffffff, font: "sans".into() };
        let mut d = Deck::new("Intro", theme).unwrap();
        let s = Slide { id: "x".into(), title: "X".into(), content: "".into(),
            layout: SlideLayout::Title, transition: Transition::None, duration_ms: None };
        d.add_slide(s.clone()).unwrap();
        let s2 = Slide { id: "x".into(), title: "X".into(), content: "".into(),
            layout: SlideLayout::Title, transition: Transition::None, duration_ms: None };
        assert!(matches!(d.add_slide(s2), Err(DeckError::DuplicateSlide(_))));
    }
}
