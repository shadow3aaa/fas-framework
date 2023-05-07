use crate::{Controller, Watcher};

// 返回支持的监视器
pub fn get_watcher(
    watchers: impl IntoIterator<Item = Box<dyn Watcher>>,
) -> Result<Box<dyn Watcher>, String> {
    for mut w in watchers.into_iter() {
        if w.support() {
            return Ok(w);
        }
    }
    Err(String::from(
        "There are currently no supported frame-aware schemes for this device",
    ))
}

// 返回元组(支持游戏和日用的控制器，只支持游戏的控制器)
type ControllerVec = Vec<Box<dyn Controller>>;
pub fn get_controller(
    controllers: impl IntoIterator<Item = Box<dyn Controller>>,
) -> Result<(ControllerVec, ControllerVec), String> {
    let mut only_game: ControllerVec = Vec::new();
    let mut all_support: ControllerVec = Vec::new();

    for mut c in controllers.into_iter() {
        if !c.support() {
            continue;
        }
        if c.d_support() {
            all_support.push(c);
        } else {
            only_game.push(c);
        }
    }
    if all_support.is_empty() {
        return Err(String::from(
            "This device does not currently have a supported performance controller",
        ));
    }
    Ok((all_support, only_game))
}
