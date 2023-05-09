/* --------------------------------------------------------------------------------------

 * Author: Zatarita
 * Last Edit: Zatarita (5/9/2023)
 * 
 * OBJ Parsing & Line Data
 * 
 * ------------------------------------------------------------------------------------*/

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::VecDeque;

#[derive(Debug)]
pub enum ParserError {
}

// Represents a single parsed line from an obj file
#[derive(Debug)]
pub struct ObjLine {
    pub keyword:    Option<String>,     // Keywords define the function of the line. Can be empty if comment
    pub parameters: VecDeque<String>,   // Parameter(s) when available supply additional information for keywords
    pub comments:   Option<String>      // Comments can appear at any point in the file.
}

impl ObjLine {
    #[allow(unused_assignments)]
    pub fn from(line: &String) -> ObjLine {
        let mut keyword: Option<String> = None;
        let mut parameters: VecDeque<String> = VecDeque::<String>::new();
        let mut comments: Option<String> = None;

        if let Some( (data, comment) ) = line.trim_end().split_once("#") {
            let mut line_elements: VecDeque<String> = data.split_ascii_whitespace().map(|s| s.to_owned() ).collect();
            
            if !comment.is_empty() {
                comments = Some(comment.to_owned());
            }
            keyword = line_elements.pop_front();
            parameters = line_elements;

        } else {
            let mut line_elements: VecDeque<String> = line.split_ascii_whitespace().map(|s| s.to_owned() ).collect();

            keyword = line_elements.pop_front();
            parameters = line_elements;
        }

        ObjLine { keyword, parameters, comments }
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
    pub fn get_line(&mut self) -> Option<String> {
        let mut buf: String = String::new();
        let bytes_read = self.reader.read_line(&mut buf).ok()?;

        if bytes_read == 0 {
            return None;
        }

        // Lines can be split using "\". If this is the case, the next line is a part of the current line.
        // We need to read the next line, and concat them together.
        while buf.trim_end().ends_with('\\') {
            let mut next_line_buf: String = String::new();      
            self.reader.read_line(&mut next_line_buf).ok()?;    // Read the next line

            buf = format!("{}{next_line_buf}", buf.trim_end().trim_end_matches("\\"));
        }

        Some(buf)
    }
}

impl Iterator for ObjParser {
    type Item = ObjLine;

    // Read the next line from the stream, parse it, and return the data
    fn next(&mut self) -> Option<Self::Item> {
        let line = self.get_line()?;                    
        let parsed_line = ObjLine::from(&line);
        Some(parsed_line)                                       
    }
}