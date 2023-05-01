use std::collections::HashSet;

#[derive(Copy, Clone)]
enum LineNumbers {
    OmitBlank,
    Default,
    Off,
}

impl LineNumbers {
    fn find(flags: &HashSet<String>) -> Self {
        if flags.contains(&"-b".to_string()) {
            return LineNumbers::OmitBlank;
        } else if flags.contains(&"-n".to_string()) {
            return LineNumbers::Default;
        } else {
            return LineNumbers::Off;
        }
    }
}

#[derive(Copy, Clone)]
enum NonPrintChars {
    EOL,
    TabsAndFormFeeds,
    Both,
    Default,
    Off,
}

impl NonPrintChars {
    fn find(flags: &HashSet<String>) -> Self {
        if flags.contains(&"-e".to_string()) && flags.contains(&"-t".to_string()) {
            return NonPrintChars::Both;
        } else if flags.contains(&"-e".to_string()) {
            return NonPrintChars::EOL;
        } else if flags.contains(&"-t".to_string()) {
            return NonPrintChars::TabsAndFormFeeds;
        } else if flags.contains(&"-v".to_string()) {
            return NonPrintChars::Default;
        } else {
            return NonPrintChars::Off;
        }
    }
}

pub struct FlaggedString {
    line_numbers: LineNumbers,
    non_print_chars: NonPrintChars,
    fold_empty_lines: bool,
    string: String,
}

impl FlaggedString {
    pub fn new(string: String, flags: &HashSet<String>) -> Self {
        Self {
            line_numbers: LineNumbers::find(&flags),
            non_print_chars: NonPrintChars::find(&flags),
            fold_empty_lines: flags.contains(&"-r".to_string()),
            string: string.chars().filter(|x| !(x == &'\r')).collect(),
        }
    }

    pub fn make_string(&self) -> String {
        let folded_string = match self.fold_empty_lines {
            true => {
                let mut consecutive_newlines = 0;
                let mut _folded_string = String::new();

                for character in self.string.chars() {
                    match character {
                        '\n' => consecutive_newlines += 1,
                        _ => consecutive_newlines = 0,
                    }

                    if consecutive_newlines <= 2 {
                        _folded_string.push(character);
                    }
                }

                _folded_string
            }

            false => self.string.clone(),
        };

        let numbered_string = match self.line_numbers {
            LineNumbers::Off => folded_string,
            LineNumbers::Default => {
                let line_count = folded_string.lines().count();
                let spaces = line_count.to_string().len() + 2;

                let mut _numbered_string = String::new();

                for (i, line) in folded_string.lines().enumerate() {
                    _numbered_string.push_str(
                        &("    ".to_string()
                            + &(i + 1).to_string()
                            + &" ".repeat(spaces - i.to_string().len())
                            + &line
                            + &"\n".to_string()),
                    );
                }
                _numbered_string
            }
            LineNumbers::OmitBlank => {
                let line_count = folded_string.lines().count();
                let spaces = line_count.to_string().len() + 2;

                let mut _numbered_string = String::new();
                let mut i = 0;
                for line in folded_string.lines() {
                    if line != "".to_string() {
                        _numbered_string.push_str(
                            &("    ".to_string()
                                + &(i + 1).to_string()
                                + &" ".repeat(spaces - i.to_string().len())
                                + &line
                                + &"\n".to_string()),
                        );
                        i += 1;
                    } else {
                        _numbered_string.push_str(&(line.to_string() + &"\n".to_string()));
                    }
                }
                _numbered_string
            }
        };

        let nonprint_string = match self.non_print_chars {
            NonPrintChars::Off | NonPrintChars::Default => numbered_string,
            NonPrintChars::EOL => numbered_string
                .chars()
                .map(|x| match x {
                    '\n' => "$".to_string() + &x.to_string(),
                    _ => x.to_string(),
                })
                .collect(),
            NonPrintChars::TabsAndFormFeeds => numbered_string
                .chars()
                .map(|x| match x {
                    '\x09' => "^I".to_string(),
                    '\x0C' => "^L".to_string(),
                    _ => x.to_string(),
                })
                .collect(),
            NonPrintChars::Both => numbered_string
                .chars()
                .map(|x| match x {
                    '\n' => "$".to_string() + &x.to_string(),
                    '\x09' => "^I".to_string(),
                    '\x0C' => "^L".to_string(),
                    _ => x.to_string(),
                })
                .collect(),
        };

        nonprint_string
    }
}
