//! A minimal random (version 4) UUID generator.
//!
//! ```
//! let id = uuid_lite2::Uuid::new_v4();
//! println!("{id}");
//! ```

use core::fmt;

/// A 128-bit UUID.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Uuid([u8; 16]);

impl Uuid {
    /// Generates a random (version 4) UUID.
    pub fn new_v4() -> Self {
        let mut bytes = [0u8; 16];
        getrandom::fill(&mut bytes).expect("failed to gather random bytes");

        // Set the version and variant bits per RFC 9562.
        bytes[6] = (bytes[6] & 0x0f) | 0x40; // version = 4
        bytes[8] = (bytes[8] & 0x3f) | 0x80; // variant = RFC 4122

        Uuid(bytes)
    }

    /// Returns the underlying 16 bytes.
    pub fn as_bytes(&self) -> &[u8; 16] {
        &self.0
    }
}

impl fmt::Display for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let b = &self.0;
        write!(
            f,
            "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7],
            b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15],
        )
    }
}

impl fmt::Debug for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::Uuid;

    #[test]
    fn version_and_variant_bits() {
        let bytes = *Uuid::new_v4().as_bytes();
        assert_eq!(bytes[6] & 0xf0, 0x40, "version must be 4");
        assert_eq!(bytes[8] & 0xc0, 0x80, "variant must be RFC 4122");
    }

    #[test]
    fn display_format() {
        let s = Uuid::new_v4().to_string();
        assert_eq!(s.len(), 36);
        assert_eq!(s.chars().filter(|&c| c == '-').count(), 4);
        // The 15th character (the version nibble) is always '4'.
        assert_eq!(s.as_bytes()[14], b'4');
    }

    #[test]
    fn is_random() {
        assert_ne!(Uuid::new_v4(), Uuid::new_v4());
    }
}
