use pulldown_cmark::{CodeBlockKind, CowStr, Event, Tag, TagEnd};
use std::io::{ErrorKind, Write};
use std::process::{exit, Command, Stdio};

pub struct HighlightCodeBlocks<'a, I> {
    inner: I,
    _marker: std::marker::PhantomData<Event<'a>>,
}

impl<I> HighlightCodeBlocks<'_, I> {
    pub fn new(inner: I) -> Self {
        Self {
            inner,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<'a, I: Iterator<Item = Event<'a>>> Iterator for HighlightCodeBlocks<'a, I> {
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Event<'a>> {
        let event = self.inner.next()?;

        let Event::Start(Tag::CodeBlock(kind)) = event else {
            return Some(event);
        };

        let language = match &kind {
            CodeBlockKind::Fenced(info) => info.to_string(),
            CodeBlockKind::Indented => String::new(),
        };

        let mut literal = String::new();
        loop {
            match self.inner.next() {
                Some(Event::Text(text)) => literal.push_str(&text),
                Some(Event::End(TagEnd::CodeBlock)) | None => break,
                Some(_) => {
                    eprintln!("This wasn't supposed to happen.");
                }
            }
        }

        Some(Event::Html(CowStr::from(render_code_block(
            &language, &literal,
        ))))
    }
}

fn render_code_block(language: &str, literal: &str) -> String {
    let mut out = String::new();

    if language.is_empty() {
        out.push_str(r#"<div class="highlighter-rouge"><div class="highlight"><pre class="highlight"><code>"#);
        out.push_str(&escape_html(literal));
    } else {
        let Ok(body) = highlight_with_rouge(language, literal) else {
            exit(1);
        };

        out.push_str(
            r#"<div class="language-{language} highlighter-rouge"><div class="highlight"><pre class="highlight"><code>"#
        );
        out.push_str(&body);
    }

    out.push_str("</code></pre></div></div>");

    out
}

fn highlight_with_rouge(language: &str, code: &str) -> Result<String, std::io::Error> {
    let mut command = Command::new("rougify");
    command.stdout(Stdio::null()).stderr(Stdio::null());

    let status = command.status();
    if let Err(e) = status
        && e.kind() == ErrorKind::NotFound
    {
        eprintln!("The CLI tool 'rougify' was not found, please install it from your package manager or use '--skip-syntax-highlighting'. Exiting...");
        return Err(e);
    }

    let mut child = Command::new("rougify")
        .args(["highlight", "-f", "html", "-l", language, "-i", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(code.as_bytes())?;
    }

    let output = child.wait_with_output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(std::io::Error::other(format!(
            "rougify exited with {}: {stderr}",
            output.status
        )));
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

fn escape_html(s: &str) -> String {
    let mut escaped = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            _ => escaped.push(c),
        }
    }

    escaped
}
