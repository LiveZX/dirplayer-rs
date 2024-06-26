mod utils;
mod player;
mod io;
mod js_api;
mod rendering;

use async_std::task::spawn_local;
use js_api::JsApi;
use num::ToPrimitive;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

#[macro_use]
extern crate pest_derive;

mod director;

use player::{cast_lib::{cast_member_ref, CastMemberRef}, commands::{player_dispatch, PlayerVMCommand}, init_player, DatumRef, PLAYER_LOCK};

#[wasm_bindgen]
extern "C" {
  fn alert(s: &str);
}

#[wasm_bindgen]
pub fn set_base_path(path: String) {
  player_dispatch(PlayerVMCommand::SetBasePath(path));
}

#[wasm_bindgen]
pub async fn load_movie_file(path: String) {
  player_dispatch(PlayerVMCommand::LoadMovieFromFile(path));
}

#[wasm_bindgen]
pub fn play() {
  player_dispatch(PlayerVMCommand::Play);
}

#[wasm_bindgen]
pub fn stop() {
  player_dispatch(PlayerVMCommand::Stop);
}

#[wasm_bindgen]
pub fn reset() {
  player_dispatch(PlayerVMCommand::Reset);
}

#[wasm_bindgen]
pub fn add_breakpoint(script_name: String, handler_name: String, bytecode_index: usize) {
  player_dispatch(PlayerVMCommand::AddBreakpoint(script_name, handler_name, bytecode_index))
}

#[wasm_bindgen]
pub fn remove_breakpoint(script_name: String, handler_name: String, bytecode_index: usize) {
  player_dispatch(PlayerVMCommand::RemoveBreakpoint(script_name, handler_name, bytecode_index))
}

#[wasm_bindgen]
pub fn toggle_breakpoint(script_name: String, handler_name: String, bytecode_index: usize) {
  player_dispatch(PlayerVMCommand::ToggleBreakpoint(script_name, handler_name, bytecode_index))
}

#[wasm_bindgen]
pub fn resume_breakpoint() {
  player_dispatch(PlayerVMCommand::ResumeBreakpoint);
}

#[wasm_bindgen]
pub fn set_stage_size(width: u32, height: u32) {
  player_dispatch(PlayerVMCommand::SetStageSize(width, height));
}

#[wasm_bindgen]
pub fn trigger_timeout(name: &str) {
  player_dispatch(PlayerVMCommand::TimeoutTriggered(name.to_string()));
}

#[wasm_bindgen]
pub fn player_print_member_bitmap_hex(cast_lib: i32, cast_member: i32) {
  player_dispatch(PlayerVMCommand::PrintMemberBitmapHex(CastMemberRef { cast_lib, cast_member }));
}

#[wasm_bindgen]
pub fn mouse_down(x: f64, y: f64) {
  player_dispatch(PlayerVMCommand::MouseDown((x.to_i16().unwrap(), y.to_i16().unwrap())));
}

#[wasm_bindgen]
pub fn mouse_up(x: f64, y: f64) {
  player_dispatch(PlayerVMCommand::MouseUp((x.to_i16().unwrap(), y.to_i16().unwrap())));
}

#[wasm_bindgen]
pub fn mouse_move(x: f64, y: f64) {
  player_dispatch(PlayerVMCommand::MouseMove((x.to_i16().unwrap(), y.to_i16().unwrap())));
}

#[wasm_bindgen]
pub fn key_down(key: String, code: u16) {
  player_dispatch(PlayerVMCommand::KeyDown(key, code));
}

#[wasm_bindgen]
pub fn key_up(key: String, code: u16) {
  player_dispatch(PlayerVMCommand::KeyUp(key, code));
}

#[wasm_bindgen]
pub fn request_datum(datum_ref: DatumRef) {
  player_dispatch(PlayerVMCommand::RequestDatum(datum_ref));
}

#[wasm_bindgen]
pub fn request_script_instance_snapshot(script_instance_ref: u32) {
  player_dispatch(PlayerVMCommand::RequestScriptInstanceSnapshot(script_instance_ref));
}

#[wasm_bindgen]
pub fn subscribe_to_member(cast_lib: i32, cast_member: i32) {
  player_dispatch(PlayerVMCommand::SubscribeToMember(cast_member_ref(cast_lib, cast_member)));
}

#[wasm_bindgen]
pub fn unsubscribe_from_member(cast_lib: i32, cast_member: i32) {
  player_dispatch(PlayerVMCommand::UnsubscribeFromMember(cast_member_ref(cast_lib, cast_member)));
}

#[wasm_bindgen]
pub fn trigger_alert_hook() {
  player_dispatch(PlayerVMCommand::TriggerAlertHook);
}

#[wasm_bindgen]
pub fn subscribe_to_channel_names() {
  spawn_local(async {
    let mut player_mutex = PLAYER_LOCK.lock().await;
    let player = player_mutex.as_mut().unwrap();

    player.is_subscribed_to_channel_names = true;
    for channel in &player.movie.score.channels {
      JsApi::dispatch_channel_name_changed(channel.number as i16);
    }
  });
}

#[wasm_bindgen]
pub fn unsubscribe_from_channel_names() {
  spawn_local(async {
    let mut player_mutex = PLAYER_LOCK.lock().await;
    let player = player_mutex.as_mut().unwrap();

    player.is_subscribed_to_channel_names = false;
  });
}

#[wasm_bindgen(start)]
pub fn main() {
  set_panic_hook();
  init_player();
}
