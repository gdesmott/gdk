// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Atom;
#[cfg(any(feature = "v3_22", feature = "dox"))]
use AxisFlags;
use AxisUse;
use Cursor;
use DeviceManager;
#[cfg(any(feature = "v3_22", feature = "dox"))]
use DeviceTool;
use DeviceType;
use Display;
use EventMask;
use GrabOwnership;
use GrabStatus;
use InputMode;
use InputSource;
use ModifierType;
use Screen;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use Seat;
use Window;
use ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Device(Object<ffi::GdkDevice>);

    match fn {
        get_type => || ffi::gdk_device_get_type(),
    }
}

impl Device {
    //pub fn free_history(events: /*Ignored*/&[&TimeCoord]) {
    //    unsafe { TODO: call ffi::gdk_device_free_history() }
    //}

    #[cfg_attr(feature = "v3_16", deprecated)]
    pub fn grab_info_libgtk_only<P: IsA<Device>>(display: &Display, device: &P) -> Option<(Window, bool)> {
        skip_assert_initialized!();
        unsafe {
            let mut grab_window = ptr::null_mut();
            let mut owner_events = mem::uninitialized();
            let ret = from_glib(ffi::gdk_device_grab_info_libgtk_only(display.to_glib_none().0, device.to_glib_none().0, &mut grab_window, &mut owner_events));
            if ret { Some((from_glib_none(grab_window), from_glib(owner_events))) } else { None }
        }
    }
}

pub trait DeviceExt {
    fn get_associated_device(&self) -> Option<Device>;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_axes(&self) -> AxisFlags;

    //fn get_axis(&self, axes: &[f64], use_: AxisUse) -> Option<f64>;

    fn get_axis_use(&self, index_: u32) -> AxisUse;

    //fn get_axis_value(&self, axes: &[f64], axis_label: &Atom) -> Option<f64>;

    fn get_device_type(&self) -> DeviceType;

    fn get_display(&self) -> Display;

    fn get_has_cursor(&self) -> bool;

    //fn get_history(&self, window: &Window, start: u32, stop: u32, events: /*Ignored*/Vec<TimeCoord>) -> Option<i32>;

    fn get_key(&self, index_: u32) -> Option<(u32, ModifierType)>;

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_last_event_window(&self) -> Option<Window>;

    fn get_mode(&self) -> InputMode;

    fn get_n_axes(&self) -> i32;

    fn get_n_keys(&self) -> i32;

    fn get_name(&self) -> Option<String>;

    fn get_position(&self) -> (Screen, i32, i32);

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_position_double(&self) -> (Screen, f64, f64);

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_product_id(&self) -> Option<String>;

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_seat(&self) -> Option<Seat>;

    fn get_source(&self) -> InputSource;

    //fn get_state(&self, window: &Window, axes: &[f64]) -> ModifierType;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_vendor_id(&self) -> Option<String>;

    fn get_window_at_position(&self) -> (Option<Window>, i32, i32);

    fn get_window_at_position_double(&self) -> (Option<Window>, f64, f64);

    #[cfg_attr(feature = "v3_20", deprecated)]
    fn grab<'a, P: Into<Option<&'a Cursor>>>(&self, window: &Window, grab_ownership: GrabOwnership, owner_events: bool, event_mask: EventMask, cursor: P, time_: u32) -> GrabStatus;

    fn list_axes(&self) -> Vec<Atom>;

    fn list_slave_devices(&self) -> Vec<Device>;

    fn set_axis_use(&self, index_: u32, use_: AxisUse);

    fn set_key(&self, index_: u32, keyval: u32, modifiers: ModifierType);

    fn set_mode(&self, mode: InputMode) -> bool;

    #[cfg_attr(feature = "v3_20", deprecated)]
    fn ungrab(&self, time_: u32);

    fn warp(&self, screen: &Screen, x: i32, y: i32);

    fn get_property_device_manager(&self) -> Option<DeviceManager>;

    fn get_property_input_mode(&self) -> InputMode;

