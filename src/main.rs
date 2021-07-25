use std::io;
use mdbook::BookItem;
use mdbook::renderer::RenderContext;
use mdbook::book::Chapter;
use pandoc::Pandoc;

extern crate serde;
#[macro_use]
extern crate serde_derive;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct PdfConfig {
    pub ignores: Vec<String>,
}

fn main() {
    let mut stdin = io::stdin();
    let ctx = RenderContext::from_json(&mut stdin).unwrap();
    let cfg: PdfConfig = ctx.config
        .get_deserialized("output.pdf")
        .unwrap_or_default();

    for item in ctx.book.iter() {

        if let BookItem::Chapter(ref ch) = *item {
            if let true = &ch.path.is_some() {


                let mut filename: std::path::PathBuf = std::path::PathBuf::new();
                filename.push(&ch.name);
                filename.set_extension("pdf");

                let content = &ch.content;
                let name = &ch.name;
                let mut md_extensions = Vec::new();
                md_extensions.push(pandoc::MarkdownExtension::BacktickCodeBlocks);
                let mut pandoc = Pandoc::new();
                pandoc.set_input(pandoc::InputKind::Pipe(content.to_string()));
                pandoc.set_input_format(pandoc::InputFormat::Commonmark, Vec::new());
                pandoc.set_output(pandoc::OutputKind::File(filename));
                pandoc.set_output_format(pandoc::OutputFormat::Pdf, Vec::new());
                pandoc.execute().unwrap();
            }
        }
    }
}