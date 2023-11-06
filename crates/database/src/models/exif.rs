use std::fmt;

pub struct Exif {
    pub path: String,
    pub tag: String,
    pub tag_id: i64,
    pub value: String,
}

impl Default for Exif {
    fn default() -> Self {
        Exif {
            path: String::new(),
            tag: String::new(),
            tag_id: 0,
            value: String::new(),
        }
    }
}

impl Exif {
    pub fn new(path: &str, tag: &str, tag_id: i64, value: &str) -> Exif {
        Exif {
            path: path.to_string(),
            tag: tag.to_string(),
            tag_id,
            value: value.to_string(),
        }
    }
}

impl fmt::Display for Exif {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "path: {}, tag: {}, tag_id: {}, value: {}",
            self.path, self.tag, self.tag_id, self.value
        )
    }
}

impl fmt::Debug for Exif {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "path: {}, tag: {}, tag_id: {}, value: {}",
            self.path, self.tag, self.tag_id, self.value
        )
    }
}

impl PartialEq for Exif {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
            && self.tag == other.tag
            && self.tag_id == other.tag_id
            && self.value == other.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exif_default() {
        let exif = Exif::default();
        assert_eq!(
            exif,
            Exif {
                path: String::new(),
                tag: String::new(),
                tag_id: 0,
                value: String::new(),
            }
        );
    }

    #[test]
    fn test_exif_new() {
        let exif = Exif::new("path", "tag", 1, "value");
        assert_eq!(
            exif,
            Exif {
                path: "path".to_string(),
                tag: "tag".to_string(),
                tag_id: 1,
                value: "value".to_string(),
            }
        );
    }
}
