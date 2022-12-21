use std::collections::HashSet;

const WINDOW_SIZE: usize = 4;

pub(crate) struct Stream {
    pub(crate) window_size: usize,
    position: usize,
    content: String,
}

impl From<&str> for Stream {
    fn from(s: &str) -> Self {
        Self {
            content: s.to_string(),
            position: 0,
            window_size: WINDOW_SIZE,
        }
    }
}

impl Iterator for Stream {
    type Item = Window;

    fn next(&mut self) -> Option<Self::Item> {
        let content = self
            .content
            .chars()
            .skip(self.position)
            .take(self.window_size)
            .collect::<Vec<char>>();
        if content.len() < self.window_size {
            return None;
        }
        let result = Some(Window {
            content,
            index: self.position,
        });
        self.position += 1;
        result
    }
}

pub(crate) struct Window {
    index: usize,
    content: Vec<char>,
}

impl Window {
    pub(crate) fn marker_position(&self) -> Option<usize> {
        let unique: HashSet<char> = HashSet::from_iter(self.content.iter().cloned());
        if unique.len() == self.content.len() {
            return Some(self.index + self.content.len());
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn marker_position() {
        let mut stream: Stream = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".into();
        assert_eq!(Some(7), stream.find_map(|w| w.marker_position()));
        let mut stream: Stream = "bvwbjplbgvbhsrlpgdmjqwftvncz".into();
        assert_eq!(Some(5), stream.find_map(|w| w.marker_position()));
        let mut stream: Stream = "nppdvjthqldpwncqszvftbrmjlhg".into();
        assert_eq!(Some(6), stream.find_map(|w| w.marker_position()));
        let mut stream: Stream = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".into();
        assert_eq!(Some(10), stream.find_map(|w| w.marker_position()));
        let mut stream: Stream = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".into();
        assert_eq!(Some(11), stream.find_map(|w| w.marker_position()));
    }
}
