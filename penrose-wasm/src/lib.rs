wasm_minimal_protocol::initiate_protocol!();

use rquickjs::{
    loader::{BuiltinResolver, ModuleLoader},
    module::ModuleDef,
    Context, Ctx, Function, Module, Object, Runtime, Value,
};
// #[cfg(target_os = "windows")]
// const EOL: &str = "\r\n";

// #[cfg(not(target_os = "windows"))]
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

pub fn osvg(svg: &str, config: Option<&str>) -> Option<String> {
    let runtime = Runtime::new().ok()?;
    let context = Context::full(&runtime).ok()?;
    let loader = (ModuleLoader::default().with_module("os", OsModule),);
    let resolver = (BuiltinResolver::default().with_module("os"),);
    runtime.set_loader(resolver, loader);

    let s = context.with(|ctx| {
        let global = ctx.globals();
        let name = "osvg.js";
        let code = include_str!("../../node_modules/@penrose/core/dist/index.js");
        Module::evaluate(ctx.clone(), name, code)
            .unwrap()
            .finish::<Value>()
            .ok()?;
        let optimize: Function = global.get("optimize").ok()?;
        let compile: Function = global.get("compile").ok()?;

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

        let config_code = format!("({})", config.unwrap_or("undefined"));
        let config: Value = ctx.eval(config_code).ok()?;
        let ret: Object = optimize.call((svg, config)).ok()?;
        let data: String = ret.get("data").ok()?;
        Some(data)
    })?;

    Some(s)
}

#[wasm_minimal_protocol::wasm_func]
pub fn run(
    substance: &[u8],
    style: &[u8],
    domain: &[u8],
    variation: &[u8],
) -> Result<Vec<u8>, String> {
    Ok(b"Some SVG!".to_vec())
}

// #[wasm_func]
// pub fn hello() -> Vec<u8> {
//     b"Hello from wasm!!!".to_vec()
// }

// #[wasm_func]
// pub fn double_it(arg: &[u8]) -> Vec<u8> {
//     [arg, arg].concat()
// }

// #[wasm_func]
// pub fn concatenate(arg1: &[u8], arg2: &[u8]) -> Vec<u8> {
//     [arg1, b"*", arg2].concat()
// }

// #[wasm_func]
// pub fn shuffle(arg1: &[u8], arg2: &[u8], arg3: &[u8]) -> Vec<u8> {
//     [arg3, b"-", arg1, b"-", arg2].concat()
// }

// #[wasm_func]
// pub fn returns_ok() -> Result<Vec<u8>, String> {
//     Ok(b"This is an `Ok`".to_vec())
// }

// #[wasm_func]
// pub fn returns_err() -> Result<Vec<u8>, String> {
//     Err(String::from("This is an `Err`"))
// }

// #[wasm_func]
// pub fn will_panic() -> Vec<u8> {
//     panic!("unconditional panic")
// }