    fn set_property_input_mode(&self, input_mode: InputMode);

    fn get_property_input_source(&self) -> InputSource;

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_property_num_touches(&self) -> u32;

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn set_property_seat(&self, seat: Option<&Seat>);

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_property_tool(&self) -> Option<DeviceTool>;

    fn get_property_type(&self) -> DeviceType;

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_tool_changed<F: Fn(&Self, &DeviceTool) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_associated_device_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_property_axes_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_device_manager_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_display_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_has_cursor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_input_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_input_source_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_n_axes_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn connect_property_num_touches_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_product_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn connect_property_seat_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_property_tool_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_vendor_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Device> + IsA<glib::object::Object>> DeviceExt for O {
    fn get_associated_device(&self) -> Option<Device> {
        unsafe {
            from_glib_none(ffi::gdk_device_get_associated_device(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_axes(&self) -> AxisFlags {
        unsafe {
            from_glib(ffi::gdk_device_get_axes(self.to_glib_none().0))
        }
    }

    //fn get_axis(&self, axes: &[f64], use_: AxisUse) -> Option<f64> {
    //    unsafe { TODO: call ffi::gdk_device_get_axis() }
    //}

    fn get_axis_use(&self, index_: u32) -> AxisUse {
        unsafe {
            from_glib(ffi::gdk_device_get_axis_use(self.to_glib_none().0, index_))
        }
    }

    //fn get_axis_value(&self, axes: &[f64], axis_label: &Atom) -> Option<f64> {
    //    unsafe { TODO: call ffi::gdk_device_get_axis_value() }
    //}

    fn get_device_type(&self) -> DeviceType {
        unsafe {
            from_glib(ffi::gdk_device_get_device_type(self.to_glib_none().0))
        }
    }

    fn get_display(&self) -> Display {
        unsafe {
            from_glib_none(ffi::gdk_device_get_display(self.to_glib_none().0))
        }
    }

    fn get_has_cursor(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_device_get_has_cursor(self.to_glib_none().0))
        }
    }

    //fn get_history(&self, window: &Window, start: u32, stop: u32, events: /*Ignored*/Vec<TimeCoord>) -> Option<i32> {
    //    unsafe { TODO: call ffi::gdk_device_get_history() }
    //}

    fn get_key(&self, index_: u32) -> Option<(u32, ModifierType)> {
        unsafe {
            let mut keyval = mem::uninitialized();
            let mut modifiers = mem::uninitialized();
            let ret = from_glib(ffi::gdk_device_get_key(self.to_glib_none().0, index_, &mut keyval, &mut modifiers));
            if ret { Some((keyval, from_glib(modifiers))) } else { None }
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_last_event_window(&self) -> Option<Window> {
        unsafe {
            from_glib_none(ffi::gdk_device_get_last_event_window(self.to_glib_none().0))
        }
    }

    fn get_mode(&self) -> InputMode {
        unsafe {
            from_glib(ffi::gdk_device_get_mode(self.to_glib_none().0))
        }
    }

    fn get_n_axes(&self) -> i32 {
        unsafe {
            ffi::gdk_device_get_n_axes(self.to_glib_none().0)
        }
    }

    fn get_n_keys(&self) -> i32 {
        unsafe {
            ffi::gdk_device_get_n_keys(self.to_glib_none().0)
        }
    }

    fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gdk_device_get_name(self.to_glib_none().0))
        }
    }

    fn get_position(&self) -> (Screen, i32, i32) {
        unsafe {
            let mut screen = ptr::null_mut();
            let mut x = mem::uninitialized();
            let mut y = mem::uninitialized();
            ffi::gdk_device_get_position(self.to_glib_none().0, &mut screen, &mut x, &mut y);
            (from_glib_none(screen), x, y)
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_position_double(&self) -> (Screen, f64, f64) {
        unsafe {
            let mut screen = ptr::null_mut();
            let mut x = mem::uninitialized();
            let mut y = mem::uninitialized();
            ffi::gdk_device_get_position_double(self.to_glib_none().0, &mut screen, &mut x, &mut y);
            (from_glib_none(screen), x, y)
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_product_id(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gdk_device_get_product_id(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_seat(&self) -> Option<Seat> {
        unsafe {
            from_glib_none(ffi::gdk_device_get_seat(self.to_glib_none().0))
        }
    }

    fn get_source(&self) -> InputSource {
        unsafe {
            from_glib(ffi::gdk_device_get_source(self.to_glib_none().0))
        }
    }

    //fn get_state(&self, window: &Window, axes: &[f64]) -> ModifierType {
    //    unsafe { TODO: call ffi::gdk_device_get_state() }
    //}

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_vendor_id(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gdk_device_get_vendor_id(self.to_glib_none().0))
        }
    }

    fn get_window_at_position(&self) -> (Option<Window>, i32, i32) {
        unsafe {
            let mut win_x = mem::uninitialized();
            let mut win_y = mem::uninitialized();
            let ret = from_glib_none(ffi::gdk_device_get_window_at_position(self.to_glib_none().0, &mut win_x, &mut win_y));
            (ret, win_x, win_y)
        }
    }

    fn get_window_at_position_double(&self) -> (Option<Window>, f64, f64) {
        unsafe {
            let mut win_x = mem::uninitialized();
            let mut win_y = mem::uninitialized();
            let ret = from_glib_none(ffi::gdk_device_get_window_at_position_double(self.to_glib_none().0, &mut win_x, &mut win_y));
            (ret, win_x, win_y)
        }
    }

    fn grab<'a, P: Into<Option<&'a Cursor>>>(&self, window: &Window, grab_ownership: GrabOwnership, owner_events: bool, event_mask: EventMask, cursor: P, time_: u32) -> GrabStatus {
        let cursor = cursor.into();
        let cursor = cursor.to_glib_none();
        unsafe {
            from_glib(ffi::gdk_device_grab(self.to_glib_none().0, window.to_glib_none().0, grab_ownership.to_glib(), owner_events.to_glib(), event_mask.to_glib(), cursor.0, time_))
        }
    }

    fn list_axes(&self) -> Vec<Atom> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gdk_device_list_axes(self.to_glib_none().0))
        }
    }

    fn list_slave_devices(&self) -> Vec<Device> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gdk_device_list_slave_devices(self.to_glib_none().0))
        }
    }

