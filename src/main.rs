use std::io;
use mdbook::{BookItem};
use mdbook::renderer::RenderContext;
use std::path::PathBuf;
use pandoc::{
    InputKind,
    InputFormat::Commonmark,
    MarkdownExtension,
    OutputKind::File,
    OutputFormat::Latex,
    Pandoc,
    PandocOption
};

extern crate serde;
#[macro_use]
extern crate serde_derive;

#[derive(Debug, Serialize, Deserialize)]
pub struct Pdf {
    pub name: String,
    pub engine: PathBuf,
    pub format: PdfFormat,
    // Content of the PDF.
    pub content: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct PdfBuilder {
    pub name: Option<String>,
    pub engine: Option<String>,
    pub format: Option<PdfFormat>,
}

impl PdfBuilder {

    fn default(context: &RenderContext) -> PdfBuilder {
        PdfBuilder {
            name: context.config.book.title.clone(),
            engine: Some("xelatex".to_string()),
            format: Some(PdfFormat::default()),
        }
    }

    fn apply_input_context(&mut self, ctx: &RenderContext) -> Self {
        match ctx.config.get_deserialized_opt("output.pdf".to_string()).unwrap() {
            Some(input) => {
                Self {
                    ..input
                }
            }
            _ => {
                    println!("WARN: PDF configuration found in book.toml is invalid. Using default settings.");
                    PdfBuilder::default(ctx)
                }
        }
    }

    // completes the builder and instantiates the Pdf configuration.
    fn build(self, content: String) -> Pdf {
        Pdf {
            name: self.name.unwrap(),
            engine: PathBuf::from(self.engine.unwrap()),
            format: self.format.unwrap_or(Default::default()),
            content: content,
        }
    }
}

impl Pdf {
    // builder for the Pdf struct
    pub fn new(context: &RenderContext) -> PdfBuilder {
        PdfBuilder::default(context).apply_input_context(context)
    }

    // processes each of the inputs
    pub fn evaluate_pdf_input(self) -> Pandoc {

        // Set the output name and extension
        let mut filename: PathBuf = PathBuf::new();
        filename.push(self.name);
        filename.set_extension("pdf".to_string());

        // set the Commonmark extensions
        let mut input_ext: Vec<MarkdownExtension> = Vec::new();


        // set the Latex extensions
        let mut output_ext: Vec<MarkdownExtension> = Vec::new();

        // construct the call to pandoc
        let mut pandoc = Pandoc::new();
        pandoc
            .set_input_format(Commonmark, input_ext)
            .set_input(InputKind::Pipe(self.content))
            .set_output(File(filename))
            .set_output_format(Latex, output_ext)
            .add_option(PandocOption::PdfEngine(self.engine));

        pandoc
    }    
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PdfFormat {
    pub font: String,
}
impl Default for PdfFormat {
    fn default() -> Self {
        Self {
            font: "DejaVue Sans".to_string(),
        }
    }
}

fn main() {
    let mut stdin = io::stdin();
    let ctx = RenderContext::from_json(&mut stdin).unwrap();

    // gather our content
    let mut content = String::new();
    for item in &mut ctx.book.iter() {
        if let BookItem::Chapter(ref ch) = *item {
            if let true = &ch.path.is_some() {
                content.push_str(&ch.content);
            }
        }
    }

    // translate our book.toml config into the Pdf struct
    let input: Pdf = Pdf::new(&ctx)
        .build(content);

    // process all the inputs
    let pandoc = input.evaluate_pdf_input();

    // give it all to pandoc
    pandoc.execute().unwrap();
}