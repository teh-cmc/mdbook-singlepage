use mdbook::{renderer::RenderContext, BookItem};
use std::{fs, fs::File, io, io::prelude::*};

fn main() {
    let mut stdin = io::stdin();
    let ctx = RenderContext::from_json(&mut stdin).unwrap();

    fs::create_dir_all(&ctx.destination).unwrap();
    let mut f = File::create(ctx.destination.join("README.md")).unwrap();

    ctx.book // intro
        .iter()
        .filter_map(|item| match item {
            BookItem::Chapter(ch) if ch.number.is_none() => Some(ch.content.clone()),
            _ => None,
        })
        .take(1)
        .for_each(|intro| f.write_fmt(format_args!("{}", intro)).unwrap());

    writeln!(f, "\n---\n").unwrap();

    ctx.book // index
        .iter()
        .filter_map(|item| match item {
            BookItem::Chapter(ch) if ch.number.is_none() => None,
            BookItem::Chapter(ch) => {
                let number = ch.number.as_ref().unwrap();
                let lvl = number.len();
                let offset = "    ".repeat(lvl - 1);
                if lvl == 1 {
                    // chapter
                    let name = &ch.name[number.to_string().len() + 1..];
                    let anchor = title_to_anchor(&ch.name);
                    format!("{}{} [{}](#{})  \n", offset, number, name, anchor).into()
                } else {
                    // section
                    let anchor = title_to_anchor(&format!("{} {}", number, ch.name));
                    format!("{}{} [{}](#{})  \n", offset, number, ch.name, anchor).into()
                }
            }
            _ => None,
        })
        .for_each(|index| f.write_all(index.as_bytes()).unwrap());

    ctx.book // everything else
        .iter()
        .filter_map(|item| match item {
            BookItem::Chapter(ch) if ch.number.is_some() => {
                (ch.number.as_ref().unwrap().len(), ch.content.clone()).into()
            }
            _ => None,
        })
        .for_each(|(lvl, content)| {
            if lvl == 1 {
                // chapter
                writeln!(f, "\n---\n").unwrap();
            } else {
                // section
                writeln!(f).unwrap();
            }
            f.write_all(content.as_bytes()).unwrap();
        });
}

fn title_to_anchor(title: &str) -> String {
    const PUNCTUATION: [char; 10] = [',', '.', '?', '!', ';', ':', '"', '\'', '&', '/'];
    const DASHES: [char; 1] = [' '];
    title
        .trim()
        .to_lowercase()
        .replace(&DASHES[..], "-")
        .replace(&PUNCTUATION[..], "")
}
