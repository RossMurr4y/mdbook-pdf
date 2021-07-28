use std::io;
use mdbook::{BookItem, Config, config, utils};
use mdbook::renderer::RenderContext;
use mdbook::book::Chapter;
use pandoc::DocumentClass::Book;
use pandoc::Pandoc;
use toml::{self, Value};

extern crate serde;
#[macro_use]
extern crate serde_derive;

/// The configuration object for the PDF backend, as represented in `book.toml`.
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct PdfConfig {
    pub pdf_name: Option<String>,
}

impl PdfConfig {
    
    /// Load the PDF configuration data from the context provided to the backend by mdBook.
    pub fn from_context(ctx: &RenderContext) -> Option<PdfConfig> {
        match ctx.config.get_deserialized_opt("output.pdf") {
            Ok(Some(cfg)) => Some(cfg),
            Ok(None) => None,
            Err(e) => {
                mdbook::utils::log_backtrace(&e);
                None
            }
        }
    }

    /// Evaluate configuration properties and create a new RenderContext from them
    fn evaluate_opts(self, context: RenderContext) -> Pandoc {

        // initialising for later use
        let input_ext = Vec::new();
        let output_ext = Vec::new();

        let mut pandoc: Pandoc = Pandoc::new();
        pandoc.set_input_format(pandoc::InputFormat::Commonmark, input_ext);
        pandoc.set_output_format(pandoc::OutputFormat::Pdf, output_ext);

        // initialize the content
        let mut content = String::new();

        // set output filename
        let mut filename: std::path::PathBuf = std::path::PathBuf::new();
        if let Some(name) = self.pdf_name {
            filename.push(name);
        } else {
            // default to setting the filename based on the book title
            filename.push(context.config.book.title.unwrap());
        }
        filename.set_extension("pdf");
        pandoc.set_output(pandoc::OutputKind::File(filename));

        // set the output content
        for item in context.book.iter() {
            if let BookItem::Chapter(ref ch) = *item {
                if let true = &ch.path.is_some() {
                    content.push_str(&ch.content);
                }
            }
        }
    
        // apply the content
        pandoc.set_input(pandoc::InputKind::Pipe(content.to_string()));
        pandoc
    }
}

fn main() {
    let mut stdin = io::stdin();
    let ctx = RenderContext::from_json(&mut stdin).unwrap();
    let cfg: Option<PdfConfig> = PdfConfig::from_context(&ctx);
    let pandoc = cfg.unwrap().evaluate_opts(ctx);
    pandoc.execute().unwrap();
}