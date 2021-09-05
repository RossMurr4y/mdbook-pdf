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
use serde::{Deserialize};
use serde_derive::{Deserialize};



#[derive(Debug, Deserialize)]
#[serde(bound(deserialize = "
    Vec<MarkdownExtension>: Deserialize<'de>,
    Vec<PandocOption>: Deserialize<'de>,
"))]
pub struct Pdf {
    pub name: PathBuf,
    pub engine: PathBuf,
    pub format: PdfFormat,
    // Content of the PDF.
    pub content: String,
    pub input_extensions: Vec<MarkdownExtension>,
    pub output_extensions: Vec<MarkdownExtension>,
    pub options: Vec<PandocOption>,
    pub book_root: PathBuf,
}

#[derive(Debug, Default, Deserialize)]
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
                if !input.format.is_none() { self.format = input.format };
                self
            },
            None => self,
        }
    }

    // completes the builder and instantiates the Pdf configuration.
    // Applies default values if they are not currently set.
    fn build(&self, content: String, context: RenderContext) -> Result<Pdf, mdbook::errors::Error> {

        // set the source directory 
        let mut root_dir = PathBuf::from(&context.root);

        let mut filename: PathBuf = PathBuf::new();
        filename.push(self.name.clone().unwrap_or_else(|| context.config.book.title.unwrap().to_string()));
        filename.set_extension("pdf");
        Ok(Pdf {
            name: filename,
            engine: PathBuf::from(self.engine.clone().unwrap_or_else(|| "xelatex".to_string())),
            format: self.format.clone().unwrap_or_else(|| Default::default()),
            content: content,
            input_extensions: Vec::new(),
            output_extensions: Vec::new(),
            options: Vec::new(),
            book_root: root_dir,
        })
    }
}

impl Pdf {
    pub fn to_pandoc(mut self) -> Pandoc {
        // process all the inputs
        let mut pandoc = Pandoc::new();

        let mut src_path = self.book_root;
        src_path.push("src");

        // add Commonmark extensions
        self.input_extensions.push(MarkdownExtension::AutoIdentifiers);
        self.input_extensions.push(MarkdownExtension::BacktickCodeBlocks);
        self.input_extensions.push(MarkdownExtension::FancyLists);
        self.input_extensions.push(MarkdownExtension::PipeTables);
        self.input_extensions.push(MarkdownExtension::ImplicitFigures);
        self.input_extensions.push(MarkdownExtension::MultilineTables);
        self.input_extensions.push(MarkdownExtension::Startnum);
        self.input_extensions.push(MarkdownExtension::YamlMetadataBlock);

        // add Latex extensions
        // tbc

        // set the name of the pandoc font option based on engine
        // some engines use a different name for this option.
        let font_var = if self.engine == PathBuf::from("pdflatex") { &"fontfamily" } else { &"mainfont"};

        pandoc
            .set_variable(&"toc", &"")
            .set_variable(&"toc-depth", &"2")
            .set_input(InputKind::Pipe(self.content))
            .set_input_format(pandoc::InputFormat::Markdown, self.input_extensions)
            .set_output(File(self.name))
            .set_output_format(Latex, self.output_extensions)
            .add_option(PandocOption::PdfEngine(self.engine))
            .set_doc_class(pandoc::DocumentClass::Article)
            .add_option(PandocOption::ResourcePath(vec!(src_path)))
            .set_variable(font_var, &self.format.font.unwrap());
        pandoc
    }
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
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
    content.push_str(format!("# {} \n", ctx.config.book.title.clone().unwrap()).as_str());
    for item in &mut ctx.book.iter() {

        if let BookItem::PartTitle(ref title) = *item {
            content.push_str(format!(
                "# {}
                
                ", title).as_str());
        }

        if let BookItem::Chapter(ref ch) = *item {
            if let true = &ch.path.is_some() {
                content.push_str(&ch.content);
            }
        }
    }

    // translate our book.toml config into the Pdf struct
    let mut builder  = PdfBuilder::new();
    builder.with_input_context(&ctx);
    let pandoc = builder
        .build(content, ctx)
        .unwrap()
        .to_pandoc();

    // give it all to pandoc
    pandoc.execute().unwrap();
}