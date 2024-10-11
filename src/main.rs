use bracket_lib::prelude::*;

//状态
struct State {

}
impl GameState for State {
    fn tick(&mut self,ctx : &mut BTerm){
        ctx.cls();
        ctx.print(1,1,"hello,bracket terminal!");
    }
}
fn main() -> BError{
    let context =BTermBuilder::simple80x50()
        .with_title("Fly Bird")
        .build()?;
    main_loop(context,State{})
}
