use swc_core::common::sync::Lrc;
use swc_core::common::{
    errors::{ColorConfig, Handler},
    FileName, SourceMap,
};
use swc_core::ecma::ast::Program;
use swc_core::ecma::codegen::{text_writer::JsWriter, Emitter};
use swc_core::ecma::parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn parse_program(cm: Lrc<SourceMap>, code: &str) -> Program {
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

    // Real usage
    // let fm = cm
    //     .load_file(Path::new("test.js"))
    //     .expect("failed to load test.js");
    let fm = cm.new_source_file(FileName::Custom("test.ts".into()), code.into());
    let lexer = Lexer::new(
        Syntax::Typescript(TsConfig {
            tsx: true,
            ..Default::default()
        }),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );
    let mut parser = Parser::new_from(lexer);
    for e in parser.take_errors() {
        e.into_diagnostic(&handler).emit();
    }

    let program = parser
        .parse_program()
        .map_err(|e| e.into_diagnostic(&handler).emit())
        .expect("failed to parser module");
    println!("{:#?}", program);

    program
}

pub fn codegen_from(cm: Lrc<SourceMap>, program: &Program) -> String {
    let mut buf = vec![];
    {
        let mut emitter = Emitter {
            cfg: swc_core::ecma::codegen::Config {
                ..Default::default()
            },
            cm: cm.clone(),
            comments: None,
            wr: JsWriter::new(cm, "\n", &mut buf, None),
        };
        emitter.emit_program(&program).unwrap();
    }
    String::from_utf8_lossy(&buf).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_get_program_ast_and_codegen() {
        let input_code = r#"const foo = ()=>{
    console.log('bar');
};
"#;
        let cm: Lrc<SourceMap> = Default::default();
        let program = parse_program(cm.clone(), input_code);
        let code = codegen_from(cm.clone(), &program);
        println!("{}", &code);
        assert_eq!(input_code, &code);
    }
}
