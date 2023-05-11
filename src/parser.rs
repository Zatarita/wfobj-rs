/* --------------------------------------------------------------------------------------

 * Author: Zatarita
 * Last Edit: Zatarita (5/11/2023)
 * 
 * OBJ Parsing & Line Data
 * 
 * ------------------------------------------------------------------------------------*/

use std::fs::File;
use std::io::{BufReader, BufRead, Seek, SeekFrom};
use std::collections::VecDeque;

#[derive(Debug)]
pub enum ParserError {
}

// Represents a single parsed line from an obj file
#[derive(Debug)]
pub struct ObjLine {
    pub keyword:    Option<String>,     // Keywords define the function of the line. Can be empty if comment
    pub parameters: VecDeque<String>,   // Parameter(s) when available supply additional information for keywords
    pub comment:   Option<String>       // Comments can appear at any point in the file.
}

impl ObjLine {
    #[allow(unused_assignments)]
    pub fn from(line: &String) -> ObjLine {
        let mut keyword:    Option<String>   = None;
        let mut parameters: VecDeque<String> = VecDeque::<String>::new();
        let mut comment:    Option<String>   = None;

        if let Some( (data, line_comment) ) = line.trim_end().split_once("#") {
            let mut line_elements: VecDeque<String> = data.split_ascii_whitespace().map(|s| s.to_owned() ).collect();
            
            if !line_comment.is_empty() {
                comment = Some(line_comment.to_owned());
            }
            keyword = line_elements.pop_front();
            parameters = line_elements;

        } else {
            let mut line_elements: VecDeque<String> = line.split_ascii_whitespace().map(|s| s.to_owned() ).collect();

            keyword = line_elements.pop_front();
            parameters = line_elements;
        }

        ObjLine { keyword, parameters, comment }
    }
}

pub struct ObjParser {
    reader: BufReader<File>
}

impl ObjParser {
    // Loads a file to be parsed.
    pub fn new(path: &str) -> std::io::Result<ObjParser> {
        Ok(ObjParser { reader: BufReader::new(File::open(path)?) })
    }

    // Gets the next line from the file accounting for potential line breaks.
    fn read_string(&mut self) -> Option<String> {
        let mut buf:    String = String::new();
        let bytes_read: usize  = self.reader.read_line(&mut buf).ok()?;

        // Check EOF
        if bytes_read == 0 {
            return None;
        }

        // Lines can be split using "\". If this is the case, the next line is a part of the current line.
        // We need to read the next line, and concat them together.
        while buf.trim_end().ends_with('\\') {
            let next_line = self.read_string()?;

            buf = format!("{}{next_line}", buf.trim_end().trim_end_matches("\\"));
        }

        Some(buf)
    }

    // Parse a line read from the stream. Progresses the stream
    pub fn get_line(&mut self) -> Option<ObjLine> {
        let line: String = self.read_string()?;                    
        let parsed_line: ObjLine = ObjLine::from(&line);
        Some(parsed_line)                              
    }

    // Ignore a line
    pub fn skip_line(&mut self) -> () {
        self.get_line();
    }

    // Parse a line read from stream. Resets stream position
    pub fn peek_line(&mut self) -> Option<ObjLine> {
        let current_pos: u64 = self.reader.seek(SeekFrom::Current(0)).ok()?;
        let line: Option<ObjLine> = self.get_line();
        let _ = self.reader.seek(SeekFrom::Start(current_pos));
        line                            
    }

    // Get the next keyword from stream, resets stream position
    pub fn get_next_keyword(&mut self) -> Option<String> {
        let line: ObjLine = self.peek_line()?;
        line.keyword
    }
}

impl Iterator for ObjParser {
    type Item = ObjLine;

    fn next(&mut self) -> Option<Self::Item> {
        self.get_line()                                 
    }
}