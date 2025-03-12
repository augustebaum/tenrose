wasm_minimal_protocol::initiate_protocol!();

use rquickjs::{
    loader::{BuiltinResolver, ModuleLoader},
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
    let runtime = Runtime::new()?;
    let context = Context::full(&runtime)?;
    let loader = (ModuleLoader::default().with_module("os", OsModule),);
    let resolver = (BuiltinResolver::default().with_module("os"),);
    runtime.set_loader(resolver, loader);

    context.with(|ctx| {
        let global = ctx.globals();
        let name = "osvg.js";
        let code = include_str!("../../node_modules/@penrose/core/dist/index.js");
        Module::evaluate(ctx.clone(), name, code)
            .unwrap()
            .finish::<Value>()?;
        let optimize: Function = global.get("optimize")?;
        let compile: Function = global.get("compile")?;
        let to_svg: Function = global.get("toSVG")?;

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
        let compiled: String = compile.call((trio,))?;
        // if (compiled.isErr()) console.error(showError(compiled.error));
        let optimized: String = optimize.call((compiled,))?;
        // let optimized = optimize(compiled.value);
        // if (optimized.isErr()) console.error(showError(optimized.error));
        let svg: String = to_svg.call((optimized,))?;
        // let ret: Object = optimize.call((svg, config)).ok()?;
        // let data: String = ret.get("data").ok()?;
        Ok::<String, rquickjs::Error>(svg)
    })

    // Ok(s.unwrap())
}

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
