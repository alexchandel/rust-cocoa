// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use base::{ObjCMethodCall, id};

pub type CGFloat = f32;

#[allow(dead_code)]
pub struct NSPoint {
    x: f64,
    y: f64,
}

impl NSPoint {
    pub fn new(x: f64, y: f64) -> NSPoint {
        NSPoint {
            x: x,
            y: y,
        }
    }
}

#[allow(dead_code)]
pub struct NSSize {
    width: f64,
    height: f64,
}

impl NSSize {
    pub fn new(width: f64, height: f64) -> NSSize {
        NSSize {
            width: width,
            height: height,
        }
    }
}

pub struct NSRect {
    pub origin: NSPoint,
    pub size: NSSize,
}

#[link(name = "AppKit", kind = "framework")]
extern {}

pub unsafe fn NSApp() -> id {
    "NSApplication".send("sharedApplication", ())
}

