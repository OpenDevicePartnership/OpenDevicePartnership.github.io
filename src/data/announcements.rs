//! Announcement metadata.
//!
//! Each [`Announcement`] is one entry in the blog-style index at
//! `/announcements` and one permalink page at
//! `/announcements/<slug>`. Only the *metadata* lives here; the
//! detail prose stays co-located with the page that renders it
//! (`src/pages/announcements.rs::render_content`).

use std::cmp::Ordering;

/// What kind of announcement this is. Drives the eyebrow label on
/// both the index card and the detail page.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AnnouncementKind {
    PressRelease,
    // Future: ReleaseNotes, Community, BlogPost, etc.
}

impl AnnouncementKind {
    /// Display label for the eyebrow.
    pub const fn label(self) -> &'static str {
        match self {
            AnnouncementKind::PressRelease => "Press release",
        }
    }
}

/// A minimal calendar date. We roll our own to avoid pulling
/// `chrono` (and its locale/time-zone surface) into the wasm bundle
/// just to print "October 7, 2025".
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Date {
    pub year: u16,
    /// 1..=12
    pub month: u8,
    /// 1..=31
    pub day: u8,
}

impl Date {
    pub const fn new(year: u16, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }

    /// ISO-8601 form, suitable for `<time datetime=...>`.
    pub fn iso(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }

    /// Long human form, e.g. `"October 7, 2025"`.
    pub fn long(&self) -> String {
        format!("{} {}, {}", month_name(self.month), self.day, self.year)
    }
}

impl Ord for Date {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.year, self.month, self.day).cmp(&(other.year, other.month, other.day))
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn month_name(m: u8) -> &'static str {
    match m {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "Unknown",
    }
}

/// One announcement entry.
#[derive(Clone, Copy)]
pub struct Announcement {
    /// URL slug used in `/announcements/<slug>`. Must be unique and
    /// stable — permalinks rely on it.
    pub slug: &'static str,
    /// Headline shown on the index card and as the H1 on the detail
    /// page.
    pub title: &'static str,
    /// Kind of announcement (drives the eyebrow label).
    pub kind: AnnouncementKind,
    /// Publication date — sortable, displayed under the title and as
    /// the dateline on the detail page.
    pub published_at: Date,
    /// One-to-two-sentence summary shown on the index card. Should
    /// stand alone without the body and stay under ~280 chars so the
    /// card layout doesn't get top-heavy.
    pub excerpt: &'static str,
    /// Optional dateline location, e.g. `"Redmond, WA"`.
    pub location: Option<&'static str>,
}

/// Canonical announcement list. **Must stay in reverse-chronological
/// order** (newest first); the `entries_are_in_reverse_chronological_order`
/// test enforces this so the index renders correctly without runtime
/// sorting.
pub const ANNOUNCEMENTS: &[Announcement] = &[Announcement {
    slug: "welcome-patina-announcement",
    title: "Patina Project to Launch at UEFI 2025 Developer Conference & Plugfest",
    kind: AnnouncementKind::PressRelease,
    published_at: Date::new(2025, 10, 7),
    excerpt: "The Open Device Partnership is announcing Patina, a new open-source, \
        Rust-based UEFI-compatible firmware designed for memory safety and to address \
        long-standing challenges in the PC firmware ecosystem.",
    location: Some("Redmond, WA"),
}];

/// Look up an announcement by slug. Returns `None` if no announcement
/// with that slug exists.
pub fn find(slug: &str) -> Option<&'static Announcement> {
    ANNOUNCEMENTS.iter().find(|a| a.slug == slug)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn welcome_patina_is_first() {
        assert_eq!(ANNOUNCEMENTS[0].slug, "welcome-patina-announcement");
        assert!(find("welcome-patina-announcement").is_some());
    }

    #[test]
    fn unknown_slug_returns_none() {
        assert!(find("does-not-exist").is_none());
    }

    #[test]
    fn slugs_are_unique() {
        let mut slugs: Vec<&str> = ANNOUNCEMENTS.iter().map(|a| a.slug).collect();
        slugs.sort();
        let len_before = slugs.len();
        slugs.dedup();
        assert_eq!(slugs.len(), len_before, "announcement slugs must be unique");
    }

    #[test]
    fn entries_are_in_reverse_chronological_order() {
        // Newest first; each entry must be strictly newer than the one
        // after it.
        for window in ANNOUNCEMENTS.windows(2) {
            let (newer, older) = (&window[0], &window[1]);
            assert!(
                newer.published_at >= older.published_at,
                "{} ({}) appears before {} ({}); list must be reverse-chronological",
                newer.slug,
                newer.published_at.iso(),
                older.slug,
                older.published_at.iso(),
            );
        }
    }

    #[test]
    fn excerpts_are_within_reasonable_length() {
        // Soft cap so cards don't grow taller than the headline+CTA
        // structure they're framed in. If you need more room, edit
        // the cap deliberately rather than letting a long excerpt
        // slip through.
        const MAX: usize = 320;
        for a in ANNOUNCEMENTS {
            assert!(
                a.excerpt.len() <= MAX,
                "announcement '{}' excerpt is {} chars (max {})",
                a.slug,
                a.excerpt.len(),
                MAX,
            );
        }
    }

    #[test]
    fn date_long_form_renders() {
        let d = Date::new(2025, 10, 7);
        assert_eq!(d.long(), "October 7, 2025");
        assert_eq!(d.iso(), "2025-10-07");
    }
}
