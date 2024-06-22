use miette::JSONReportHandler;
use oxc_allocator::Allocator;
use oxc_codegen::{CodeGenerator, WhitespaceRemover};
use oxc_minifier::{CompressOptions, Minifier, MinifierOptions};
use oxc_parser::Parser;
use oxc_span::SourceType;
use rustler::serde::SerdeTerm;
use rustler::NifStruct;
use std::path::Path;

#[derive(Debug, NifStruct)]
#[module = "OxcEx.CompressOptions"]
struct CompressOptionsWrapper {
    booleans: bool,
    drop_debugger: bool,
    drop_console: bool,
    evaluate: bool,
    join_vars: bool,
    loops: bool,
    typeofs: bool,
}

impl From<CompressOptionsWrapper> for CompressOptions {
    fn from(value: CompressOptionsWrapper) -> Self {
        CompressOptions {
            booleans: value.booleans,
            drop_debugger: value.drop_debugger,
            drop_console: value.drop_console,
            evaluate: value.evaluate,
            join_vars: value.join_vars,
            loops: value.loops,
            typeofs: value.typeofs,
        }
    }
}

#[rustler::nif]
fn minify(
    input: &str,
    might_be_path: &Path,
    compress_options: CompressOptionsWrapper,
) -> Result<String, SerdeTerm<serde_json::Value>> {
    let source_type = SourceType::from_path(might_be_path).unwrap();

    match minify_inner(&input, source_type, true, true, compress_options.into()) {
        Ok(output) => Ok(output),
        Err(e) => Err(SerdeTerm(e)),
    }
}

fn minify_inner(
    source_text: &str,
    source_type: SourceType,
    mangle: bool,
    whitespace: bool,
    compress: CompressOptions,
) -> Result<String, serde_json::Value> {
    let allocator = Allocator::default();
    let ret = Parser::new(&allocator, source_text, source_type).parse();
    if ret.panicked {
        let report = JSONReportHandler::new();
        let mut cursor = String::new();

        ret.errors.into_iter().for_each(|e| {
            report.render_report(&mut cursor, &e).ok();
        });
        let out = serde_json::from_str(&cursor).unwrap_or(serde_json::Value::Null);
        return Err(out);
    }

    let program = allocator.alloc(ret.program);
    let options = MinifierOptions { mangle, compress };
    Minifier::new(options).build(&allocator, program);
    let out_text = if whitespace {
        WhitespaceRemover::new().build(program)
    } else {
        CodeGenerator::new().build(program)
    }
    .source_text;

    Ok(out_text)
}

rustler::init!("Elixir.OxcEx.Native", [minify]);
