use crate::fs_utils::{ClearContents, copy_recursively, strip_liquid_extension};
use crate::site::{BlogPost, Site};
use crate::time_utils::format_utc;
use chrono::Utc;
use liquid::Parser as LiquidParser;
use liquid::{Object, Template, ValueView, object};
use std::fs::{create_dir_all, read_to_string, write};
use std::path::{Path, PathBuf};

pub struct SiteRenderer {
    liquid_parser: LiquidParser,
}

pub struct Renderable<'a> {
    html_file: &'a PathBuf,
    header_template: Option<&'a Template>,
    current_section: Option<&'a str>,
    template: Option<&'a Template>,
    title: Option<&'a String>,
    extra_page_data: Option<Object>,
}

impl SiteRenderer {
    pub fn new(liquid_parser: LiquidParser) -> SiteRenderer {
        SiteRenderer { liquid_parser }
    }

    pub fn render(&self, site: Site, root_dir: &Path, output_dir: &Path) {
        let liquid_parser = &self.liquid_parser;
        let blog_posts = site.blog_posts;
        let post_data: Vec<Object> = blog_posts
            .iter()
            .map(Self::blog_post_to_liquid_data)
            .collect();

        let layouts_dir = root_dir.join("layouts");
        let default_template = liquid_parser
            .parse_file(layouts_dir.join("default.html.liquid"))
            .unwrap();
        let post_template = liquid_parser
            .parse_file(layouts_dir.join("post.html.liquid"))
            .unwrap();
        let header_template = liquid_parser
            .parse_file(root_dir.join("header.html.liquid"))
            .unwrap();

        output_dir.delete_recursively();

        copy_recursively(root_dir, &root_dir.join("static"), output_dir).unwrap();
        // copy_recursively(root_dir, &root_dir.join("_redirects"), output_dir).unwrap();

        let site_data: Object = object!({
            "url": "https://danperez.dev",
            "time": format_utc(Utc::now()),
            "posts": post_data,
        });

        Self::render_html(
            &Renderable {
                html_file: &root_dir.join("index.html.liquid"),
                header_template: Some(&header_template),
                current_section: Some("home"),
                template: None,
                title: None,
                extra_page_data: None,
            },
            &site_data,
            &output_dir.join("index.html"),
            liquid_parser,
        );

        Self::render_html(
            &Renderable {
                html_file: &root_dir.join("atom.xml"),
                header_template: None,
                current_section: None,
                template: None,
                title: None,
                extra_page_data: None,
            },
            &site_data,
            &output_dir.join("atom.xml"),
            liquid_parser,
        );

        Self::render_html(
            &Renderable {
                html_file: &root_dir.join("blog.html.liquid"),
                header_template: Some(&header_template),
                current_section: Some("blog"),
                template: Some(&default_template),
                title: Some(&String::from("Posts")),
                extra_page_data: None,
            },
            &site_data,
            &output_dir.join("blog/index.html"),
            liquid_parser,
        );

        Self::render_html(
            &Renderable {
                html_file: &root_dir.join("../site/resume.html.liquid"),
                header_template: Some(&header_template),
                current_section: Some("resume"),
                template: Some(&default_template),
                title: Some(&String::from("Resume")),
                extra_page_data: Some(object!({
                    "resume_last_updated": site.resume.last_updated.format("%Y-%m-%d").to_string(),
                })),
            },
            &site_data,
            &output_dir.join("resume/index.html"),
            liquid_parser,
        );

        for (blog_post, page_data) in blog_posts.iter().zip(&post_data) {
            Self::render_page(
                output_dir,
                Some(&header_template),
                page_data,
                Some(&post_template),
                &site_data,
            );
            println!(
                "Rendered page {} with date {}\n",
                blog_post.slug, blog_post.date
            );
        }
    }

    fn render_html(
        renderable: &Renderable,
        site_data: &Object,
        output_file: &PathBuf,
        liquid_parser: &LiquidParser,
    ) {
        println!("Rendering {} to HTML...", renderable.html_file.display());

        let header_rendered = renderable.header_template.map(|t| {
            t.render(&object!({ "current_section": renderable.current_section }))
                .unwrap()
        });
        let mut page_obj: Object = object!({
            "title": renderable.title,
        });
        if let Some(extra) = &renderable.extra_page_data {
            for (k, v) in extra {
                page_obj.insert(k.clone(), v.clone());
            }
        }

        let content = read_to_string(renderable.html_file).unwrap();
        let intermediate_data: Object = object!({
            "site": site_data,
            "header-content": header_rendered,
            "page": page_obj,
        });
        let intermediate = liquid_parser
            .parse(&content)
            .unwrap()
            .render(&intermediate_data)
            .unwrap();

        let rendered = if let Some(template) = renderable.template {
            template
                .render(&object!({
                    "content": intermediate,
                    "page": page_obj,
                    "header-content": header_rendered,
                    "site": site_data
                }))
                .unwrap()
        } else {
            intermediate
        };

        create_dir_all(output_file.parent().unwrap()).unwrap();
        write(output_file, rendered).unwrap();

        println!(
            "Successfully rendered HTML for {}\n",
            renderable.html_file.display()
        );
    }

    fn render_page(
        output_dir: &Path,
        header_template: Option<&Template>,
        page_data: &Object,
        template: Option<&Template>,
        site_data: &Object,
    ) {
        println!("Rendering page {}", page_data.get("url").unwrap().to_kstr());

        let header_rendered =
            header_template.map(|t| t.render(&object!({ "current_section": "blog" })).unwrap());

        let binding = page_data.get("content").unwrap().to_kstr();
        let content = binding.as_str();
        let rendered = if let Some(template) = template {
            &template
                .render(&object!({
                    "content": content,
                    "header-content": header_rendered,
                    "page": page_data,
                    "site": site_data,
                }))
                .unwrap()
        } else {
            content
        };

        let url_path = page_data.get("url").unwrap();
        let output_file = output_dir.join(Self::url_path_to_relative_file_path(
            url_path.to_kstr().as_str(),
        ));
        let output_file = strip_liquid_extension(&output_file);
        if let Some(parent) = output_file.parent() {
            create_dir_all(parent).expect("Could not create parent");
        }
        std::fs::write(&output_file, rendered).expect("Could not write output HTML");
    }

    fn url_path_to_relative_file_path(path: &str) -> String {
        let trimmed = path.trim_start_matches('/');
        let suffix = if path.ends_with('/') {
            "index.html.liquid"
        } else {
            ".html"
        };
        format!("{trimmed}{suffix}")
    }

    fn blog_post_to_liquid_data(blog_post: &BlogPost) -> Object {
        object!({
            "title": blog_post.title,
            "id": format!("/{}", blog_post.slug),
            "url": format!("/{}/", blog_post.slug),
            "lead": blog_post.lead,
            "date": blog_post.date.clone(),
            "content": blog_post.content,
        })
    }
}