    fn set_axis_use(&self, index_: u32, use_: AxisUse) {
        unsafe {
            ffi::gdk_device_set_axis_use(self.to_glib_none().0, index_, use_.to_glib());
        }
    }

    fn set_key(&self, index_: u32, keyval: u32, modifiers: ModifierType) {
        unsafe {
            ffi::gdk_device_set_key(self.to_glib_none().0, index_, keyval, modifiers.to_glib());
        }
    }

    fn set_mode(&self, mode: InputMode) -> bool {
        unsafe {
            from_glib(ffi::gdk_device_set_mode(self.to_glib_none().0, mode.to_glib()))
        }
    }

    fn ungrab(&self, time_: u32) {
        unsafe {
            ffi::gdk_device_ungrab(self.to_glib_none().0, time_);
        }
    }

    fn warp(&self, screen: &Screen, x: i32, y: i32) {
        unsafe {
            ffi::gdk_device_warp(self.to_glib_none().0, screen.to_glib_none().0, x, y);
        }
    }

    fn get_property_device_manager(&self) -> Option<DeviceManager> {
        unsafe {
            let mut value = Value::from_type(<DeviceManager as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "device-manager".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn get_property_input_mode(&self) -> InputMode {
        unsafe {
            let mut value = Value::from_type(<InputMode as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "input-mode".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_input_mode(&self, input_mode: InputMode) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "input-mode".to_glib_none().0, Value::from(&input_mode).to_glib_none().0);
        }
    }

    fn get_property_input_source(&self) -> InputSource {
        unsafe {
            let mut value = Value::from_type(<InputSource as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "input-source".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_property_num_touches(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "num-touches".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn set_property_seat(&self, seat: Option<&Seat>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "seat".to_glib_none().0, Value::from(seat).to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_property_tool(&self) -> Option<DeviceTool> {
        unsafe {
            let mut value = Value::from_type(<DeviceTool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "tool".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn get_property_type(&self) -> DeviceType {
        unsafe {
            let mut value = Value::from_type(<DeviceType as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "type".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "changed",
                transmute(changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_tool_changed<F: Fn(&Self, &DeviceTool) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &DeviceTool) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "tool-changed",
                transmute(tool_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_associated_device_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::associated-device",
                transmute(notify_associated_device_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_property_axes_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::axes",
                transmute(notify_axes_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_device_manager_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::device-manager",
                transmute(notify_device_manager_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_display_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::display",
                transmute(notify_display_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_has_cursor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::has-cursor",
                transmute(notify_has_cursor_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_input_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::input-mode",
                transmute(notify_input_mode_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_input_source_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::input-source",
                transmute(notify_input_source_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_n_axes_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::n-axes",
                transmute(notify_n_axes_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::name",
                transmute(notify_name_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn connect_property_num_touches_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::num-touches",
                transmute(notify_num_touches_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_product_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::product-id",
                transmute(notify_product_id_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn connect_property_seat_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::seat",
                transmute(notify_seat_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_property_tool_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::tool",
                transmute(notify_tool_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::type",
                transmute(notify_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_vendor_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::vendor-id",
                transmute(notify_vendor_id_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn changed_trampoline<P>(this: *mut ffi::GdkDevice, f: glib_ffi::gpointer)
where P: IsA<Device> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Device::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_22", feature = "dox"))]
unsafe extern "C" fn tool_changed_trampoline<P>(this: *mut ffi::GdkDevice, tool: *mut ffi::GdkDeviceTool, f: glib_ffi::gpointer)
where P: IsA<Device> {
    let f: &&(Fn(&P, &DeviceTool) + 'static) = transmute(f);
    f(&Device::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(tool))
}

unsafe extern "C" fn notify_associated_device_trampoline<P>(this: *mut ffi::GdkDevice, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Device> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Device::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_22", feature = "dox"))]
unsafe extern "C" fn notify_axes_trampoline<P>(this: *mut ffi::GdkDevice, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Device> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Device::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_device_manager_trampoline<P>(this: *mut ffi::GdkDevice, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Device> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Device::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_display_trampoline<P>(this: *mut ffi::GdkDevice, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Device> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Device::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_has_cursor_trampoline<P>(this: *mut ffi::GdkDevice, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Device> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Device::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_input_mode_trampoline<P>(this: *mut ffi::GdkDevice, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Device> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Device::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_input_source_trampoline<P>(this: *mut ffi::GdkDevice, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Device> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Device::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_n_axes_trampoline<P>(this: *mut ffi::GdkDevice, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Device> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Device::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_name_trampoline<P>(this: *mut ffi::GdkDevice, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Device> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Device::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_20", feature = "dox"))]
unsafe extern "C" fn notify_num_touches_trampoline<P>(this: *mut ffi::GdkDevice, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Device> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Device::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
unsafe extern "C" fn notify_product_id_trampoline<P>(this: *mut ffi::GdkDevice, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Device> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Device::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_20", feature = "dox"))]
unsafe extern "C" fn notify_seat_trampoline<P>(this: *mut ffi::GdkDevice, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Device> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Device::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_22", feature = "dox"))]
unsafe extern "C" fn notify_tool_trampoline<P>(this: *mut ffi::GdkDevice, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Device> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Device::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_type_trampoline<P>(this: *mut ffi::GdkDevice, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Device> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Device::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
unsafe extern "C" fn notify_vendor_id_trampoline<P>(this: *mut ffi::GdkDevice, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Device> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Device::from_glib_borrow(this).downcast_unchecked())
}
