#![no_std]
#![no_main]

use kartoffel::*;

#[no_mangle]
fn main() {
    loop {
        radar_wait();

        let scan = radar_scan_3x3();

        let front = scan.at(0, -1);
        let left = scan.at(-1, 0);
        let right = scan.at(-1, 0);
        let back = scan.at(-1, 0);

        match (front, left, right, back) {
            ('@', _, _, _) => {
                arm_wait();
                arm_stab();
            }
            (_, '@', _, _) => {
                motor_wait();
                motor_turn_left();
            }
            (_, _, '@', _) => {
                motor_wait();
                motor_turn_right();
            }
            (_, _, _, '@') => {
                motor_wait();
                motor_turn_left();
                motor_wait();
                motor_turn_left();
            }
            ('.', _, _, _) => {
                motor_wait();
                motor_step_fw();
            }

            (_, '.', _, _) => {
                motor_wait();
                motor_turn_left();
            }
            (_, _, '.', _) => {
                motor_wait();
                motor_turn_right();
            }
            (_, _, _, '.') => {
                motor_wait();
                motor_step_bw();
            }
            _ => {}
        }
    }
}
