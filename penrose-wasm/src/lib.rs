use rquickjs::{
    loader::{BuiltinLoader, BuiltinResolver, ModuleLoader},
    module::ModuleDef,
    Context, Ctx, Function, Module, Object, Runtime, Value,
};

const EOL: &str = "\n";

pub struct OsModule;

impl ModuleDef for OsModule {
    fn declare(declare: &rquickjs::module::Declarations) -> rquickjs::Result<()> {
        declare.declare("EOL")?;
        declare.declare("default")?;
        rquickjs::Result::Ok(())
    }

    fn evaluate<'js>(
        ctx: &Ctx<'js>,
        exports: &rquickjs::module::Exports<'js>,
    ) -> rquickjs::Result<()> {
        let os = Object::new(ctx.clone())?;
        os.set("EOL", EOL)?;
        exports.export("EOL", EOL)?;
        exports.export("default", os)?;
        rquickjs::Result::Ok(())
    }
}

pub fn trio_to_svg(trio: Vec<u8>) -> Result<String, rquickjs::Error> {
    // let loader = (ModuleLoader::default().with_module("os", OsModule),);
    let loader = BuiltinLoader::default().with_module(
        "CC",
        r#"
// print("hello");
export const s = "abc";
        "#,
    );
    let resolver = (BuiltinResolver::default().with_module("CC"),);

    let runtime = Runtime::new()?;
    runtime.set_loader(resolver, loader);

    let context = Context::full(&runtime)?;

    context.with(|ctx| {
        // let globals = ctx.globals();
        // let name = "index.js";

        // let code = include_str!("./test.js");
        // Module::evaluate(ctx.clone(), "A", code)
        //     .unwrap()
        //     .finish::<Value>()?;

        // println!("{}", code);

        let unevaluated_module = Module::declare(
            ctx.clone(),
            "B",
            r#"
                import { s } from "CC";

                export { s as hey };
            "#,
        )
        .unwrap();

        let (module, prom) = unevaluated_module.clone().eval().unwrap();
        prom.finish::<()>();

        // let Ok((module, prom)) = ({
        //     unevaluated_module.eval()
        // }) else {
        //     println!("{:?}", ctx.catch());
        //     panic!();
        // };

        println!(
            "{:?}",
            (module
                .clone()
                .namespace()
                .unwrap()
                .keys::<String>()
                .collect::<Vec<_>>())
        );

        println!(
            "{:?}",
            (
                module.clone().get::<&str, Value>("hey").unwrap()
                // .keys::<String>()
                // .collect::<Vec<_>>())
            )
        );

        // match Module::evaluate(
        //     ctx.clone(),
        //     "B",
        //     r#"
        //             import { s } from "CC";

        //             print(s);
        //         "#,
        // ) {
        //     Ok(x) => {
        //         println!("{:?}", x);
        //         x.finish()?
        //     }
        //     Err(_) => {
        //         println!("{:?}", ctx.catch());
        //     }
        // }

        // .unwrap()
        // .finish::<()>()
        // .unwrap();

        // let value: Value = ctx
        //     .eval::<Value, &str>(code)
        //     // .unwrap()
        //     // ;
        //     .expect("evaluation failed");
        // println!("{:?}", value);

        // println!("{:?}", globals.clone().keys::<String>().collect::<Vec<_>>());
        // println!("he",);
        // let optimize: Function = globals.get("optimize")?;
        // let compile: Function = globals.get("compile")?;
        // let to_svg: Function = globals.get("toSVG")?;

        // const trio = {
        //         substance: `
        //           Set A
        //           Label A $e=mc^2$
        //         `,
        //         style: `canvas {
        //           width = 150
        //           height = 150
        //         }
        //         forall Set A {
        //           center = (0, 0)
        //           Circle {
        //             center: center
        //             r: 50
        //          }
        //           Equation {
        //             center: center
        //             string: A.label
        //           }
        //         }
        //         `,
        //         domain: `type Set`,
        //         variation: `test`,
        //       };
        //       const compiled = await compile(trio);
        //       if (compiled.isErr()) console.error(showError(compiled.error));
        //       const optimized = optimize(compiled.value);
        //       if (optimized.isErr()) console.error(showError(optimized.error));
        //       document
        //         .getElementById("penrose")
        //         .appendChild(await toSVG(optimized.value));

        // let config_code = format!("({})", config.unwrap_or("undefined"));
        // let config: Value = ctx.eval(config_code).ok()?;
        // let compiled: String = compile.call((trio,))?;
        // if (compiled.isErr()) console.error(showError(compiled.error));
        // let optimized: String = optimize.call((compiled,))?;
        // let optimized = optimize(compiled.value);
        // if (optimized.isErr()) console.error(showError(optimized.error));
        // let svg: String = to_svg.call((optimized,))?;
        // let ret: Object = optimize.call((svg, config)).ok()?;
        // let data: String = ret.get("data").ok()?;
        Ok::<String, rquickjs::Error>("success".to_string())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = run(b"", b"", b"", b"");
    //     assert_eq!(result, Ok(vec![]));
    // }

    #[test]
    fn it_works_trio_to_svg() {
        let result = run(b"", b"", b"", b"");
        assert_eq!(result, Ok(vec![]));
    }
}

wasm_minimal_protocol::initiate_protocol!();

#[wasm_minimal_protocol::wasm_func]
pub fn run(
    substance: &[u8],
    style: &[u8],
    domain: &[u8],
    variation: &[u8],
) -> Result<Vec<u8>, String> {
    trio_to_svg([substance, style, domain, variation].concat())
        .map_err(|err| err.to_string())
        .map(|svg| svg.into_bytes().to_vec())
}
