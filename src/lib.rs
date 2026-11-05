#[derive(Debug, Clone)]
pub struct DeckTheme { pub name: String, pub bg_color: u32 }

#[derive(Debug)]
pub struct Deck { pub title: String, pub theme: DeckTheme, slides: Vec<String> }
impl Deck {
    pub fn new(title: impl Into<String>, theme: DeckTheme) -> Self {
        Self { title: title.into(), theme, slides: Vec::new() }
    }
    pub fn add_slide(&mut self, s: impl Into<String>) { self.slides.push(s.into()); }
    pub fn count(&self) -> usize { self.slides.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_deck() {
        let mut d = Deck::new("Intro", DeckTheme { name: "Dark".into(), bg_color: 0x111111 });
        d.add_slide("Slide 1");
        assert_eq!(d.count(), 1);
    }
}
