use std::io;
use mdbook::{BookItem};
use mdbook::renderer::RenderContext;
use std::path::PathBuf;
use pandoc::{Pandoc, PandocOption};

extern crate serde;
#[macro_use]
extern crate serde_derive;

/// The configuration object for the PDF backend, as represented in `book.toml`.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PdfConfig {
    /// The filename to provide the output.
    /// Filetype not required.
    pub output_name: Option<String>,

    /// Optional configuration settings to be passed to Pandoc.
    pub pandoc: Option<PandocConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PandocConfig {

    /// The PDF Engine to pass to Pandoc.
    /// -–pdf-engine=[engine]
    pub engine: Option<PdfEngineType>,

    /// The primary font to be used in your file output.
    /// Rather than support a wide range of variable names based on the pdf-engine
    /// chosen, mdbook-pdf attempts to standardise these variables and will adjust
    /// for you.
    /// For example if you are familiar with pdflatex, then this option controls the
    /// variable for `fontfamily`, whilst in xelatex it controls `mainfont`.
    pub main_font: Option<String>,
}

impl<'a> PandocConfig {
    pub fn set_engine(&self, pandoc: &'a mut Pandoc) -> &'a Pandoc {
        let engine_config = self.engine.as_ref();
        let engine_path: PathBuf = match engine_config.unwrap_or_default() {
            PdfEngineType::Context => PathBuf::from("context".to_string()),
            PdfEngineType::Lualatex => PathBuf::from("lualatex".to_string()),
            PdfEngineType::Pdflatex => PathBuf::from("pdflatex".to_string()),
            PdfEngineType::Pdfroff => PathBuf::from("pdfroff".to_string()),
            PdfEngineType::Prince => PathBuf::from("prince".to_string()),
            PdfEngineType::Weasyprint => PathBuf::from("weasyprint".to_string()),
            PdfEngineType::Wkhtmltopdf => PathBuf::from("wkhtmltopdf".to_string()),
            PdfEngineType::Xelatex => PathBuf::from("xelatex".to_string()),
        };
        pandoc.add_option(PandocOption::PdfEngine(engine_path))
    }

    pub fn set_main_font(&self, pandoc: &'a mut Pandoc) -> &'a Pandoc {
        let engine_config = self.engine.as_ref();
        let var_key  = match engine_config.unwrap_or_default() {
            PdfEngineType::Pdflatex => &"fontfamily",
            PdfEngineType::Lualatex => &"mainfont",
            PdfEngineType::Xelatex => &"mainfont",
            PdfEngineType::Context => &"mainfont",
            _ => {
                println!("WARNING: main_font configured, but engine type {:#?} does not support this option.", engine_config);
                return pandoc
            }
        };
        let font_config = match self.main_font.as_ref() {
            Some(font) => font,
            None => "Liberation Serif",
        };
        pandoc.set_variable(var_key, font_config)
    }
}

impl Default for &PdfEngineType {
    fn default() -> Self {
        &PdfEngineType::Xelatex
    }
}

impl Default for PandocConfig {
    fn default() -> Self {
        Self {
            engine: Some(PdfEngineType::Xelatex),
            main_font: Some((&"Liberation Serif").to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PdfEngineType {
    Wkhtmltopdf,
    Weasyprint,
    Prince,
    Pdflatex,
    Lualatex,
    Xelatex,
    Pdfroff,
    Context
}

impl PdfConfig {
    
    /// Load the PDF configuration data from the context provided to the backend by mdBook.
    pub fn from_context(ctx: &RenderContext) -> Option<PdfConfig> {
        match ctx.config.get_deserialized_opt("output.pdf") {
            Ok(Some(cfg)) => Some(cfg),
            Ok(None) => Some(PdfConfig::default()),
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

        let mut pandoc= Pandoc::new();
        let pandoc_config = self.pandoc.unwrap_or_default();
        pandoc_config.set_engine(&mut pandoc);
        pandoc_config.set_main_font(&mut pandoc);

        pandoc.set_input_format(pandoc::InputFormat::Commonmark, input_ext);
        pandoc.set_output_format(pandoc::OutputFormat::Latex, output_ext);

        pandoc.set_show_cmdline(true);
        //pandoc.set_doc_class(pandoc::DocumentClass::Report);

        // initialize the content
        let mut content = String::new();

        // set output filename
        let mut filename: PathBuf = PathBuf::new();

        match self.output_name {
            Some(name) => {
                filename.push(name);
            }
            None => {
                    // default to setting the filename based on the book title
                    filename.push(context.config.book.title.unwrap());
                }
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
    let cfg = PdfConfig::from_context(&ctx);
    let pandoc = cfg.unwrap().evaluate_opts(ctx);
    pandoc.execute().unwrap();
}