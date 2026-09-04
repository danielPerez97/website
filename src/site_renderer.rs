use std::fs::{create_dir_all, read_to_string, write};
use std::path::PathBuf;
use chrono::Utc;
use liquid::{object, Object, Template, ValueView, ParserBuilder};
use liquid::Parser as LiquidParser;
use crate::liquid_xml_escape::XmlEscape;
use crate::fs_utils::{copy_recursively, ClearContents};
use crate::liquid_date_to_xml_schema::DateToXmlSchema;
use crate::site::{BlogPost, Site};
use crate::time_utils::format_utc;

pub struct SiteRenderer {
    liquid_parser: LiquidParser,
}

impl SiteRenderer {
    pub fn new() -> SiteRenderer {
        SiteRenderer {
            liquid_parser: ParserBuilder::with_stdlib()
                .filter(XmlEscape)
                .filter(DateToXmlSchema)
                .build().unwrap(),
        }
    }

    pub fn render(&self, site: Site, root_dir: &PathBuf, output_dir: &PathBuf) {
        let liquid_parser = &self.liquid_parser;
        let blog_posts = site.blog_posts;
        let post_data: Vec<Object> = blog_posts.iter()
            .map(Self::blog_post_to_liquid_data)
            .collect();

        let layouts_dir = root_dir.join("layouts");
        let default_template = liquid_parser.parse_file(layouts_dir.join("default.html")).unwrap();
        let post_template = liquid_parser.parse_file(layouts_dir.join("post.html")).unwrap();
        let header_file = root_dir.join("header.html");
        let header_content = read_to_string(header_file).unwrap();

        output_dir.delete_recursively();

        copy_recursively(root_dir, &root_dir.join("static"), output_dir).unwrap();
        // copy_recursively(root_dir, &root_dir.join("_redirects"), output_dir).unwrap();

        let site_data: Object = object!({
            "url": "https://danperez.dev",
            "time": format_utc(Utc::now()),
            "posts": post_data,
        });

        Self::render_html(
            &root_dir.join("index.html"),
            Some(&header_content),
            None,
            None,
            &site_data,
            &output_dir.join("index.html"),
            liquid_parser
        );

        Self::render_html(
            &root_dir.join("atom.xml"),
            None,
            None,
            None,
            &site_data,
            &output_dir.join("atom.xml"),
            liquid_parser
        );

        Self::render_html(
            &root_dir.join("blog.html"),
            Some(&header_content),
            Some(&default_template),
            Some(String::from("Posts")),
            &site_data,
            &output_dir.join("blog/index.html"),
            liquid_parser
        );


        Self::render_html(
            &root_dir.join("../site/resume.html"),
            Some(&header_content),
            Some(&default_template),
            Some(String::from("Resume")),
            &site_data,
            &output_dir.join("resume/index.html"),
            liquid_parser
        );

        for (blog_post, page_data) in blog_posts.iter().zip(&post_data) {
            Self::render_page(output_dir, page_data, Some(&post_template), &site_data);
            println!("Rendered page {} with date {}\n", blog_post.slug, blog_post.date)
        }
    }

    fn render_html(
        html_file: &PathBuf,
        header: Option<&String>,
        template: Option<&Template>,
        title: Option<String>,
        site_data: &Object,
        output_file: &PathBuf,
        liquid_parser: &LiquidParser,
    ) {
        println!("Rendering {} to HTML...", html_file.display());

        let content = read_to_string(html_file).unwrap();
        let intermediate_data: Object = object!({
            "site": site_data,
            "header-content": header,
        });
        let intermediate = liquid_parser.parse(&content)
            .unwrap().render(&intermediate_data).unwrap();

        let rendered = if let Some(template) = template {
            template.render(&object!({
                "content": intermediate,
                "page": object!({
                    "title": title,
                }),
                "header-content": header,
                "site": site_data
            })).unwrap()
        } else {
            intermediate
        };

        create_dir_all(output_file.parent().unwrap()).unwrap();
        write(output_file, rendered).unwrap();

        println!("Successfully rendered HTML for {}\n", html_file.display())
    }

    fn render_page(
        output_dir: &PathBuf,
        page_data: &Object,
        template: Option<&Template>,
        site_data: &Object
    ) {
        println!("Rendering page {}", page_data.get("url").unwrap().to_kstr());

        let binding = page_data.get("content").unwrap().to_kstr();
        let content = binding.as_str();
        let rendered = if template.is_none() {
            content
        } else {
            &template.unwrap().render(
                &object!({
                    "content": content,
                    "page": page_data,
                    "site": site_data,
                })
            ).unwrap()
        };

        let url_path = page_data.get("url").unwrap();
        let output_file = output_dir.join(Self::url_path_to_relative_file_path(url_path.to_kstr().as_str()));
        if let Some(parent) = output_file.parent() {
            create_dir_all(parent).expect("Could not create parent");
        }
        std::fs::write(&output_file, rendered).expect("Could not write output HTML");
    }

    fn url_path_to_relative_file_path(path: &str) -> String {
        let trimmed = path.trim_start_matches('/');
        let suffix = if path.ends_with('/') { "index.html" } else { ".html" };
        format!("{trimmed}{suffix}")
    }

    fn blog_post_to_liquid_data(blog_post: &BlogPost) -> Object {
        object!({
            "title": blog_post.title,
            "id": format!("/{}", blog_post.slug),
            "url": format!("/{}/", blog_post.slug),
            "lead": "TODO",
            "date": blog_post.date.clone(),
            "content": blog_post.content,
        })
    }
}