use chrono::{Datelike, Utc};
use mlua::Function;

fn main() {
    let lua_vm = mlua::Lua::new();
    let lua_fn = format!("
        function (year, month, day, weekday)
            return {}
        end
    ", "day == 6 and month == 5");
    let x = Utc::now();
    let lua_fn: Function = lua_vm.load(lua_fn).eval().unwrap();
    let r: bool = lua_fn.call((x.year(), x.month(), x.day(), x.weekday().to_string())).unwrap();
    println!("{}", x.day());
    println!("{r}");
}
