extern crate cocoa;

use cocoa::base::{NSUInteger, nil, ObjCSelector};
use cocoa::appkit::{NSApp, NSRect, NSPoint, NSSize,
					NSAutoreleasePool, NSProcessInfo,
					NSApplication, NSApplicationActivationPolicyRegular,
					NSWindow, NSTitledWindowMask, NSBackingStoreBuffered,
					NSString,
					NSMenu, NSMenuItem};

fn main() {
	unsafe {
		let pool = NSAutoreleasePool::new(nil);

		let app = NSApp();
		app.setActivationPolicy_(NSApplicationActivationPolicyRegular);

		// create Menu Bar
		let menubar = NSMenu::new(nil);
		let app_menu_item = NSMenuItem::new(nil);
		menubar.addItem_(app_menu_item);
		app.setMainMenu_(menubar);

		// create Application menu
		let app_menu = NSMenu::new(nil);
		let quit_prefix = NSString::alloc(nil).init_str("Quit \0");
		let quit_title = quit_prefix.stringByAppendingString_(
			NSProcessInfo::processInfo(nil).processName()
		);
		let quit_action = "terminate:".as_selector();
		let quit_key = NSString::alloc(nil).init_str("q\0");
		let quit_item = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
			quit_title,
			quit_action,
			quit_key
		);
		app_menu.addItem_(quit_item);
		app_menu_item.setSubmenu_(app_menu);

		// create Window
		let window = NSWindow::alloc(nil).initWithContentRect_styleMask_backing_defer_(
			NSRect::new(NSPoint::new(0., 0.), NSSize::new(200., 200.)),
			NSTitledWindowMask as NSUInteger,
			NSBackingStoreBuffered,
			false
		);
		// this segfaults in invoke_msg_NSPoint_NSPoint():
		// NSWindow::cascadeTopLeftFromPoint_(window, NSPoint::new(20., 20.));
		window.center();
		let title = NSString::alloc(nil).init_str("Hello World!\0");
		window.setTitle_(title);
		window.makeKeyAndOrderFront_(nil);

		app.activateIgnoringOtherApps_(true);
		app.run();
	}
}
