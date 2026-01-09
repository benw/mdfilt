use anyhow::Result;
use clap::Parser as ClapParser;
use pulldown_cmark::{Event, HeadingLevel, Parser, Options, Tag, TagEnd, TextMergeStream};

use std::fs;

#[derive(ClapParser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input markdown file
    markdown: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let input = fs::read_to_string(&args.markdown)?;
    let options = Options::ENABLE_TABLES;
    let parser = Parser::new_ext(&input, options);
    let iterator = TextMergeStream::new(parser);
    let mut stack = vec![];
    for event in iterator {
        //println!("\n{:?}", event);
        match event {
            Event::Start(tag) => {
                match tag {
                    Tag::Paragraph => println!(""),
                    Tag::Heading { level, .. } => {
                        print!("\n{} ", match level {
                            HeadingLevel::H1 => "#",
                            HeadingLevel::H2 => "##",
                            HeadingLevel::H3 => "###",
                            HeadingLevel::H4 => "####",
                            HeadingLevel::H5 => "#####",
                            HeadingLevel::H6 => "######",
                        });
                    }
                    Tag::Emphasis => print!("*"),
                    Tag::Strong => print!("**"),
                    _ => ()
                }
                stack.push(tag);
            }
            Event::End(tagend) => {
                stack.pop();
                match tagend {
                    TagEnd::Paragraph | TagEnd::Heading(_) => println!(""),
                    TagEnd::Emphasis => print!("*"),
                    TagEnd::Strong => print!("**"),
                    _ => (),
                }
            }
            Event::Text(text) => {
                match stack.get(0) {
                    Some(Tag::Paragraph) | Some(Tag::Heading { .. }) => print!("{}", text),
                    _ => (),
                }
            }
            Event::Code(text) => {
                match stack.get(0) {
                    Some(Tag::Paragraph) | Some(Tag::Heading { .. }) => print!("`{}`", text),
                    _ => (),
                }
            }
            _ => (),
        }
    }
    println!("\n");
    Ok(())
}
