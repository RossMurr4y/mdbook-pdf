use std::result::Result;
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


#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PdfBuilder {
    pub name: Option<String>,
    pub engine: Option<String>,
    pub format: Option<PdfFormat>,
}

impl PdfBuilder {

    pub fn new() -> Self {
        PdfBuilder {
            name: None,
            engine: None,
            format: None,
        }
    }

    fn with_input_context<'de>(&mut self, ctx: &RenderContext) -> &Self {
        match ctx.config.get_deserialized_opt::<PdfBuilder, String>("output.pdf".to_string()).unwrap() {
            Some(input) => {
                if !input.name.is_none() { self.name = input.name };
                if !input.engine.is_none() { self.engine = input.engine };
                if !(Some(PdfFormat::default()) == input.format) { self.format = input.format };
                self
            },
            None => self,
        }
    }

    fn with_name(&mut self, name: String) ->  &mut Self {
        self.name = Some(name);
        self
    }

    fn with_engine(&mut self, engine: String) -> &mut Self {
        self.engine = Some(engine);
        self
    }

    fn with_format(&mut self, format: PdfFormat) -> &mut Self {
        self.format = Some(format);
        self
    }

    // completes the builder and instantiates the Pdf configuration.
    // Applies default values if they are not currently set.
    fn build(self, content: String, context: RenderContext) -> Result<Pdf, mdbook::errors::Error> {
        Ok(Pdf {
            name: self.name.as_ref().unwrap_or(&context.config.book.title.unwrap()).to_string(),
            engine: PathBuf::from(self.engine.as_ref().unwrap_or(&"xelatex".to_string())),
            format: self.format.unwrap_or(PdfFormat::default()),
            content: content,
        })
    }
}

impl Pdf {

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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PdfFormat {
    pub font: Option<String>,
}
impl Default for PdfFormat {
    fn default() -> Self {
        Self {
            font: Some("DejaVu Sans".to_string()),
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
    let mut builder  = PdfBuilder::new();
    builder.with_input_context(&ctx);
    let input = builder.build(content, ctx);

    // process all the inputs
    let pandoc = input.expect("Error unwraping the pdf builder result.").evaluate_pdf_input();

    // give it all to pandoc
    pandoc.execute().unwrap();
}