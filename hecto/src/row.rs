use crate::highlighting;
use crate::SearchDirection;
use std::cmp;
use termion::color;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
pub struct Row {
    string: String,
    highlighting: Vec<highlighting::Type>,
    len: usize,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        // Self {
        //    string: String::from(slice),
        // }
        // let mut row = Self {
        //     string: String::from(slice),
        //     len: 0,
        // };
        // row.update_len();
        // row 
        Self {
            string: String::from(slice),
            highlighting: Vec::new(),
            len: slice.graphemes(true).count(),
        }
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        // self.string.get(start..end).unwrap_or_default().to_string()        
        let mut result = String::new();
        let mut current_highlighting = &highlighting::Type::None;
        #[allow(clippy::integer_arithmetic)]
        for (index, grapheme) in self.string[..]
            .graphemes(true)
                .enumerate()
                .skip(start)
                .take(end - start)
                {
                    //result.push_str(grapheme);
                   // if grapheme == "\t" {
                   //     result.push_str(" ");
                   // } else {
                   //     result.push_str(grapheme);
                   // }
                   if let Some(c) = grapheme.chars().next() {
                       let highlighting_type = self
                           .highlighting
                           .get(index)
                           .unwrap_or(&highlighting::Type::None);
                      // let start_highlight =
                      //     format!("{}", termion::color::Fg(highlighting_type.to_color()));
                      // result.push_str(&start_highlight[..]);
                       if highlighting_type != current_highlighting {
                            current_highlighting = highlighting_type;
                            let start_highlight =
                                format!("{}", termion::color::Fg(highlighting_type.to_color()));            
                            result.push_str(&start_highlight[..]);
                       }
                       if c == '\t' {
                            result.push_str(" ");
                       // } else if c.is_ascii_digit() {
                       //     result.push_str(
                       //         &format!(
                       //             "{}{}{}",
                       //             termion::color::Fg(color::Rgb(220, 163, 163)),
                       //             c,
                       //             color::Fg(color::Reset)
                       //             )[..],
                       //         );
                        } else {
                            result.push(c);
                        }
                       // let end_highlight = format!("{}", termion::color::Fg(color::Reset));
                       // result.push_str(&end_highlight[..]);
                   }
                }
        let end_highlight = format!("{}", termion::color::Fg(color::Reset));
        result.push_str(&end_highlight[..]);
        result
    }

    pub fn len(&self) -> usize {
        // self.string.len()
        // self.string[..].graphemes(true).count()
        self.len
    }

    pub fn is_empty(&self) -> bool {
        // self.string.is_empty()
        self.len == 0
    }

    fn update_len(&mut self) {
        self.len = self.string[..].graphemes(true).count();
    }

    pub fn insert(&mut self, at: usize, c: char) {
        if at >= self.len() {
            self.string.push(c);
            // } else {
            //     let mut result: String = self.string[..].graphemes(true).take(at).collect();
            //     let remainder: String = self.string[..].graphemes(true).skip(at).collect();
            //     result.push(c);
            //     result.push_str(&remainder);
            //     self.string = result;
            // }
            // self.update_len();
        self.len += 1;
        return;
    }
    let mut result: String = String::new();
    let mut length = 0;
    for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
        length += 1;
        if index == at {
            length += 1;
            result.push(c);
        }
        result.push_str(grapheme);
    }
    self.len = length;
    self.string = result;

}

// #[allow(clippy::integer_arithmetic)]
pub fn delete(&mut self, at: usize) {
    if at >= self.len() {
        return;
    }
    // } else {
    //     let mut result: String = self.string[..].graphemes(true).take(at).collect();
    //     let remainder: String = self.string[..].graphemes(true).skip(at + 1).collect();
    //     result.push_str(&remainder);
    //     self.string = result;
    // }
    // self.update_len();
let mut result: String = String::new();
let mut length = 0;
for(index, grapheme) in self.string[..].graphemes(true).enumerate() {
    if index != at {
        length += 1;
        result.push_str(grapheme);
    }
}

self.len = length;
self.string = result;
}

pub fn append(&mut self, new: &Self) {
    self.string = format!("{}{}", self.string, new.string);
    // self.update_len();
    self.len += new.len;
}

pub fn split(&mut self, at: usize) -> Self {
    // let beginning: String = self.string[..].graphemes(true).take(at).collect();
    // let remainder: String = self.string[..].graphemes(true).skip(at).collect();
    // self.string = beginning;
    // self.update_len();
    // Self::from(&remainder[..])
    let mut row: String = String::new();
    let mut length = 0;
    let mut splitted_row: String = String::new();
    let mut splitted_length = 0;
    for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
        if index < at {
            length += 1;
            row.push_str(grapheme);
        } else {
            splitted_length += 1;
            splitted_row.push_str(grapheme);
        }
    }

    self.string = row;
    self.len = length;
    Self {
        string: splitted_row,
        len: splitted_length,
        highlighting: Vec::new(),
    }
}

pub fn as_bytes(&self) -> &[u8] {
    self.string.as_bytes()
}

pub fn find(&self, query: &str, at: usize, direction: SearchDirection) -> Option<usize> {
    if at > self.len {
        return None;
    }
    let start = if direction == SearchDirection::Forward {
        at
    } else {
        0
    };
    let end = if direction == SearchDirection::Forward {
        self.len
    } else {
        at
    };

    #[allow(clippy::integer_arithmetic)]
    let substring: String = self.string[..]
        .graphemes(true)
        .skip(start)
        .take(end - start)
        .collect();
    // let matching_byte_index = substring.find(query);
    // if let Some(matching)byte_index) = matching_byte_index {
    //     for (grapheme_index, (byte_index, _)) in
    //         substring[..].grapheme_indices(true).enumerate()
    //         {
    //             if matching_byte_index == byte_index {
    //                 #[allow(clippy::integer_arithmetic)]
    //                 return Some(after + grapheme_index);
    //             }
    //         }
    // }
    let matching_byte_index = if direction == SearchDirection::Forward {
        substring.find(query)
    } else {
        substring.rfind(query)
    };
    if let Some(matching_byte_index) = matching_byte_index {
        for (grapheme_index, (byte_index, _)) in
            substring[..].grapheme_indices(true).enumerate()
            {
                if matching_byte_index == byte_index {
                    #[allow(clippy::integer_arithmetic)]
                    return Some(start + grapheme_index);
                }
            }
    }
    None
}

// pub fn find(&self, query: &str) -> Option<usize> {
//     let matching_byte_index = self.string.find(query);
//     if let Some(matching_byte_index) = matching_byte_index {
//         for (grapheme_index, (byte_index, _)) in
//             self.string[..].grapheme_indices(true).enumerate()
//         {
//             if matching_byte_index == byte_index {
//                 return Some(grapheme_index);
//             }
//         }
//     }
//     None
// }
    pub fn highlight(&mut self) {
        let mut highlighting = Vec::new();
        for c in self.string.chars() {
            if c.is_ascii_digit() {
                highlighting.push(highlighting::Type::Number);
            } else {
                highlighting.push(highlighting::Type::None);
            }
        }
        self.highlighting = highlighting;
    }
}
